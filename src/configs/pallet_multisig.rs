use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletMultisigTraits {
    RuntimeEvent,
    RuntimeCall,
    Currency,
    DepositBase,
    DepositFactor,
    MaxSignatories,
    WeightInfo,
}

impl fmt::Display for PalletMultisigTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletMultisigTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletMultisigTraits::RuntimeCall => write!(f, "RuntimeCall"),
            PalletMultisigTraits::Currency => write!(f, "Currency"),
            PalletMultisigTraits::DepositBase => write!(f, "DepositBase"),
            PalletMultisigTraits::DepositFactor => write!(f, "DepositFactor"),
            PalletMultisigTraits::MaxSignatories => write!(f, "MaxSignatories"),
            PalletMultisigTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletMultisigConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletMultisigConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "This pallet contains functionality for multi-signature dispatch, a (potentially) stateful operation, allowing multiple signed origins (accounts) to coordinate and dispatch a call from a well-known origin, derivable deterministically from the set of account IDs and the threshold number of accounts from the set that must approve it. In the case that the threshold is just one then this is a stateless operation. This is useful for multisig wallets where cryptographic threshold signatures are not available or desired.",
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME multisig pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-multisig".to_string(),
                version: None,
                alias: "pallet multisig".to_string(),
                default_features: false,
                git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                tag: Some("polkadot-v1.14.0".to_string()),
                branch: None,
            },
            additional_pallets: None,
            additional_deps: None,
        };

        let runtime = PalletRuntimeConfig {
            construct_runtime: PalletConstructRuntimeConfig {
                index: Some(10),
                runtime: (
                    "Multisig".to_string(),
                    "pallet_multisig::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletMultisigTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletMultisigTraits::RuntimeCall.to_string(),
                    "RuntimeCall".to_string(),
                ),
                (
                    PalletMultisigTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletMultisigTraits::DepositBase.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletMultisigTraits::DepositFactor.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletMultisigTraits::MaxSignatories.to_string(),
                    "ConstU32<20>".to_string(),
                ),
                (
                    PalletMultisigTraits::WeightInfo.to_string(),
                    "pallet_multisig::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: None,
            runtime_api_code: None,
        };

        PalletMultisigConfig {
            name: "Pallet multisig".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}
