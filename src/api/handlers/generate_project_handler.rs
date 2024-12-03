use std::{collections::HashMap, sync::Arc};

use poem_openapi::{
    payload::{Json, PlainText},
    types::Example,
    ApiResponse, Object,
};
use scc::HashMap as ConcurrentHashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::{
    code_generator::{types::TemplateType, CodeGenerator, CodeGeneratorServiceError},
    object_store::ObjectStoreService,
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
#[derive(Object, Deserialize)]
pub struct NewProject {
    /// The name of the project
    name: String,
    /// The list of pallets to include in the project, where the key is the
    /// pallet name and the value is a optional map of configuration parameters
    pallets: HashMap<String, Option<HashMap<String, ParameterConfiguration>>>,
     /// The template type for the project
     template: TemplateType,
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
    project: Json<NewProject>,
) -> GenerateProjectResponse {
    let mut project_name = project.name.clone();
    // Append uuid to project name
    project_name = format!("{}_{}", project_name, Uuid::new_v4());

    let template_type = project.template.clone();

    let archive = match code_generator_service
        .generate_project_archive(
            &project.pallets,
            template_type
        )
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
    tokio::spawn(async move {
        let object_name = format!("{}.zip", project_name);
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
    GenerateProjectResponse::Ok(Json(status_id))
}
