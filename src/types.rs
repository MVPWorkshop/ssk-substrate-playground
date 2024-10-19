use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumIter};

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
    pub index: Option<u32>,
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
pub struct PalletGenesisConfig {
    pub config_struct_name: String,
    pub struct_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PalletDependencyConfig {
    pub pallet: CargoComplexDependency,
    pub additional_pallets: Option<Vec<CargoComplexDependency>>,
    pub additional_deps: Option<Vec<CargoSimpleDependency>>,
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

#[derive(PartialEq, Eq, Hash, Clone, Debug, EnumIter, Serialize, Deserialize, Display)]
pub enum ESupportedPallets {
    PalletAssets,
    PalletAura,     // non-optional
    PalletBalances, // non-optional
    PalletBounties,
    PalletChildBounties,
    PalletCollective,
    PalletDemocracy,
    PalletGrandpa, // non-optional
    PalletIdentity,
    PalletMembership,
    PalletMultisig,
    PalletNfts,
    PalletProxy,
    PalletScheduler,
    PalletSociety,
    PalletSudo,               // non-optional
    PalletTimestamp,          // non-optional
    PalletTransactionPayment, // non-optional
    PalletTreasury,
    PalletUniques,
    PalletUtility,
    PalletVesting,
    Unknown,
}

impl ESupportedPallets {
    pub fn toml_path(&self) -> String {
        format!(
            "src/toml_configs/{}.toml",
            self.to_string().to_case(Case::Snake)
        )
    }
}

impl TryFrom<&str> for ESupportedPallets {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Assets" => Ok(ESupportedPallets::PalletAssets),
            "Treasury" => Ok(ESupportedPallets::PalletTreasury),
            "Vesting" => Ok(ESupportedPallets::PalletVesting),
            "Society" => Ok(ESupportedPallets::PalletSociety),
            "Utility" => Ok(ESupportedPallets::PalletUtility),
            "Identity" => Ok(ESupportedPallets::PalletIdentity),
            "Multisig" => Ok(ESupportedPallets::PalletMultisig),
            "Proxy" => Ok(ESupportedPallets::PalletProxy),
            "Uniques" => Ok(ESupportedPallets::PalletUniques),
            "Nfts" => Ok(ESupportedPallets::PalletNfts),
            "Membership" => Ok(ESupportedPallets::PalletMembership),
            "ChildBounties" => Ok(ESupportedPallets::PalletChildBounties),
            "Bounties" => Ok(ESupportedPallets::PalletBounties),
            "Collective" => Ok(ESupportedPallets::PalletCollective),
            "Scheduler" => Ok(ESupportedPallets::PalletScheduler),
            _ => Ok(ESupportedPallets::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;
    use crate::code_generator::{get_pallet_configs, get_pallet_configs_depricated};

    #[test]
    #[ignore]
    // This test is ignored because it writes to the file system.
    // Special test used to generate toml files for pallets.
    fn print_pallet_config() {
        let supported_configs = ESupportedPallets::iter().collect::<Vec<_>>();
        let pallet_configs = get_pallet_configs_depricated(supported_configs);
        let directory = "src/toml_configs";
        for pallet_config in pallet_configs {
            let filename = format!(
                "{}/{}.toml",
                directory,
                pallet_config.name.to_lowercase().replace(" ", "_")
            );
            let toml_string = toml::to_string_pretty(&pallet_config).unwrap();
            std::fs::write(filename, toml_string).unwrap();
        }
    }

    #[test]
    fn confirm_two_pallet_configs() {
        let supported_configs = ESupportedPallets::iter().collect::<Vec<_>>();
        let pallet_configs = get_pallet_configs_depricated(supported_configs.clone());
        let toml_pallet_configs = get_pallet_configs(supported_configs).unwrap();
        assert_eq!(pallet_configs, toml_pallet_configs);
    }
}
