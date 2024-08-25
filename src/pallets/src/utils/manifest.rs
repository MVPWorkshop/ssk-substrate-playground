use super::super::types::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ManifestPalletConfig {
    pub name: String,
    pub dependencies: PalletDependencyConfig,
}

pub struct SubstrateManifestUtil {
    pallet_config: ManifestPalletConfig,
    runtime_manifest: String,
}

impl SubstrateManifestUtil {
    pub fn new(pallet_config: ManifestPalletConfig, runtime_manifest: String) -> Self {
        SubstrateManifestUtil {
            pallet_config,
            runtime_manifest,
        }
    }

    fn generate_complex_dependency_config(&self, config: &CargoComplexDependency) -> String {
        let mut dependency_code = format!("[dependencies.{}]\n", config.alias);
        dependency_code += &format!("default-features = {}\n", config.default_features);
        if let Some(ref git_repo) = config.git_repo {
            dependency_code += &format!("git = '{}'\n", git_repo);
        }
        dependency_code += &format!("package = '{}'\n", config.package);
        if let Some(ref tag) = config.tag {
            dependency_code += &format!("tag = '{}'\n", tag);
        }
        if let Some(ref branch) = config.branch {
            dependency_code += &format!("branch = '{}'\n", branch);
        }
        dependency_code += &format!("version = '{:?}'\n\n", config.version);

        dependency_code
    }

    fn get_manifest_features_code(&self) -> Result<Regex, &'static str> {
        let re = Regex::new(r"\[features\][\s\S]+std\s+=\s+\[(?P<std_deps>[\s\S]+)\]").unwrap();
        if re.is_match(&self.runtime_manifest) {
            Ok(re)
        } else {
            Err("No manifest '[features]' found")
        }
    }

    fn get_manifest_simple_deps(&self) -> Result<Regex, &'static str> {
        let re = Regex::new(r"\[dependencies\]").unwrap();
        if re.is_match(&self.runtime_manifest) {
            Ok(re)
        } else {
            Err("No '[dependencies]' found inside code")
        }
    }

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

    pub fn generate_code(&mut self) -> String {
        self.add_pallet_to_manifest();
        self.add_additional_dependencies();
        self.runtime_manifest.clone()
    }
}
