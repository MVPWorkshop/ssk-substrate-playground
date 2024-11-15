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
    pub optional_parameter_types: Option<Vec<ParameterType>>,
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

#[cfg(test)]
mod tests {
    use crate::{
        code_generator::{generate_project, get_all_pallet_configs_from_dir},
        types::{ParameterType, ParameterTypeExpression, ParameterTypePrefix},
        CONFIG_DIR,
    };

    #[tokio::test]
    #[ignore]
    async fn build_all_pallets() {
        let pallets = get_all_pallet_configs_from_dir(CONFIG_DIR).await.unwrap();
        let total_pallets_len = pallets.len();
        let pallets = pallets
            .into_iter()
            .filter(|pallet_config| !pallet_config.metadata.is_essential)
            .collect::<Vec<_>>();
        assert!(
            pallets.len() == total_pallets_len - 6,
            "6 essential pallets are not filtered out",
        );
        let _ = generate_project(&"test_project".to_string(), pallets).await;
    }
    #[tokio::test]
    #[ignore]
    async fn build_optional_parameter_types() {
        let pallets = get_all_pallet_configs_from_dir(CONFIG_DIR).await.unwrap();
        let mut bounties = pallets
            .into_iter()
            .find(|pallet_config| pallet_config.name == "Pallet bounties")
            .unwrap();
        let opt = ParameterType {
            name: "BountyDepositBase".to_string(),
            description: "The base amount of deposit for a bounty".to_string(),
            prefix: ParameterTypePrefix::Const,
            p_type: "Balance".to_string(),
            expression: ParameterTypeExpression {
                default_unit: "DOLLARS".to_string(),
                default_multiiplier: Some(1),
                format: "{} * {}".to_string(),
                possible_units: vec![
                    "DOLLARS".to_string(),
                    "CENTS".to_string(),
                    "MILLICENTS".to_string(),
                ],
                multiplier_configurable: true,
            },
        };
        bounties.runtime.optional_parameter_types = Some(vec![opt]);
        println!("{}", toml::to_string_pretty(&bounties).unwrap());
    }

    #[test]
    fn test_parameter_type_prefix() {
        assert_eq!(ParameterTypePrefix::Const.to_string(), " const ");
        assert_eq!(ParameterTypePrefix::Type.to_string(), " type ");
        assert_eq!(ParameterTypePrefix::Empty.to_string(), " ");
    }
}
