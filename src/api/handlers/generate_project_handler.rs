use std::{collections::HashMap, sync::Arc};

use poem_openapi::{
    payload::{Json, PlainText},
    types::Example,
    ApiResponse, Object,
};
use scc::HashMap as ConcurrentHashMap;
use serde::{Deserialize, Serialize};
use tmpdir::TmpDir;
use uuid::Uuid;

use crate::services::{
    code_generator::{types::TemplateType, CodeGenerator, CodeGeneratorServiceError},
    traits::{object_store::ObjectStoreService, version_control::VersionControlService},
};

#[derive(Object, Clone, Serialize, Deserialize)]
pub struct ParameterConfiguration {
    /// The multiplier of the parameter
    pub multiplier: Option<i64>,
    /// The unit of the parameter
    pub unit: Option<String>,
}

impl Example for ParameterConfiguration {
    fn example() -> Self {
        Self {
            multiplier: Some(123),
            unit: Some("Some unit from provided list".to_string()),
        }
    }
}

#[derive(Object, Deserialize, Clone)]
pub struct GitHubCredentials {
    pub username: String,
    pub token: String,
    pub email: String,
}
#[derive(Object, Deserialize)]
pub struct NewProject {
    /// The name of the project
    name: String,
    /// The list of pallets to include in the project, where the key is the
    /// pallet name and the value is a optional map of configuration parameters
    pallets: HashMap<String, Option<HashMap<String, ParameterConfiguration>>>,
    /// The template type for the project
    template: TemplateType,
    github: Option<GitHubCredentials>,
}

impl Example for NewProject {
    fn example() -> Self {
        let mut pallets = HashMap::new();
        let mut pallet_config = HashMap::new();
        pallet_config.insert("Some Config".to_string(), ParameterConfiguration::example());
        pallets.insert("Some Pallet".to_string(), Some(pallet_config));
        Self {
            name: "project_name".to_string(),
            template: TemplateType::SoloChain,
            pallets,
            github: None,
        }
    }
}

#[derive(ApiResponse)]
pub enum GenerateProjectResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<Uuid>),
    #[oai(status = 404)]
    PalletNotFound(PlainText<String>),
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

pub async fn generate_a_project_handler(
    task_status_map: Arc<
        ConcurrentHashMap<Uuid, Option<Result<String, CodeGeneratorServiceError>>>,
    >,
    object_store_service: Arc<dyn ObjectStoreService>,
    code_generator_service: Arc<dyn CodeGenerator>,
    version_control_service: Arc<dyn VersionControlService>,
    project: Json<NewProject>,
) -> GenerateProjectResponse {
    let mut project_name = project.name.clone();
    // Append uuid to project name
    project_name = format!("{}_{}", project_name, Uuid::new_v4());

    let archive = match code_generator_service
        .generate_project_archive(&project.pallets, &project.template)
        .await
    {
        Ok(archive) => archive,
        Err(CodeGeneratorServiceError::PalletNotFoundError(pallet_name)) => {
            return GenerateProjectResponse::PalletNotFound(PlainText(format!(
                "Pallet not found: {}",
                pallet_name
            )))
        }
        Err(e) => {
            return GenerateProjectResponse::InternalServerError(PlainText(format!(
                "Internal Server Error: {}",
                e
            )))
        }
    };
    let status_id = Uuid::new_v4();
    // TODO: hadnle result
    let _ = task_status_map.insert_async(status_id, None).await;
    match project.github.clone() {
        None => {
            tokio::spawn(async move {
                let object_name = format!("{}.zip", &project_name);
                if let Err(e) = object_store_service
                    .upload_content(archive, object_name.as_str())
                    .await
                    .map_err(CodeGeneratorServiceError::OtherError)
                {
                    let _ = task_status_map
                        .update_async(&status_id, |_, v| {
                            *v = Some(Err(e));
                            v.clone()
                        })
                        .await;
                    return;
                }
                let result = object_store_service
                    .get_presigned_url(object_name.as_str(), 3600)
                    .await
                    .map_err(CodeGeneratorServiceError::OtherError);
                let _ = task_status_map
                    .update_async(&status_id, |_, v| {
                        *v = Some(result);
                        v.clone()
                    })
                    .await;
            });
        }
        Some(gh) => {
            let gh = gh.clone();
            tokio::spawn(async move {
                // create tmpdir
                log::info!("GitHub project upload started...");
                let tmpdir = match TmpDir::new(&project_name).await {
                    Ok(t) => t,
                    Err(e) => {
                        log::error!("TmpDir Error");
                        let _ = task_status_map
                            .update_async(&status_id, |_, v| {
                                *v = Some(Err(CodeGeneratorServiceError::OtherError(format!(
                                    "TmpDirError: {}",
                                    e
                                ))));
                                v.clone()
                            })
                            .await;
                        return;
                    }
                };
                log::info!("TmpDir Created...");
                if let Err(e) = code_generator_service
                    .unpack_archive_to_folder(archive, tmpdir.as_ref())
                    .await
                {
                    log::error!("Unpack to tmpdir failed...");
                    let _ = task_status_map
                        .update_async(&status_id, |_, v| {
                            *v = Some(Err(CodeGeneratorServiceError::OtherError(format!(
                                "TmpDirError: {}",
                                e
                            ))));
                            v.clone()
                        })
                        .await;
                    return;
                };
                log::info!("Archive Unpacked...");
                if let Err(e) = version_control_service
                    .create_remote_repo(&gh.username, &gh.token, project_name.as_ref())
                    .await
                {
                    log::error!("Create remote repo failed");
                    let _ = task_status_map
                        .update_async(&status_id, |_, v| {
                            *v = Some(Err(CodeGeneratorServiceError::OtherError(format!(
                                "GitHub create repo failed: {}",
                                e
                            ))));
                            v.clone()
                        })
                        .await;
                    return;
                };
                log::info!("Github Repo Created...");
                if let Err(e) = version_control_service
                    .push_folder_to_repo(
                        tmpdir.as_ref(),
                        project_name.as_ref(),
                        &gh.username,
                        &gh.token,
                        &gh.email,
                    )
                    .await
                {
                    log::error!("push folder to repo failed");
                    let _ = task_status_map
                        .update_async(&status_id, |_, v| {
                            *v = Some(Err(CodeGeneratorServiceError::OtherError(format!(
                                "GitHub push repo failed: {}",
                                e
                            ))));
                            v.clone()
                        })
                        .await;
                    return;
                }
                log::info!("Github Repo pushed...");
                let _ = task_status_map
                    .update_async(&status_id, |_, v| {
                        *v = Some(Ok(format!(
                            "https://github.com/{}/{}",
                            &gh.username, &project_name
                        )));
                        v.clone()
                    })
                    .await;
            });
        }
    }
    GenerateProjectResponse::Ok(Json(status_id))
}
