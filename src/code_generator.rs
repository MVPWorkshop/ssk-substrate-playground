use crate::utils::file_manager::{copy_dir_recursive, create_new_folder};
use crate::utils::manifest::generate_manifest_file;
use crate::utils::runtime_lib::generate_runtime_lib_file;

use super::types::PalletConfig;

use log::{error, info};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

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
        .map_err(|_| PalletConfigLoadError {
            message: "read dir error.".to_string(),
        })?;

    let mut dir_entries = vec![];
    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|_| PalletConfigLoadError {
            message: "Failed to read directory entry.".to_string(),
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
) -> Result<(), PalletConfigLoadError> {
    // Create a new project directory and copy the template.
    create_new_project_async(project_name).await;

    println!("Created project: {}", project_name);

    // Add the pallets to the new project.
    let x = project_name.clone();
    tokio::task::spawn_blocking(move || add_pallets(&x, pallets))
        .await
        .map_err(|e| PalletConfigLoadError {
            message: format!("Error adding pallets: {:?}", e),
        })?;
    println!("Added pallets to the project: {}", project_name);
    Ok(())
}
