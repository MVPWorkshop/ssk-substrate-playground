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

pub fn create_new_project(project_name: String) {
    let base_path = Path::new("../../generated_code");

    match create_new_folder(base_path, &project_name) {
        Ok(_) => println!("Created new project folder!"),
        Err(e) => eprintln!("Error creating folder: {}", e),
    }

    let src = Path::new("../../templates/solochain/basic");
    let path = format!("../../generated_code/{}", project_name);
    let dest = Path::new(&path);

    match copy_dir_recursive(src, dest) {
        Ok(_) => println!("Project created successfully!"),
        Err(e) => eprintln!("Error in creating the project: {}", e),
    }
}

pub fn add_pallets(project_name: String, pallet_configs: Vec<PalletConfig>) {
    for pallet_config in pallet_configs {
        let project_directory = format!("../../generated_code/{}", project_name);

        // Specify the file path
        let runtime_file_path = project_directory.clone() + "/runtime/src/lib.rs";
        let chain_spec_file_path = project_directory.clone() + "/node/src/chain_spec.rs";
        let manifest_path = project_directory + "/runtime/cargo.toml";

        // Generating the manifest config.
        let pallet_manifest_config = ManifestPalletConfig {
            name: pallet_config.name.to_string(),
            dependencies: pallet_config.dependencies.clone(),
        };

        // Call the function to read the file
        match read_file_to_string(&manifest_path) {
            Ok(content) => {
                let mut util = SubstrateManifestUtil::new(pallet_manifest_config, content);
                let updated_manifest = util.generate_code();
                match replace_file_content(Path::new(&manifest_path), &updated_manifest) {
                    Ok(_) => println!("Operation completed successfully."),
                    Err(e) => eprintln!("Error replacing file content: {}", e),
                }
            }
            Err(e) => {
                println!("Failed to read the file: {}", e);
            }
        }

        let runtime_string = read_file_to_string(&runtime_file_path).unwrap();
        let chain_spec_string = read_file_to_string(&chain_spec_file_path).unwrap();

        let mut pallet_config =
            SubstrateRuntimeUtil::new(pallet_config, runtime_string, chain_spec_string);
        let updated_code = pallet_config.generate_runtime_code();

        // Replacing the Runtime from new runtime generated.
        let runtime_path = Path::new(&runtime_file_path);
        match replace_file_content(runtime_path, &updated_code.updated_runtime_code) {
            Ok(_) => println!("Runtime updated successfully."),
            Err(e) => eprintln!("Error replacing Runtime content: {}", e),
        }

        // Replacing the chain spec file from new chain spec generated.
        let chain_spec_path = Path::new(&chain_spec_file_path);
        match replace_file_content(chain_spec_path, &updated_code.updated_chain_spec_code) {
            Ok(_) => println!("Chain spec updated successfully."),
            Err(e) => eprintln!("Error replacing Chain spec content: {}", e),
        }
    }
}

pub fn get_pallet_configs(pallets: Vec<ESupportedPallets>) -> Vec<PalletConfig> {
    let mut pallets_config: Vec<PalletConfig> = Vec::new();

    for pallet in pallets {
        match pallet {
            ESupportedPallets::PalletUtility => {
                let pallet_utility_config = PalletUtilityConfig::new();

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

pub fn generate_project(project_name: String, pallets: Vec<ESupportedPallets>) {
    // Creating the new project according to the opted option.
    create_new_project(project_name.clone());

    // Create the pallet configs for the given pallets to integrate.
    let pallet_configs = get_pallet_configs(pallets);

    // Add the pallet to the generated project.
    add_pallets(project_name, pallet_configs);
}
