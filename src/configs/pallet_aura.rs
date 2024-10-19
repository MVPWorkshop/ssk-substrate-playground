use super::super::types::*;

#[derive(Debug, Clone, Copy)]
pub enum PalletUtilityTraits {
    AuthorityId,
    DisabledValidators,
    MaxAuthorities,
    AllowMultipleBlocksPerSlot,
    SlotDuration,
}

#[derive(Debug, Clone)]
pub struct PalletAuraConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletAuraConfig {
    fn default() -> Self {
        PalletAuraConfig::new()
    }
}

impl PalletAuraConfig {
    pub fn new() -> Self {
        let pallet_description =
            "The Aura module extends Aura consensus by managing offline reporting.";

        PalletAuraConfig {
            name: String::from("Pallet Aura"),
            metadata: PalletMetadata {
                size: 5078,
                is_essential: true,
                license: Some(String::from("Apache-2.0")),
                compatibility: SubstrateVersion::Two,
                authors: vec![CommonAuthors::ParityTechnologies],
                categories: Some(vec![PalletCategories::Consensus]),
                short_description: String::from("FRAME AURA consensus pallet"),
                description: String::from(pallet_description),
            },
            runtime: PalletRuntimeConfig {
                pallet_traits: vec![
                    (String::from("AuthorityId"), String::from("AuraId")),
                    (String::from("DisabledValidators"), String::from("()")),
                    (String::from("MaxAuthorities"), String::from("ConstU32<32>")),
                    (
                        String::from("AllowMultipleBlocksPerSlot"),
                        String::from("ConstBool<false>"),
                    ),
                    (
                        String::from("SlotDuration"),
                        String::from("pallet_aura::MinimumPeriodTimesTwo<Runtime>"),
                    ),
                ]
                .into_iter()
                .collect(),
                additional_pallet_impl_code: None,
                additional_runtime_lib_code: Some(vec![String::from(
                    "use sp_consensus_aura::sr25519::AuthorityId as AuraId;",
                )]),
                construct_runtime: PalletConstructRuntimeConfig {
                    index: None,
                    runtime: ("Aura".to_string(), "pallet_aura".to_string()),
                },
                genesis_config: Some(PalletGenesisConfig {
                    config_struct_name: String::from("aura"),
                    struct_fields: vec![(
                        "authorities".to_string(),
                        "initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>(),"
                            .to_string(),
                    )]
                    .into_iter()
                    .collect(),
                }),
                additional_chain_spec_code: Some(
                    vec!["use node_template_runtime::AuraId;".to_string()]
                        .into_iter()
                        .collect(),
                ),
                runtime_api_code: None,
            },
            dependencies: PalletDependencyConfig {
                pallet: CargoComplexDependency {
                    package: String::from("pallet-aura"),
                    version: None,
                    alias: String::from("pallet-aura"),
                    default_features: false,
                    git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                    tag: Some("polkadot-v1.14.0".to_string()),
                    branch: None,
                },
                additional_pallets: None,
                additional_deps: None,
                required_pallets: None,
            },
        }
    }
}
