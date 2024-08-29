use super::super::types::*;
use chrono::Utc;

#[derive(Debug, Clone, Copy)]
pub enum PalletTimestampTraits {
    Moment,
    OnTimestampSet,
    MinimumPeriod,
    WeightInfo,
}

#[derive(Debug, Clone)]
pub struct PalletTimestampConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletTimestampConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Timestamp pallet is designed to create a consensus-based time source. This helps ensure that nodes maintain a synchronized view of time that all network participants can agree on.",
            "It defines an _acceptable range_ using a configurable constant to specify how much time must pass before setting the new timestamp. Validator nodes in the network must verify that the timestamp falls within this acceptable range and reject blocks that do not.",
            "NOTE: The timestamp set by this pallet is the recommended way to query the onchain time instead of using block numbers alone. Measuring time with block numbers can cause cumulative calculation errors if depended upon in time critical operations and hence should generally be avoided.",
            ].join("\n");
        PalletTimestampConfig {
            name: String::from("Pallet Timestamp"),
            metadata: PalletMetadata {
                size: 5078,
                updated: Utc::now().timestamp().to_string(),
                license: Some(String::from("Apache-2.0")),
                compatibility: SubstrateVersion::Two,
                authors: vec![CommonAuthors::ParityTechnologies],
                categories: Some(vec![PalletCategories::Consensus].into()),
                short_description: String::from("FRAME Timestamp consensus pallet"),
                description: String::from(pallet_description),
            },
            runtime: PalletRuntimeConfig {
                pallet_traits: vec![
                    (String::from("Moment"), String::from("u64")),
                    (String::from("OnTimestampSet"), String::from("Aura")),
                    (
                        String::from("MinimumPeriod"),
                        String::from("ConstU64<{ SLOT_DURATION / 2 }>"),
                    ),
                    (String::from("WeightInfo"), String::from("()")),
                ]
                .into_iter()
                .collect(),

                additional_runtime_lib_code: None,
                construct_runtime: PalletConstructRuntimeConfig {
                    index: None,
                    runtime: ("Timestamp".to_string(), "pallet_timestamp".to_string()),
                },
                genesis_config: None,
                additional_chain_spec_code: None,
                runtime_api_code: None,
            },
            dependencies: PalletDependencyConfig {
                pallet: CargoComplexDependency {
                    package: String::from("pallet-timestamp"),
                    version: None,
                    alias: String::from("pallet-timestamp"),
                    default_features: false,
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
