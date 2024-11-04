use super::types::PalletConfig;
use super::utils::file_manager::{
    copy_dir_recursive, create_new_folder, read_file_to_string, replace_file_content,
};
use super::utils::manifest::ManifestPalletConfig;
use super::utils::manifest::SubstrateManifestUtil;
use super::utils::runtime::SubstrateRuntimeUtil;

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
    for pallet_config in pallet_configs {
        // Project directory path.
        let project_directory = format!("generated_code/{}", project_name);

        // File paths for runtime, chain spec, and manifest.
        let runtime_file_path = project_directory.clone() + "/runtime/src/lib.rs";
        let chain_spec_file_path = project_directory.clone() + "/node/src/chain_spec.rs";
        let manifest_path = project_directory + "/runtime/Cargo.toml";

        // Create manifest configuration for the pallet.
        let pallet_manifest_config = ManifestPalletConfig {
            name: pallet_config.name.to_string(),
            dependencies: pallet_config.dependencies.clone(),
        };

        // Read and update the manifest file.
        let content = match read_file_to_string(&manifest_path) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Failed to read the manifest file '{}': {}",
                    manifest_path, e
                );
                continue;
            }
        };

        let mut util = SubstrateManifestUtil::new(pallet_manifest_config, content);
        let updated_manifest = util.generate_code();
        if let Err(e) = replace_file_content(Path::new(&manifest_path), &updated_manifest) {
            error!(
                "Failed to replace manifest content in '{}': {}",
                manifest_path, e
            );
            continue;
        }
        info!("Manifest file '{}' updated successfully", manifest_path);

        // Read runtime and chain spec files.
        let runtime_string = match read_file_to_string(&runtime_file_path) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Failed to read the runtime file '{}': {}",
                    runtime_file_path, e
                );
                continue;
            }
        };

        let chain_spec_string = match read_file_to_string(&chain_spec_file_path) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Failed to read the chain spec file '{}': {}",
                    chain_spec_file_path, e
                );
                continue;
            }
        };

        // Generate new runtime code with the added pallet.
        let mut pallet_config =
            SubstrateRuntimeUtil::new(pallet_config, runtime_string, chain_spec_string);
        let updated_code = pallet_config.generate_runtime_code();

        // Replace runtime code with the new generated code.
        let runtime_path = Path::new(&runtime_file_path);
        if let Err(e) = replace_file_content(runtime_path, &updated_code.updated_runtime_code) {
            error!(
                "Failed to replace runtime content in '{}': {}",
                runtime_file_path, e
            );
            continue;
        }
        info!("Runtime file '{}' updated successfully", runtime_file_path);

        // Replace chain spec code with the new generated code.
        let chain_spec_path = Path::new(&chain_spec_file_path);
        if let Err(e) = replace_file_content(chain_spec_path, &updated_code.updated_chain_spec_code)
        {
            error!(
                "Failed to replace chain spec content in '{}': {}",
                chain_spec_file_path, e
            );
            continue;
        }
        info!(
            "Chain spec file '{}' updated successfully",
            chain_spec_file_path
        );
    }
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
