use super::super::types::*;

#[derive(Debug, Clone, Copy)]
pub enum PalletTransactionPaymentTraits {
    RuntimeEvent,
    OnChargeTransaction,
    OperationalFeeMultiplier,
    WeightToFee,
    LengthToFee,
    FeeMultiplierUpdate,
}

#[derive(Debug, Clone)]
pub struct PalletTransactionPaymentConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletTransactionPaymentConfig {
    fn default() -> Self {
        PalletTransactionPaymentConfig::new()
    }
}

impl PalletTransactionPaymentConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "This pallet provides the basic logic needed to pay the absolute minimum amount needed for a transaction to be included. This includes:",
            "- _base fee_: This is the minimum amount a user pays for a transaction. It is declared as a base _weight_ in the runtime and converted to a fee using `WeightToFee`.",
            "- _weight fee_: A fee proportional to amount of weight a transaction consumes.",
            "- _length fee_: A fee proportional to the encoded length of the transaction.",
            "- _tip_: An optional tip. Tip increases the priority of the transaction, giving it a higher chance to be included by the transaction queue.",
            "The base fee and adjusted weight and length fees constitute the _inclusion fee_, which is the minimum fee for a transaction to be included in a block.",
        ].join("\n");
        PalletTransactionPaymentConfig {
            name: String::from("Pallet Transaction Payment"),
            metadata: PalletMetadata {
                size: 5078,
                //updated: Utc::now().timestamp().to_string(),
                license: Some(String::from("Apache-2.0")),
                compatibility: SubstrateVersion::Two,
                authors: vec![CommonAuthors::ParityTechnologies],
                categories: Some(vec![PalletCategories::Consensus]),
                short_description: String::from("FRAME Transaction payment pallet"),
                description: pallet_description,
            },
            runtime: PalletRuntimeConfig {
                pallet_traits: vec![
                    (String::from("RuntimeEvent"), String::from("RuntimeEvent")),
                    (String::from("OnChargeTransaction"), String::from("FungibleAdapter<Balances, ()>")),
                    (String::from("OperationalFeeMultiplier"), String::from("ConstU8<5>")),
                    (String::from("WeightToFee"), String::from("IdentityFee<Balance>")),
                    (String::from("LengthToFee"), String::from("IdentityFee<Balance>")),
                    (String::from("FeeMultiplierUpdate"), String::from("ConstFeeMultiplier<Multiplier::one()>")),
                ]
                    .into_iter()
                    .collect(),
                additional_pallet_impl_code: None,
                additional_runtime_lib_code: Some(vec![String::from(
                    "use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};",
                )]),
                construct_runtime: PalletConstructRuntimeConfig {
                    index: None,
                    runtime: ("TransactionPayment".to_string(), "pallet_transaction_payment".to_string()),
                },
                genesis_config: None,
                additional_chain_spec_code: None,
                runtime_api_code: None,
            },

            dependencies: PalletDependencyConfig {
                pallet: CargoComplexDependency {
                    package: String::from("pallet-transaction-payment"),
                    version: None,
                    alias: String::from("pallet-transaction-payment"),
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
