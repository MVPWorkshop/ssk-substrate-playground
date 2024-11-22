use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct PalletConstructRuntimeConfig {
    pub runtime: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct PalletGenesisConfig {
    pub config_struct_name: String,
    pub struct_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct PalletRuntimeConfig {
    pub construct_runtime: PalletConstructRuntimeConfig,
    pub pallet_traits: HashMap<String, String>,
    pub additional_pallet_impl_code: Option<String>,
    pub genesis_config: Option<PalletGenesisConfig>,
    pub additional_chain_spec_code: Option<Vec<String>>,
    pub additional_runtime_lib_code: Option<Vec<String>>,
    pub runtime_api_code: Option<String>,
    pub optional_parameter_types: Option<HashMap<String, ParameterType>>,
}
#[derive(EnumString, Display, Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Enum)]
#[serde(rename_all = "snake_case")]
pub enum ParameterTypePrefix {
    #[strum(serialize = " ")]
    Empty,
    #[strum(serialize = " const ")]
    Const,
    #[strum(serialize = " type ")]
    Type,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct ParameterTypeExpression {
    pub default_unit: String,
    pub default_multiiplier: Option<i64>,
    pub format: String,
    pub possible_units: Vec<String>,
    pub multiplier_configurable: bool,
    pub configured_multiplier: Option<i64>,
    pub configured_unit: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct ParameterType {
    pub name: String,
    pub description: String,
    pub prefix: ParameterTypePrefix,
    pub p_type: String,
    pub expression: ParameterTypeExpression,
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct CargoSimpleDependency {
    package: String,
    version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct PalletDependencyConfig {
    pub pallet: CargoComplexDependency,
    pub additional_pallets: Option<Vec<CargoComplexDependency>>,
    pub additional_deps: Option<Vec<CargoSimpleDependency>>,
    pub required_pallets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Enum)]
pub enum SubstrateVersion {
    One,
    Two,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, Display, Enum)]
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, Enum)]
pub enum CommonAuthors {
    ParityTechnologies,
    IndividualDevelopers,
    SubstrateDevHub,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Object)]
pub struct PalletConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}
