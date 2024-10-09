use super::super::pallet_index::pallet_index::BOUNTIES;
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
                index: Some(BOUNTIES),
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
            additional_runtime_lib_code: Some(vec![String::from(
                "use pallet_bounties::legacy::BountiesInfo;",
            )]),
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

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for PalletBountiesTraits enum display implementation
    #[test]
    fn test_pallet_bounties_traits_display() {
        assert_eq!(
            PalletBountiesTraits::RuntimeEvent.to_string(),
            "RuntimeEvent"
        );
        assert_eq!(
            PalletBountiesTraits::BountyDepositBase.to_string(),
            "BountyDepositBase"
        );
        assert_eq!(
            PalletBountiesTraits::BountyDepositPayoutDelay.to_string(),
            "BountyDepositPayoutDelay"
        );
        assert_eq!(
            PalletBountiesTraits::BountyUpdatePeriod.to_string(),
            "BountyUpdatePeriod"
        );
        assert_eq!(
            PalletBountiesTraits::CuratorDepositMultiplier.to_string(),
            "CuratorDepositMultiplier"
        );
        assert_eq!(
            PalletBountiesTraits::CuratorDepositMax.to_string(),
            "CuratorDepositMax"
        );
        assert_eq!(
            PalletBountiesTraits::CuratorDepositMin.to_string(),
            "CuratorDepositMin"
        );
        assert_eq!(
            PalletBountiesTraits::BountyValueMinimum.to_string(),
            "BountyValueMinimum"
        );
        assert_eq!(
            PalletBountiesTraits::DataDepositPerByte.to_string(),
            "DataDepositPerByte"
        );
        assert_eq!(
            PalletBountiesTraits::MaximumReasonLength.to_string(),
            "MaximumReasonLength"
        );
        assert_eq!(PalletBountiesTraits::WeightInfo.to_string(), "WeightInfo");
        assert_eq!(
            PalletBountiesTraits::ChildBountyManager.to_string(),
            "ChildBountyManager"
        );
        assert_eq!(PalletBountiesTraits::OnSlash.to_string(), "OnSlash");
    }

    // Test case for PalletBountiesConfig::new() method (assuming PalletBountiesConfig struct exists)
    #[test]
    fn test_pallet_bounties_config_new() {
        let pallet_bounties_config = PalletBountiesConfig::new();

        assert_eq!(pallet_bounties_config.name, "Pallet bounties");

        assert_eq!(
            pallet_bounties_config.metadata.short_description,
            "FRAME bounties pallet"
        );
        assert_eq!(pallet_bounties_config.metadata.size, 10500);
        assert_eq!(
            pallet_bounties_config.metadata.authors[0],
            CommonAuthors::ParityTechnologies
        );
        assert_eq!(
            pallet_bounties_config.metadata.categories.clone().unwrap()[0],
            PalletCategories::Governance
        );
        assert_eq!(
            pallet_bounties_config.metadata.license.clone().unwrap(),
            "Apache-2.0"
        );

        // Ensure description matches
        let expected_description = [
           "The Bounties pallet facilitates the management and payout of rewards for completing specific tasks or objectives, with a curator overseeing the process, and the ability to create child bounties for splitting larger tasks. It works closely with the Treasury pallet."
         ].join("\n");
        assert_eq!(
            pallet_bounties_config.metadata.description,
            expected_description
        );

        // Test dependencies
        assert_eq!(
            pallet_bounties_config.dependencies.pallet.package,
            "pallet-bounties"
        );
        assert_eq!(
            pallet_bounties_config.dependencies.pallet.alias,
            "pallet bounties"
        );
        assert_eq!(
            pallet_bounties_config
                .dependencies
                .pallet
                .git_repo
                .clone()
                .unwrap(),
            "https://github.com/paritytech/polkadot-sdk.git"
        );
        assert_eq!(
            pallet_bounties_config
                .dependencies
                .pallet
                .tag
                .clone()
                .unwrap(),
            "polkadot-v1.14.0"
        );

        // Test runtime configuration
        let runtime_traits = &pallet_bounties_config.runtime.pallet_traits;

        assert_eq!(runtime_traits.get("RuntimeEvent").unwrap(), "RuntimeEvent");
        assert_eq!(
            runtime_traits.get("BountyDepositBase").unwrap(),
            "Get<BalanceOf<Self, I>>"
        );
        assert_eq!(
            runtime_traits.get("BountyDepositPayoutDelay").unwrap(),
            "Get<BlockNumberFor<Self>>"
        );
        assert_eq!(
            runtime_traits.get("BountyUpdatePeriod").unwrap(),
            "Get<BlockNumberFor<Self>>"
        );
        assert_eq!(
            runtime_traits.get("CuratorDepositMultiplier").unwrap(),
            "Get<Permill>"
        );
        assert_eq!(
            runtime_traits.get("CuratorDepositMax").unwrap(),
            "Get<Option<BalanceOf<Self, I>>>"
        );
        assert_eq!(
            runtime_traits.get("CuratorDepositMin").unwrap(),
            "Get<Option<BalanceOf<Self, I>>>"
        );
        assert_eq!(
            runtime_traits.get("BountyValueMinimum").unwrap(),
            "Get<BalanceOf<Self, I>>"
        );
        assert_eq!(
            runtime_traits.get("DataDepositPerByte").unwrap(),
            "Get<BalanceOf<Self, I>>"
        );
        assert_eq!(
            runtime_traits.get("MaximumReasonLength").unwrap(),
            "Get<u32>"
        );
        assert_eq!(
            runtime_traits.get("WeightInfo").unwrap(),
            "pallet_bounties::weights::SubstrateWeight<Runtime>"
        );
        assert_eq!(
            runtime_traits.get("ChildBountyManager").unwrap(),
            "ChildBountyManager<BalanceOf<Self, I>>"
        );
        assert_eq!(
            runtime_traits.get("OnSlash").unwrap(),
            "OnUnbalanced<pallet_treasury::NegativeImbalanceOf<Self, I>>"
        );

        // Test runtime construct configuration
        assert_eq!(
            pallet_bounties_config
                .runtime
                .construct_runtime
                .index
                .unwrap(),
            19
        );
        assert_eq!(
            pallet_bounties_config.runtime.construct_runtime.runtime.0,
            "Bounties"
        );
        assert_eq!(
            pallet_bounties_config.runtime.construct_runtime.runtime.1,
            "pallet_bounties::Pallet<Runtime>"
        );
    }
}
