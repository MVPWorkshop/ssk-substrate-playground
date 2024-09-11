use super::super::types::*;
use chrono::Utc;

#[derive(Debug, Clone, Copy)]
pub enum PalletBalancesTraits {
    MaxLocks,
    MaxReserves,
    ReserveIdentifier,
    Balance,
    RuntimeEvent,
    DustRemoval,
    ExistentialDeposit,
    AccountStore,
    WeightInfo,
    FreezeIdentifier,
    MaxFreezes,
    RuntimeHoldReason,
    RuntimeFreezeReason,
}

#[derive(Debug, Clone)]
pub struct PalletBalancesConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletBalancesConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Balances pallet provides functionality for handling accounts and balances for a single token.",
            "It makes heavy use of concepts such as Holds and Freezes from the [`frame_support::traits::fungible`] traits, therefore you should read and understand those docs as a prerequisite to understanding this pallet.",
            ].join("\n");
        PalletBalancesConfig {
            name: String::from("Pallet Balances"),
            metadata: PalletMetadata {
                size: 5078,
                updated: Utc::now().timestamp().to_string(),
                license: Some(String::from("Apache-2.0")),
                compatibility: SubstrateVersion::Two,
                authors: vec![CommonAuthors::ParityTechnologies],
                categories: Some(vec![PalletCategories::Assets].into()),
                short_description: String::from("FRAME Balances pallet"),
                description: String::from(pallet_description),
            },
            runtime: PalletRuntimeConfig {
                pallet_traits: vec![
                    (String::from("MaxLocks"), String::from("ConstU32<50>")),
                    (String::from("MaxReserves"), String::from("()")),
                    (String::from("ReserveIdentifier"), String::from("[u8; 8]")),
                    (String::from("Balance"), String::from("Balance")),
                    (String::from("RuntimeEvent"), String::from("RuntimeEvent")),
                    (String::from("DustRemoval"), String::from("()")),
                    (
                        String::from("ExistentialDeposit"),
                        String::from("ConstU128<500>"),
                    ),
                    (String::from("AccountStore"), String::from("System")),
                    (
                        String::from("WeightInfo"),
                        String::from("pallet_balances::weights::SubstrateWeight<Runtime>"),
                    ),
                    (
                        String::from("FreezeIdentifier"),
                        String::from("RuntimeFreezeReason"),
                    ),
                    (
                        String::from("MaxFreezes"),
                        String::from("VariantCountOf<RuntimeFreezeReason>"),
                    ),
                    (
                        String::from("RuntimeHoldReason"),
                        String::from("RuntimeHoldReason"),
                    ),
                    (
                        String::from("RuntimeFreezeReason"),
                        String::from("RuntimeHoldReason"),
                    ),
                ]
                .into_iter()
                .collect(),

                additional_runtime_lib_code: None,
                construct_runtime: PalletConstructRuntimeConfig {
                    index: None,
                    runtime: ("Balances".to_string(), "pallet_balances".to_string()),
                },
                genesis_config: Some(PalletGenesisConfig {
                    config_struct_name: String::from("balances"),
                    struct_fields: vec![(
                        "balances".to_string(),
                        "endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),"
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
                    package: String::from("pallet-balances"),
                    version: None,
                    alias: String::from("pallet-balances"),
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
