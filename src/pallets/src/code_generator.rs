use super::configs::pallet_utility::PalletUtilityConfig;
use super::types::PalletConfig;
use super::utils::file_manager::{
    copy_dir_recursive, create_new_folder, read_file_to_string, replace_file_content,
};
use super::utils::manifest::ManifestPalletConfig;
use super::utils::manifest::SubstrateManifestUtil;
use super::utils::runtime::SubstrateRuntimeUtil;
use crate::types::ESupportedPallets;
use std::path::Path;

/// Creates a new project directory and copies a basic template into it.
///
/// # Arguments
///
/// * `project_name` - The name of the project to be created.
pub fn create_new_project(project_name: String) {
    // Base path for generated projects.
    let base_path = Path::new("../../generated_code");

    // Create a new folder for the project.
    match create_new_folder(base_path, &project_name) {
        Ok(_) => println!("Created new project folder!"),
        Err(e) => eprintln!("Error creating folder: {}", e),
    }

    // Source path for the template to be copied.
    let src = Path::new("../../templates/solochain/basic");
    // Destination path for the new project.
    let path = format!("../../generated_code/{}", project_name);
    let dest = Path::new(&path);

    // Copy the basic template to the new project folder.
    match copy_dir_recursive(src, dest) {
        Ok(_) => println!("Project created successfully!"),
        Err(e) => eprintln!("Error in creating the project: {}", e),
    }
}

/// Adds specified pallets to the project by modifying relevant files.
///
/// # Arguments
///
/// * `project_name` - The name of the project where pallets are to be added.
/// * `pallet_configs` - A list of configurations for the pallets to be added.
pub fn add_pallets(project_name: String, pallet_configs: Vec<PalletConfig>) {
    for pallet_config in pallet_configs {
        // Project directory path.
        let project_directory = format!("../../generated_code/{}", project_name);

        // File paths for runtime, chain spec, and manifest.
        let runtime_file_path = project_directory.clone() + "/runtime/src/lib.rs";
        let chain_spec_file_path = project_directory.clone() + "/node/src/chain_spec.rs";
        let manifest_path = project_directory + "/runtime/cargo.toml";

        // Create manifest configuration for the pallet.
        let pallet_manifest_config = ManifestPalletConfig {
            name: pallet_config.name.to_string(),
            dependencies: pallet_config.dependencies.clone(),
        };

        // Read and update the manifest file.
        match read_file_to_string(&manifest_path) {
            Ok(content) => {
                let mut util = SubstrateManifestUtil::new(pallet_manifest_config, content);
                let updated_manifest = util.generate_code();
                match replace_file_content(Path::new(&manifest_path), &updated_manifest) {
                    Ok(_) => println!("Manifest updated successfully."),
                    Err(e) => eprintln!("Error replacing manifest content: {}", e),
                }
            }
            Err(e) => {
                println!("Failed to read the manifest file: {}", e);
            }
        }

        // Read runtime and chain spec files.
        let runtime_string = read_file_to_string(&runtime_file_path).unwrap();
        let chain_spec_string = read_file_to_string(&chain_spec_file_path).unwrap();

        // Generate new runtime code with the added pallet.
        let mut pallet_config =
            SubstrateRuntimeUtil::new(pallet_config, runtime_string, chain_spec_string);
        let updated_code = pallet_config.generate_runtime_code();

        // Replace runtime code with the new generated code.
        let runtime_path = Path::new(&runtime_file_path);
        match replace_file_content(runtime_path, &updated_code.updated_runtime_code) {
            Ok(_) => println!("Runtime updated successfully."),
            Err(e) => eprintln!("Error replacing runtime content: {}", e),
        }

        // Replace chain spec code with the new generated code.
        let chain_spec_path = Path::new(&chain_spec_file_path);
        match replace_file_content(chain_spec_path, &updated_code.updated_chain_spec_code) {
            Ok(_) => println!("Chain spec updated successfully."),
            Err(e) => eprintln!("Error replacing chain spec content: {}", e),
        }
    }
}

/// Generates configurations for the supported pallets.
///
/// # Arguments
///
/// * `pallets` - A list of supported pallets to configure.
///
/// # Returns
///
/// A vector containing configurations for the specified pallets.
pub fn get_pallet_configs(pallets: Vec<ESupportedPallets>) -> Vec<PalletConfig> {
    let mut pallets_config: Vec<PalletConfig> = Vec::new();

    for pallet in pallets {
        match pallet {
            ESupportedPallets::PalletUtility => {
                // Get configuration for the utility pallet.
                let pallet_utility_config = PalletUtilityConfig::new();

                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: pallet_utility_config.name,
                    metadata: pallet_utility_config.metadata,
                    runtime: pallet_utility_config.runtime,
                    dependencies: pallet_utility_config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            _ => {}
        }
    }

    pallets_config
}

/// Generates a new project and integrates specified pallets into it.
///
/// # Arguments
///
/// * `project_name` - The name of the project to generate.
/// * `pallets` - A list of supported pallets to be integrated into the project.
pub fn generate_project(project_name: String, pallets: Vec<ESupportedPallets>) {
    // Create a new project directory and copy the template.
    create_new_project(project_name.clone());

    // Generate configurations for the specified pallets.
    let pallet_configs = get_pallet_configs(pallets);

    // Add the pallets to the new project.
    add_pallets(project_name, pallet_configs);
}
