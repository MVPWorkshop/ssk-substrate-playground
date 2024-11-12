use crate::utils::manifest::generate_manifest_file;
use crate::utils::runtime_lib::generate_runtime_lib_file;

use super::types::PalletConfig;
use super::utils::file_manager::{copy_dir_recursive, create_new_folder};

use log::{error, info};
use std::fmt;
use std::path::Path;

/// Creates a new project directory and copies a basic template into it.
///
/// # Arguments
///
/// * `project_name` - The name of the project to be created.
pub fn create_new_project(project_name: &String) {
    // Base path for generated projects.
    let base_path = Path::new("generated_code");

    // Create a new folder for the project.
    if let Err(e) = create_new_folder(base_path, project_name) {
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
    if let Err(e) = copy_dir_recursive(src, dest) {
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

/// Reads all `.toml` files from the specified directory and parses them into `PalletConfig` objects.
///
/// # Arguments
///
/// * `dir` - A `String` representing the path to the directory containing the `.toml` files.
///
/// # Returns
///
/// * `Result<Vec<PalletConfig>, PalletConfigLoadError>` - A result containing a vector of `PalletConfig` objects
///   if successful, or a `PalletConfigLoadError` if an error occurs.
///
/// # Errors
///
/// This function will return a `PalletConfigLoadError` if:
/// - The directory cannot be read.
/// - Any directory entry cannot be read.
/// - Any `.toml` file cannot be read.
/// - Any `.toml` file cannot be parsed into a `PalletConfig`.
///
/// # Example
///
/// ```rust
/// use substrate_runtime_builder::code_generator::get_all_pallet_configs_from_dir;
/// let configs = get_all_pallet_configs_from_dir("path/to/dir");
/// match configs {
///     Ok(configs) => println!("Successfully loaded configs: {:?}", configs),
///     Err(e) => eprintln!("Error loading configs: {}", e),
/// }
/// ```
pub fn get_all_pallet_configs_from_dir(
    dir: &str,
) -> Result<Vec<PalletConfig>, PalletConfigLoadError> {
    // Read directory entries
    let dir_entries = std::fs::read_dir(dir)
        .map_err(|_| PalletConfigLoadError {
            message: "read dir error.".to_string(),
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| PalletConfigLoadError {
            message: "Failed to read directory entry.".to_string(),
        })?;

    // Filter and read .toml files into strings
    let toml_strings_no_updated = dir_entries
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
        // Read .toml files into strings
        .map(|path| {
            std::fs::read_to_string(&path).map_err(|_| PalletConfigLoadError {
                message: format!("Failed to read file: {:?}", path),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Parse TOML strings into PalletConfigNoUpdated
    let pallet_configs = toml_strings_no_updated
        .into_iter()
        .map(|x| {
            toml::from_str(x.as_str()).map_err(|_| PalletConfigLoadError {
                message: "cenvert to toml error.".to_string(),
            })
        })
        .collect::<Result<Vec<PalletConfig>, PalletConfigLoadError>>()?;
    Ok(pallet_configs)
}

pub fn generate_project(
    project_name: &String,
    pallets: Vec<PalletConfig>,
) -> Result<(), PalletConfigLoadError> {
    // Create a new project directory and copy the template.
    create_new_project(project_name);

    println!("Created project: {}", project_name);

    // Add the pallets to the new project.
    add_pallets(project_name, pallets);
    println!("Added pallets to the project: {}", project_name);
    Ok(())
}
