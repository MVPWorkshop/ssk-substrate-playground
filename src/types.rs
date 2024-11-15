use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::Display;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum PalletModuleParts {
    Module,
    Call,
    Storage,
    Event,
    Origin,
    Config,
    Inherent,
    ValidateUnsigned,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PalletConstructRuntimeConfig {
    pub runtime: (String, String),
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PalletTraitsConfig {
    custom_name: Option<String>,
    type_: String,
    value: String,
    is_not_const: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PalletRuntimeConfig {
    pub construct_runtime: PalletConstructRuntimeConfig,
    pub pallet_traits: HashMap<String, String>,
    pub additional_pallet_impl_code: Option<String>,
    pub additional_chain_spec_code: Option<Vec<String>>,
    pub additional_runtime_lib_code: Option<Vec<String>>,
    pub runtime_api_code: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CargoSimpleDependency {
    package: String,
    version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CargoComplexDependency {
    pub package: String,
    pub version: Option<String>,
    pub alias: String,
    pub default_features: bool,
    pub git_repo: Option<String>,
    pub tag: Option<String>,
    pub branch: Option<String>,
}

impl CargoComplexDependency {
    pub fn name_cebab_case(&self) -> String {
        self.alias.to_lowercase().replace(" ", "-")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PalletDependencyConfig {
    pub pallet: CargoComplexDependency,
    pub additional_pallets: Option<Vec<CargoComplexDependency>>,
    pub additional_deps: Option<Vec<CargoSimpleDependency>>,
    pub required_pallets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum SubstrateVersion {
    One,
    Two,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, Display)]
pub enum PalletCategories {
    Accounts,
    Assets,
    Consensus,
    Governance,
    Identity,
    Runtime,
    SmartContracts,
    NFT,
    Other,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq)]
pub enum CommonAuthors {
    ParityTechnologies,
    IndividualDevelopers,
    SubstrateDevHub,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PalletMetadata {
    pub description: String,
    pub short_description: String,
    pub compatibility: SubstrateVersion,
    pub license: Option<String>,
    pub authors: Vec<CommonAuthors>,
    pub categories: Option<Vec<PalletCategories>>,
    pub size: usize,
    pub is_essential: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PalletConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

#[cfg(test)]
mod tests {
    use crate::{
        code_generator::{generate_project, get_all_pallet_configs_from_dir},
        CONFIG_DIR,
    };

    #[tokio::test]
    #[ignore]
    async fn build_all_pallets() {
        let pallets = get_all_pallet_configs_from_dir(CONFIG_DIR).await.unwrap();
        let _ = generate_project(&"test_project".to_string(), pallets).await;
    }
}
