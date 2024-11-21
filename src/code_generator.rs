use crate::services::template_archiver::{TemplateArchiverError, TemplateArchiverService};
use crate::utils::file_manager::{copy_dir_recursive, create_new_folder};
use crate::utils::manifest::{generate_manifest_file, generate_manifest_file_to_bytes};
use crate::utils::runtime_lib::{generate_runtime_lib_file, generate_runtime_lib_file_bytes};

use super::types::PalletConfig;

use log::{error, info};
use scc::HashMap as ConcurrentHashMap;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_new_project_async(project_name: &String) {
    // Base path for generated projects.
    let base_path = Path::new("generated_code");

    // Create a new folder for the project.
    if let Err(e) = create_new_folder(base_path, project_name).await {
        error!("Failed to create project folder '{}': {}", project_name, e);
        return;
    }
    info!("Created new project folder '{}'", project_name);

    // Source path for the template to be copied.
    let src = Path::new("templates/solochain/basic");
    // Destination path for the new project.
    let path = format!("generated_code/{}", project_name);
    let dest = Path::new(&path);

    // Copy the basic template to the new project folder.
    if let Err(e) = copy_dir_recursive(src, dest).await {
        error!(
            "Failed to copy template for project '{}': {}",
            project_name, e
        );
        return;
    }
    info!("Project '{}' created successfully", project_name);
}

/// Adds specified pallets to the project by modifying relevant files.
///
/// # Arguments
///
/// * `project_name` - The name of the project where pallets are to be added.
/// * `pallet_configs` - A list of configurations for the pallets to be added.
pub fn add_pallets(project_name: &String, pallet_configs: Vec<PalletConfig>) {
    generate_manifest_file(
        format!("generated_code/{}/runtime/Cargo.toml.hbs", project_name).as_str(),
        &pallet_configs,
    )
    .unwrap();
    generate_runtime_lib_file(
        format!("generated_code/{}/runtime/src/lib.rs.hbs", project_name).as_str(),
        &pallet_configs,
    )
    .unwrap();
}

pub async fn add_pallets_to_archive<ZB, TAS>(
    template_archive_service: &TAS,
    zipper_buffer: ZB,
    pallet_configs: Vec<PalletConfig>,
) -> Result<ZB, TemplateArchiverError>
where
    TAS: TemplateArchiverService<ZippedBuffer = ZB>,
{
    let manifest_file_path = "templates/solochain/basic/runtime/Cargo.toml.hbs";
    let manifest_file_content =
        generate_manifest_file_to_bytes(manifest_file_path, &pallet_configs).unwrap();
    let zipper_buffer = template_archive_service
        .add_content_to_archive(
            zipper_buffer,
            &manifest_file_content,
            Path::new("runtime/Cargo.toml"),
        )
        .await?;

    let runtime_lib_file_path = "templates/solochain/basic/runtime/src/lib.rs.hbs";
    let runtime_lib_file_content =
        generate_runtime_lib_file_bytes(runtime_lib_file_path, &pallet_configs).unwrap();
    let zipper_buffer = template_archive_service
        .add_content_to_archive(
            zipper_buffer,
            &runtime_lib_file_content,
            Path::new("runtime/src/lib.rs"),
        )
        .await?;

    Ok(zipper_buffer)
}

// TODO: Make proper Errors, with thiserror
#[derive(Debug, Clone)]
pub struct PalletConfigLoadError {
    pub message: String,
}

impl fmt::Display for PalletConfigLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub async fn get_all_pallet_configs_from_dir(
    dir: &str,
) -> Result<HashMap<String, PalletConfig>, PalletConfigLoadError> {
    // Read directory entries
    let mut read_dir = tokio::fs::read_dir(dir)
        .await
        .map_err(|e| PalletConfigLoadError {
            message: format!("Failed to read directory: {:?}", e),
        })?;

    let mut dir_entries = vec![];
    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| PalletConfigLoadError {
            message: format!("Failed to read directory: {:?}", e),
        })?
    {
        dir_entries.push(entry);
    }
    // Filter and read .toml files into strings
    let file_paths = dir_entries
        .into_iter()
        // Filter out non-.toml files
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut toml_strings_no_updated = vec![];
    for file_path in file_paths {
        let string =
            tokio::fs::read_to_string(&file_path)
                .await
                .map_err(|_| PalletConfigLoadError {
                    message: format!("Failed to read file: {:?}", file_path),
                })?;
        toml_strings_no_updated.push(string);
    }
    let pallet_configs = toml_strings_no_updated
        .into_iter()
        .map(|x| match toml::from_str::<PalletConfig>(x.as_str()) {
            Ok(pallet_config) => Ok((pallet_config.name.clone(), pallet_config)),
            Err(e) => Err(PalletConfigLoadError {
                message: format!("Failed to parse toml: {:?}", e),
            }),
        })
        .collect::<Result<HashMap<String, PalletConfig>, PalletConfigLoadError>>()?;
    Ok(pallet_configs)
}

pub async fn generate_project(
    project_name: &String,
    pallets: Vec<PalletConfig>,
    task_id: Uuid,
    task_status_map: Arc<ConcurrentHashMap<Uuid, Option<Result<(), PalletConfigLoadError>>>>,
) {
    // Create a new project directory and copy the template.
    create_new_project_async(project_name).await;
    println!("Created project: {}", project_name);

    // Add the pallets to the new project.
    let x = project_name.clone();
    let result = tokio::task::spawn_blocking(move || add_pallets(&x, pallets))
        .await
        .map_err(|e| PalletConfigLoadError {
            message: format!("Error adding pallets: {:?}", e),
        });
    let _ = task_status_map
        .update_async(&task_id, |_, v| {
            *v = Some(result);
            v.clone()
        })
        .await;
    println!("Added pallets to the project: {}", project_name);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        services::template_archiver::async_zip::AsyncZipTemplateArchiverService, CONFIG_DIR,
    };

    #[tokio::test]
    #[ignore]
    async fn test_add_archived_pallets() {
        let pallets = get_all_pallet_configs_from_dir(CONFIG_DIR).await.unwrap();
        let pallets = pallets
            .into_iter()
            .map(|(_, config)| config)
            .collect::<Vec<_>>();
        let archiver = AsyncZipTemplateArchiverService;
        let zipper_buffer = archiver
            .archive_template(Path::new("templates/solochain/basic"), "hbs")
            .await;
        let zipper_buffer =
            add_pallets_to_archive(&archiver, zipper_buffer.unwrap(), pallets).await;
        // save zipper_buffer to file test.zip in root directory
        let bytes = archiver
            .close_archive(zipper_buffer.unwrap())
            .await
            .unwrap();
        tokio::fs::write("test.zip", bytes).await.unwrap();
    }
}
