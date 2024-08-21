use super::super::types::*;
use chrono::Utc;

#[derive(Debug, Clone, Copy)]
pub enum PalletUtilityTraits {
    RuntimeEvent,
    RuntimeCall,
    PalletsOrigin,
    WeightInfo,
}

//
// impl fmt::Display for PalletUtilityTraits {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PalletUtilityTraits::Event => write!(f, "Event"),
//             PalletUtilityTraits::Call => write!(f, "Call"),
//             PalletUtilityTraits::WeightInfo => write!(f, "WeightInfo"),
//         }
//     }
// }
#[derive(Debug, Clone)]
pub struct PalletUtilityConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletUtilityConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "A stateless module with helpers for dispatch management which does no re-authentication.",
            "This module contains two basic pieces of functionality:",
            "- Batch dispatch: A stateless operation, allowing any origin to execute multiple calls in a single dispatch. This can be useful to amalgamate proposals, combining set_code with corresponding set_storages, for efficient multiple payouts with just a single signature verify, or in combination with one of the other two dispatch functionality.",
            "- Pseudonymal dispatch: A stateless operation, allowing a signed origin to execute a call from an alternative signed origin. Each account has 2 * 2**16 possible \"pseudonyms\" (alternative account IDs) and these can be stacked. This can be useful as a key management tool, where you need multiple distinct accounts (e.g. as controllers for many staking accounts), but where it's perfectly fine to have each of them controlled by the same underlying keypair. Derivative accounts are, for the purposes of proxy filtering considered exactly the same as the origin and are thus hampered with the origin's filters.",
            "Since proxy filters are respected in all dispatches of this module, it should never need to be filtered by any proxy."
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME utilities pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Runtime]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-utility".to_string(),
                version: None,
                alias: "pallet-utility".to_string(),
                default_features: None,
                git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                tag: Some("polkadot-v1.14.0".to_string()),
                branch: None,
            },
            additional_pallets: None,
            additional_deps: None,
        };

        let runtime = PalletRuntimeConfig {
            construct_runtime: PalletConstructRuntimeConfig {
                index: None,
                runtime: ("Utility".to_string(), "pallet_utility::Pallet<Runtime>".to_string()),
            },
            pallet_traits: vec![
                ("RuntimeEvent".to_string(), "RuntimeEvent".to_string()),
                ("RuntimeCall".to_string(), "RuntimeCall".to_string()),
                ("PalletsOrigin".to_string(), "OriginCaller".to_string()),
                (
                    "WeightInfo".to_string(),
                    "pallet_utility::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: None,
        };

        PalletUtilityConfig {
            name: "Pallet utility".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}
