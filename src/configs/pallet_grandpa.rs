use super::super::types::*;

#[derive(Debug, Clone, Copy)]
pub enum PalletGrandpaTraits {
    RuntimeEvent,
    WeightInfo,
    MaxAuthorities,
    MaxNominators,
    MaxSetIdSessionEntries,
    KeyOwnerProof,
    EquivocationReportSystem,
}

#[derive(Debug, Clone)]
pub struct PalletGrandpaConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletGrandpaConfig {
    fn default() -> Self {
        PalletGrandpaConfig::new()
    }
}

impl PalletGrandpaConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "This manages the GRANDPA authority set ready for the native code. These authorities are only for GRANDPA finality, not for consensus overall.",
        ].join("\n");
        PalletGrandpaConfig {
            name: String::from("Pallet Grandpa"),
            metadata: PalletMetadata {
                size: 5078,
                is_essential: true,
                license: Some(String::from("Apache-2.0")),
                compatibility: SubstrateVersion::Two,
                authors: vec![CommonAuthors::ParityTechnologies],
                categories: Some(vec![PalletCategories::Consensus]),
                short_description: String::from("FRAME Grandpa consensus pallet"),
                description: pallet_description,
            },
            runtime: PalletRuntimeConfig {
                pallet_traits: vec![
                    (String::from("RuntimeEvent"), String::from("RuntimeEvent")),
                    (String::from("WeightInfo"), String::from("()")),
                    (String::from("MaxAuthorities"), String::from("ConstU32<32>")),
                    (String::from("MaxNominators"), String::from("ConstU32<0>")),
                    (
                        String::from("MaxSetIdSessionEntries"),
                        String::from("ConstU64<0>"),
                    ),
                    (String::from("KeyOwnerProof"), String::from("sp_core::Void")),
                    (String::from("EquivocationReportSystem"), String::from("()")),
                ]
                .into_iter()
                .collect(),
                additional_pallet_impl_code: None,
                additional_runtime_lib_code: Some(vec![String::from(
                    "use pallet_grandpa::AuthorityId as GrandpaId;",
                )]),
                construct_runtime: PalletConstructRuntimeConfig {
                    index: None,
                    runtime: ("Grandpa".to_string(), "pallet_grandpa".to_string()),
                },
                genesis_config: Some(PalletGenesisConfig {
                    config_struct_name: String::from("grandpa"),
                    struct_fields: vec![(
                        "authorities".to_string(),
                        "initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>(),"
                            .to_string(),
                    )]
                    .into_iter()
                    .collect(),
                }),
                additional_chain_spec_code: None,
                runtime_api_code: None,
            },

            dependencies: PalletDependencyConfig {
                pallet: CargoComplexDependency {
                    package: String::from("pallet-grandpa"),
                    version: None,
                    alias: String::from("pallet-grandpa"),
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
