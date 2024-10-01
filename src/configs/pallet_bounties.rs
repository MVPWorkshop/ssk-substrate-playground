use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletBountiesTraits {
    RuntimeEvent,
    BountyDepositBase,
    BountyDepositPayoutDelay,
    BountyUpdatePeriod,
    CuratorDepositMultiplier,
    CuratorDepositMax,
    CuratorDepositMin,
    BountyValueMinimum,
    DataDepositPerByte,
    MaximumReasonLength,
    WeightInfo,
    ChildBountyManager,
    OnSlash,
}

impl fmt::Display for PalletBountiesTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletBountiesTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletBountiesTraits::BountyDepositBase => write!(f, "BountyDepositBase"),
            PalletBountiesTraits::BountyDepositPayoutDelay => write!(f, "BountyDepositPayoutDelay"),
            PalletBountiesTraits::BountyUpdatePeriod => write!(f, "BountyUpdatePeriod"),
            PalletBountiesTraits::CuratorDepositMultiplier => write!(f, "CuratorDepositMultiplier"),
            PalletBountiesTraits::CuratorDepositMax => write!(f, "CuratorDepositMax"),
            PalletBountiesTraits::CuratorDepositMin => write!(f, "CuratorDepositMin"),
            PalletBountiesTraits::BountyValueMinimum => write!(f, "BountyValueMinimum"),
            PalletBountiesTraits::DataDepositPerByte => write!(f, "DataDepositPerByte"),
            PalletBountiesTraits::MaximumReasonLength => write!(f, "MaximumReasonLength"),
            PalletBountiesTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletBountiesTraits::ChildBountyManager => write!(f, "ChildBountyManager"),
            PalletBountiesTraits::OnSlash => write!(f, "OnSlash"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletBountiesConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletBountiesConfig {
    pub fn new() -> Self {
        let pallet_description = ["The Bounties pallet facilitates the management and payout of rewards for completing specific tasks or objectives, with a curator overseeing the process, and the ability to create child bounties for splitting larger tasks. It works closely with the Treasury pallet."
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME bounties pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),  
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-bounties".to_string(),
                version: None,
                alias: "pallet bounties".to_string(),
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
                index: Some(15),
                runtime: (
                    "Bounties".to_string(),
                    "pallet_bounties::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                    (
                        PalletBountiesTraits::RuntimeEvent.to_string(),
                        "RuntimeEvent".to_string(),
                    ),
                    (
                        PalletBountiesTraits::BountyDepositBase.to_string(),
                        "Get<BalanceOf<Self, I>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::BountyDepositPayoutDelay.to_string(),
                        "Get<BlockNumberFor<Self>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::BountyUpdatePeriod.to_string(),
                        "Get<BlockNumberFor<Self>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::CuratorDepositMultiplier.to_string(),
                        "Get<Permill>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::CuratorDepositMax.to_string(),
                        "Get<Option<BalanceOf<Self, I>>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::CuratorDepositMin.to_string(),
                        "Get<Option<BalanceOf<Self, I>>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::BountyValueMinimum.to_string(),
                        "Get<BalanceOf<Self, I>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::DataDepositPerByte.to_string(),
                        "Get<BalanceOf<Self, I>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::MaximumReasonLength.to_string(),
                        "Get<u32>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::WeightInfo.to_string(),
                        "pallet_bounties::weights::SubstrateWeight<Runtime>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::ChildBountyManager.to_string(),
                        "ChildBountyManager<BalanceOf<Self, I>>".to_string(),
                    ),
                    (
                        PalletBountiesTraits::OnSlash.to_string(),
                        "OnUnbalanced<pallet_treasury::NegativeImbalanceOf<Self, I>>".to_string(),
                    ),
                
            ]
            .into_iter()
            .collect(),
            genesis_config: None,
            additional_pallet_impl_code: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use pallet_bounties::legacy::BountiesInfo;"),
                String::from("use frame_system::EnsureRoot;"),
            ]),
            runtime_api_code: None,
        };

        PalletBountiesConfig {
            name: "Pallet bounties".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}