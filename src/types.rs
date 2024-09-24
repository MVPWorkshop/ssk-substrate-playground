use std::collections::HashMap;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct PalletConstructRuntimeConfig {
    pub index: Option<u32>,
    pub runtime: (String, String),
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct PalletTraitsConfig {
    custom_name: Option<String>,
    type_: String,
    value: String,
    is_not_const: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct PalletGenesisConfig {
    pub config_struct_name: String,
    pub struct_fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PalletRuntimeConfig {
    pub construct_runtime: PalletConstructRuntimeConfig,
    pub pallet_traits: HashMap<String, String>,
    pub additional_pallet_impl_code: Option<String>,
    pub genesis_config: Option<PalletGenesisConfig>,
    pub additional_chain_spec_code: Option<Vec<String>>,
    pub additional_runtime_lib_code: Option<Vec<String>>,
    pub runtime_api_code: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct CargoSimpleDependency {
    package: String,
    version: String,
}

#[derive(Debug, Clone)]
pub struct CargoComplexDependency {
    pub package: String,
    pub version: Option<String>,
    pub alias: String,
    pub default_features: bool,
    pub git_repo: Option<String>,
    pub tag: Option<String>,
    pub branch: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PalletDependencyConfig {
    pub pallet: CargoComplexDependency,
    pub additional_pallets: Option<Vec<CargoComplexDependency>>,
    pub additional_deps: Option<Vec<CargoSimpleDependency>>,
}

#[derive(Debug, Clone)]
pub enum SubstrateVersion {
    One,
    Two,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PalletCategories {
    Accounts,
    Assets,
    Consensus,
    Governance,
    Identity,
    Runtime,
    SmartContracts,
    Uniques,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommonAuthors {
    ParityTechnologies,
    IndividualDevelopers,
    SubstrateDevHub,
}

#[derive(Debug, Clone)]
pub struct PalletMetadata {
    pub description: String,
    pub short_description: String,
    pub compatibility: SubstrateVersion,
    pub license: Option<String>,
    pub authors: Vec<CommonAuthors>,
    pub categories: Option<Vec<PalletCategories>>,
    pub size: usize,
    pub updated: String,
}

#[derive(Debug, Clone)]
pub struct PalletConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

#[derive(Debug, Clone)]
pub enum ESupportedPallets {
    PalletUtility,
    PalletIdentity,
    PalletMultisig,
    PalletProxy,
    PalletUniques,
    Unknown,
}

impl TryFrom<&str> for ESupportedPallets {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Utility" => Ok(ESupportedPallets::PalletUtility),
            "Identity" => Ok(ESupportedPallets::PalletIdentity),
            "Multisig" => Ok(ESupportedPallets::PalletMultisig),
            "Proxy" => Ok(ESupportedPallets::PalletProxy),
            "Uniques" => Ok(ESupportedPallets::PalletUniques),
            _ => Ok(ESupportedPallets::Unknown),
        }
    }
}
