use super::super::types::*;
use regex::Regex;
use std::collections::HashMap;

/// Represents the configuration for a manifest pallet.
#[derive(Debug, Clone)]
pub struct ManifestPalletConfig {
    pub name: String,
    pub dependencies: PalletDependencyConfig,
}

/// Utility for managing and updating a Substrate runtime manifest with pallet dependencies.
pub struct SubstrateManifestUtil {
    pallet_config: ManifestPalletConfig,
    runtime_manifest: String,
}

impl SubstrateManifestUtil {
    /// Creates a new instance of `SubstrateManifestUtil`.
    ///
    /// # Arguments
    ///
    /// * `pallet_config` - Configuration for the pallet.
    /// * `runtime_manifest` - The current content of the runtime's manifest file.
    ///
    /// # Returns
    ///
    /// A new instance of `SubstrateManifestUtil`.
    pub fn new(pallet_config: ManifestPalletConfig, runtime_manifest: String) -> Self {
        SubstrateManifestUtil {
            pallet_config,
            runtime_manifest,
        }
    }

    /// Generates a formatted string representing a complex dependency configuration for the pallet.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to a `CargoComplexDependency` containing the configuration details.
    ///
    /// # Returns
    ///
    /// A string representing the dependency in the inline TOML format.
    fn generate_complex_dependency_config(&self, config: &CargoComplexDependency) -> String {
        let mut dependency_code = format!("{} = {{ ", config.alias);

        if let Some(ref git_repo) = config.git_repo {
            dependency_code += &format!("git = '{}', ", git_repo);
        }

        if let Some(ref tag) = config.tag {
            dependency_code += &format!("tag = '{}', ", tag);
        }

        if let Some(ref branch) = config.branch {
            dependency_code += &format!("branch = '{}', ", branch);
        }

        // Not required for now maybe helpful in future configurations.
        // dependency_code += &format!("version = '{:?}', ", config.version);
        dependency_code += &format!("default-features = {}", config.default_features);

        // Close the inline table
        dependency_code += " }\n";

        dependency_code
    }

    /// Retrieves the regular expression for matching the `[features]` section in the manifest.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Regex` if the `[features]` section is found, or an error message.
    fn get_manifest_features_code(&self) -> Result<Regex, &'static str> {
        let re = Regex::new(r"\[features\][\s\S]+std\s+=\s+\[(?P<std_deps>[\s\S]+)\]").unwrap();
        if re.is_match(&self.runtime_manifest) {
            Ok(re)
        } else {
            Err("No manifest '[features]' found")
        }
    }

    /// Retrieves the regular expression for matching the `[dependencies]` section in the manifest.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Regex` if the `[dependencies]` section is found, or an error message.
    fn get_manifest_simple_deps(&self) -> Result<Regex, &'static str> {
        let re = Regex::new(r"\[dependencies\]").unwrap();
        if re.is_match(&self.runtime_manifest) {
            Ok(re)
        } else {
            Err("No '[dependencies]' found inside code")
        }
    }

    /// Adds the main pallet's dependency to the runtime manifest.
    ///
    /// This function generates the necessary configuration and updates the `[features]` and
    /// `[dependencies]` sections of the manifest.
    fn add_pallet_to_manifest(&mut self) {
        let dependency_code =
            self.generate_complex_dependency_config(&self.pallet_config.dependencies.pallet);
        let std_code = format!(
            "    '{}/std',\n",
            self.pallet_config.dependencies.pallet.alias
        );

        let manifest_features = self.get_manifest_features_code().unwrap();
        let old_manifest_features_code = manifest_features
            .find(&self.runtime_manifest)
            .unwrap()
            .as_str();
        let mut new_manifest_features_code = dependency_code + old_manifest_features_code;

        let old_std = manifest_features
            .captures(&self.runtime_manifest)
            .unwrap()
            .name("std_deps")
            .unwrap()
            .as_str();
        let new_std = format!("{}{}", old_std, std_code);

        new_manifest_features_code = new_manifest_features_code.replace(old_std, &new_std);

        self.runtime_manifest = self
            .runtime_manifest
            .replace(old_manifest_features_code, &new_manifest_features_code);
    }

    /// Adds additional dependencies to the runtime manifest.
    ///
    /// This function handles both simple and complex dependencies and updates the `[features]` and
    /// `[dependencies]` sections accordingly.
    fn add_additional_dependencies(&mut self) {
        if let Some(ref additional_pallets) = self.pallet_config.dependencies.additional_pallets {
            let manifest_features = self.get_manifest_features_code().unwrap();
            let old_manifest_features_code = manifest_features
                .find(&self.runtime_manifest)
                .unwrap()
                .as_str()
                .to_string();
            let old_std = manifest_features
                .captures(&self.runtime_manifest)
                .unwrap()
                .name("std_deps")
                .unwrap()
                .as_str()
                .to_string();

            let mut new_manifest_features_code = old_manifest_features_code.clone();
            let mut new_std = old_std.clone();

            let simple_dependencies = self.get_manifest_simple_deps().unwrap();
            let old_simple_dependencies_code = simple_dependencies
                .find(&self.runtime_manifest)
                .unwrap()
                .as_str()
                .to_string();
            let mut new_simple_dependencies_code = old_simple_dependencies_code.clone();

            for dependency in additional_pallets {
                if dependency.git_repo.is_none()
                    && dependency.tag.is_none()
                    && dependency.branch.is_none()
                {
                    new_simple_dependencies_code +=
                        &format!("\n{} = '{:?}'", dependency.package, dependency.version);
                } else {
                    new_manifest_features_code = self
                        .generate_complex_dependency_config(dependency)
                        + &new_manifest_features_code;
                    new_std += &format!("    '{}/std',\n", dependency.alias);
                }
            }

            new_manifest_features_code = new_manifest_features_code.replace(&old_std, &new_std);

            self.runtime_manifest = self
                .runtime_manifest
                .replace(&old_manifest_features_code, &new_manifest_features_code);
            self.runtime_manifest = self
                .runtime_manifest
                .replace(&old_simple_dependencies_code, &new_simple_dependencies_code);
        }
    }

    /// Generates the updated runtime manifest code.
    ///
    /// This function calls the necessary internal methods to add the pallet and its dependencies to
    /// the runtime manifest and returns the updated manifest content.
    ///
    /// # Returns
    ///
    /// A string containing the updated runtime manifest.
    pub fn generate_code(&mut self) -> String {
        self.add_pallet_to_manifest();
        self.add_additional_dependencies();
        self.runtime_manifest.clone()
    }
}
