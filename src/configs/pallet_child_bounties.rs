use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletChildBountiesTraits {
    RuntimeEvent,
    MaxActiveChildBountyCount,
    ChildBountyValueMinimum,
    WeightInfo,
}

impl fmt::Display for PalletChildBountiesTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletChildBountiesTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletChildBountiesTraits::MaxActiveChildBountyCount => write!(f, "MaxActiveChildBountyCount"),
            PalletChildBountiesTraits::ChildBountyValueMinimum => write!(f, "ChildBountyValueMinimum"),
            PalletChildBountiesTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}

pub struct PalletChildBountiesConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletChildBountiesConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "A pallet for managing child bounties within the governance system.",
            "The Child Bounties pallet allows for the creation and management of child bounties tied to a parent bounty, including: child bounty creation, value management, and payout distribution."
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME child bounties pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500, 
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-child-bounties".to_string(),
                version: None,
                alias: "pallet child bounties".to_string(),
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
                index: Some(16),
                runtime: (
                    "ChildBounties".to_string(),
                    "pallet_child_bounties::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletChildBountiesTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletChildBountiesTraits::MaxActiveChildBountyCount.to_string(),
                    "ConstU32<5>".to_string(),  
                ),
                (
                    PalletChildBountiesTraits::ChildBountyValueMinimum.to_string(),
                    "ConstU128<{ 500 * 1000 }>".to_string(),
                ),
                (
                    PalletChildBountiesTraits::WeightInfo.to_string(),
                    "pallet_child_bounties::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: None,
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use pallet_child_bounties::legacy::ChildBountiesInfo;"),
                String::from("use frame_system::EnsureRoot;"),
            ]),
            runtime_api_code: None,
        };
        
        PalletChildBountiesConfig {
            name: "Pallet child bounties".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}