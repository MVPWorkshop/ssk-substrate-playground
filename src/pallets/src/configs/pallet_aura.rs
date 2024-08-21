use super::super::types::*;
use chrono::Local;

#[derive(Debug, Clone, Copy)]
pub enum PalletUtilityTraits {
    AuthorityId,
    DisabledValidators,
    MaxAuthorities,
    AllowMultipleBlocksPerSlot,
    SlotDuration
}

#[derive(Debug, Clone)]
pub struct PalletAuraConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletAuraConfig {
    pub fn new() -> Self {
        let pallet_description = "The Aura module extends Aura consensus by managing offline reporting.";

        PalletAuraConfig {
            name: String::from("Pallet Aura"),
            metadata: PalletMetadata {
                size: 5078,
                updated: Local::now().to_string(),
                license: Some(String::from("Apache-2.0")),
                compatibility: SubstrateVersion::Two,
                authors: vec![CommonAuthors::ParityTechnologies],
                categories: Some(vec![PalletCategories::Consensus].into()),
                short_description: String::from("FRAME AURA consensus pallet"),
                description: String::from(pallet_description),
            },
            runtime: PalletRuntimeConfig {
                pallet_traits: vec![
                    (String::from("AuthorityId"), String::from("AuraId")),
                    (String::from("DisabledValidators"), String::from("()")),
                    (String::from("MaxAuthorities"), String::from("ConstU32<32>")),
                    (String::from("AllowMultipleBlocksPerSlot"), String::from("ConstBool<false>")),
                    (String::from("SlotDuration"), String::from("pallet_aura::MinimumPeriodTimesTwo<Runtime>")),
                ].into_iter().collect(),


                additional_runtime_lib_code: Some(vec![
                    String::from("use sp_consensus_aura::sr25519::AuthorityId as AuraId;"),
                ]),
                construct_runtime: PalletConstructRuntimeConfig {
                    generic: None,
                },
                genesis_config: Some(PalletGenesisConfig {
                    config_struct_name: String::from("aura"),
                    struct_fields: vec![
                        ("authorities".to_string(), "initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>(),".to_string()),
                    ].into_iter().collect(),
                }),
                additional_chain_spec_code: Some(vec![
                    "use node_template_runtime::AuraId;".to_string(),
                ].into_iter().collect()),
            },
            dependencies: PalletDependencyConfig {
                pallet: CargoComplexDependency {
                    package: String::from("pallet-aura"),
                    version: None,
                    alias: String::from("pallet-aura"),
                    default_features: None,
                    git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                    tag: Some("polkadot-v1.14.0".to_string()),
                    branch: None,
                },
                additional_pallets: None,
                additional_deps: None,
            },
        }
    }
}
