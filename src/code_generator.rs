use super::configs::pallet_identity::PalletIdentityConfig;
use super::configs::pallet_membership::PalletMembershipConfig;
use super::configs::pallet_nfts::PalletNftsConfig;
use super::configs::pallet_proxy::PalletProxyConfig;
use super::configs::pallet_uniques::PalletUniquesConfig;
use super::configs::pallet_utility::PalletUtilityConfig;
use super::configs::pallet_child_bounties::PalletChildBountiesConfig;
use super::types::PalletConfig;
use super::utils::file_manager::{
    copy_dir_recursive, create_new_folder, read_file_to_string, replace_file_content,
};
use super::utils::manifest::ManifestPalletConfig;
use super::utils::manifest::SubstrateManifestUtil;
use super::utils::runtime::SubstrateRuntimeUtil;
use crate::configs::pallet_assets::PalletAssetsConfig;
use crate::configs::pallet_multisig::PalletMultisigConfig;
use crate::configs::pallet_treasury::PalletTreasuryConfig;
use crate::types::ESupportedPallets;
use log::{error, info};
use std::path::Path;

/// Creates a new project directory and copies a basic template into it.
///
/// # Arguments
///
/// * `project_name` - The name of the project to be created.
pub fn create_new_project(project_name: String) {
    // Base path for generated projects.
    let base_path = Path::new("generated_code");

    // Create a new folder for the project.
    if let Err(e) = create_new_folder(base_path, &project_name) {
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
pub fn add_pallets(project_name: String, pallet_configs: Vec<PalletConfig>) {
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
                let config = PalletUtilityConfig::new();

                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletIdentity => {
                // Get configuration for the identity pallet.
                let config = PalletIdentityConfig::new();

                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletMultisig => {
                // Get configuration for the multisig pallet.
                let config = PalletMultisigConfig::new();

                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletProxy => {
                // Get configuration for the proxy pallet.
                let config = PalletProxyConfig::new();

                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletUniques => {
                // Get configuration for the uniques pallet.
                let config = PalletUniquesConfig::new();
                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletMembership => {
                // Get configuration for the Membership pallet.
                let config = PalletMembershipConfig::new();
                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletNfts => {
                // Get configuration for the uniques pallet.
                let config = PalletNftsConfig::new();
                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletAssets => {
                // Get configuration for the assets pallet.
                let config = PalletAssetsConfig::new();
                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletTreasury => {
                // Get configuration for the treasury pallet.
                let config = PalletTreasuryConfig::new();
                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
                };
                pallets_config.push(pallet_config);
            }
            ESupportedPallets::PalletChildBounties => {
                // Get configuration for the treasury pallet.
                let config = PalletChildBountiesConfig::new();
                // Create a pallet configuration and add it to the list.
                let pallet_config = PalletConfig {
                    name: config.name,
                    metadata: config.metadata,
                    runtime: config.runtime,
                    dependencies: config.dependencies.clone(),
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

    print!("Created project: {}\n", project_name.clone());

    // Generate configurations for the specified pallets.
    let pallet_configs = get_pallet_configs(pallets);

    // Add the pallets to the new project.
    add_pallets(project_name.clone(), pallet_configs);
    print!("Added pallets to the project: {}\n", project_name);
}
