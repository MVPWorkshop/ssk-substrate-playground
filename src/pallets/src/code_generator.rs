use super::configs::pallet_aura::PalletAuraConfig;
use super::configs::pallet_balances::PalletBalancesConfig;
use super::configs::pallet_timestamp::PalletTimestampConfig;
use super::configs::pallet_utility::PalletUtilityConfig;
use super::types::PalletConfig;
use super::util::pallets_config_to_model;
use super::utils::file_manager::{
    copy_dir_recursive, create_new_folder, read_file_to_string, replace_file_content,
};
use super::utils::manifest::ManifestPalletConfig;
use super::utils::manifest::SubstrateManifestUtil;
use super::utils::runtime::{GeneratedRuntime, SubstrateRuntimeUtil};
use actix_web::{web, App, HttpServer};
use std::path::Path;
use std::process;

pub fn generate_project() {
    let pallet_utility_config = PalletUtilityConfig::new();
    // println!("Pallet Utility Config: {:?}", pallet_utility_config);

    // Specify the file path
    let runtime_file_path = "../../templates/solochain/basic/runtime/src/lib.rs";

    let chainspec_file_path = "../../templates/solochain/basic/node/src/chain_spec.rs";

    // Call the function to read the file
    let runtime_string = read_file_to_string(runtime_file_path).unwrap();

    let chainspec_string = read_file_to_string(chainspec_file_path).unwrap();

    // println!("File content:\n{}", runtime_string);
    // println!("File content:\n{}", chainspec_string);

    let config = PalletConfig {
        name: pallet_utility_config.name,
        metadata: pallet_utility_config.metadata,
        runtime: pallet_utility_config.runtime,
        dependencies: pallet_utility_config.dependencies.clone(),
    };

    let mut util = SubstrateRuntimeUtil::new(config, runtime_string, chainspec_string);
    // println!("{:#?}", util);
    let updated_runtime = util.generate_code();
    // println!("{}", updated_runtime.updated_runtime_code);

    let base_path = Path::new("../../generated_code");
    let folder_name = "pankaj";

    match create_new_folder(base_path, folder_name) {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => eprintln!("Error creating folder: {}", e),
    }

    let src = Path::new("../../templates/solochain/basic");
    let path = format!("../../generated_code/{}", folder_name);
    let dest = Path::new(&path);

    match copy_dir_recursive(src, dest) {
        Ok(_) => println!("Folder cloned successfully!"),
        Err(e) => eprintln!("Error cloning folder: {}", e),
    }

    let pallet_config = ManifestPalletConfig {
        name: "utility_pallet".to_string(),
        dependencies: pallet_utility_config.dependencies.clone(),
    };

    // Specify the file path
    let manifest_path = format!("../../generated_code/{}/runtime/cargo.toml", folder_name);

    // Call the function to read the file
    match read_file_to_string(&manifest_path) {
        Ok(content) => {
            // println!("File content:\n{}", content);
            let mut util = SubstrateManifestUtil::new(pallet_config, content);
            let updated_manifest = util.generate_code();
            // println!("{}", updated_manifest);

            match replace_file_content(Path::new(&manifest_path), &updated_manifest) {
                Ok(_) => println!("Operation completed successfully."),
                Err(e) => eprintln!("Error replacing file content: {}", e),
            }
        }
        Err(e) => {
            println!("Failed to read the file: {}", e);
        }
    }

    let file_path = Path::new("../../generated_code/pankaj/runtime/src/lib.rs");

    match replace_file_content(file_path, &updated_runtime.updated_runtime_code) {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => eprintln!("Error replacing file content: {}", e),
    }
}
