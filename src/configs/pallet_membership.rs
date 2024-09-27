use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletMembershipTraits {
    RuntimeEvent,
    AddOrigin,
    RemoveOrigin,
    SwapOrigin,
    ResetOrigin,
    PrimeOrigin,
    MembershipInitialized,
    MembershipChanged,
    MaxMembers,
    WeightInfo,
}

impl fmt::Display for PalletMembershipTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletMembershipTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletMembershipTraits::AddOrigin => write!(f, "AddOrigin"),
            PalletMembershipTraits::RemoveOrigin => write!(f, "RemoveOrigin"),
            PalletMembershipTraits::SwapOrigin => write!(f, "SwapOrigin"),
            PalletMembershipTraits::ResetOrigin => write!(f, "ResetOrigin"),
            PalletMembershipTraits::PrimeOrigin => write!(f, "PrimeOrigin"),
            PalletMembershipTraits::MembershipInitialized => write!(f, "MembershipInitialized"),
            PalletMembershipTraits::MembershipChanged => write!(f, "MembershipChanged"),
            PalletMembershipTraits::MaxMembers => write!(f, "MaxMembers"),
            PalletMembershipTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PalletMembershipConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletMembershipConfig {
    pub fn new() -> Self {
        let pallet_description = ["Allows control of membership of a set of AccountIds, useful for managing membership of a collective. A prime member may be set."
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME membership pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),  
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-membership".to_string(),
                version: None,
                alias: "pallet membership".to_string(),
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
                index: Some(13),
                runtime: (
                    "Membership".to_string(),
                    "pallet_membership::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![

            (
                PalletMembershipTraits::RuntimeEvent.to_string(),
                "RuntimeEvent".to_string(),
            ),
            (
                PalletMembershipTraits::AddOrigin.to_string(),
                "EnsureOrigin<Self::RuntimeOrigin>".to_string(),
            ),
            (
                PalletMembershipTraits::RemoveOrigin.to_string(),
                "EnsureOrigin<Self::RuntimeOrigin>".to_string(),
            ),
            (
                PalletMembershipTraits::SwapOrigin.to_string(),
                "EnsureOrigin<Self::RuntimeOrigin>".to_string(),
            ),
            (
                PalletMembershipTraits::ResetOrigin.to_string(),
                "EnsureOrigin<Self::RuntimeOrigin>".to_string(),
            ),
            (
                PalletMembershipTraits::PrimeOrigin.to_string(),
                "EnsureOrigin<Self::RuntimeOrigin>".to_string(),
            ),
            (
                PalletMembershipTraits::MembershipInitialized.to_string(),
                "InitializeMembers<Self::AccountId>".to_string(),
            ),
            (
                PalletMembershipTraits::MembershipChanged.to_string(),
                "ChangeMembers<Self::AccountId>".to_string(),
            ),
            (
                PalletMembershipTraits::MaxMembers.to_string(),
                "Get<u32>".to_string(),
            ),
            (
                PalletMembershipTraits::WeightInfo.to_string(),
                "pallet_membership::weights::SubstrateWeight<Runtime>".to_string(),
            ),


            ]
            .into_iter()
            .collect(),
            genesis_config: None,
            additional_pallet_impl_code: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use pallet_membership::legacy::MembershipInfo;"),
                String::from("use frame_system::EnsureRoot;"),
            ]),
            runtime_api_code: None,
        };

        PalletMembershipConfig {
            name: "Pallet membership".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for PalletMembershipTraits enum display implementation
    #[test]
    fn test_pallet_membership_traits_display() {
        assert_eq!(
            PalletMembershipTraits::RuntimeEvent.to_string(),
            "RuntimeEvent"
        );
        assert_eq!(PalletMembershipTraits::AddOrigin.to_string(), "AddOrigin");
        assert_eq!(PalletMembershipTraits::RemoveOrigin.to_string(), "RemoveOrigin");
        assert_eq!(PalletMembershipTraits::SwapOrigin.to_string(), "SwapOrigin");
        assert_eq!(PalletMembershipTraits::ResetOrigin.to_string(), "ResetOrigin");
        assert_eq!(PalletMembershipTraits::PrimeOrigin.to_string(), "PrimeOrigin");
        assert_eq!(
            PalletMembershipTraits::MembershipInitialized.to_string(),
            "MembershipInitialized"
        );
        assert_eq!(
            PalletMembershipTraits::MembershipChanged.to_string(),
            "MembershipChanged"
        );
        assert_eq!(PalletMembershipTraits::MaxMembers.to_string(), "MaxMembers");
        assert_eq!(PalletMembershipTraits::WeightInfo.to_string(), "WeightInfo");
    }

  // Test case for PalletMembershipConfig::new() method (assuming PalletMembershipConfig struct exists)
  #[test]
  fn test_pallet_membership_config_new() {
      let pallet_membership_config = PalletMembershipConfig::new();

      // Test the name
      assert_eq!(pallet_membership_config.name, "Pallet membership");

      // Test metadata
      assert_eq!(
          pallet_membership_config.metadata.short_description,
          "FRAME membership pallet"
      );
      assert_eq!(pallet_membership_config.metadata.size, 10500);  // Assuming a size value
      assert_eq!(
          pallet_membership_config.metadata.authors[0],
          CommonAuthors::ParityTechnologies
      );
      assert_eq!(
          pallet_membership_config.metadata.categories.clone().unwrap()[0],
          PalletCategories::Governance
      );
      assert_eq!(
          pallet_membership_config.metadata.license.clone().unwrap(),
          "Apache-2.0"
      );
      // Ensure description matches
      let expected_description = [
        "Allows control of membership of a set of AccountIds, useful for managing membership of a collective. A prime member may be set."
              ].join("\n");
    assert_eq!(
        pallet_membership_config.metadata.description,
        expected_description
    );

    // Test dependencies
    assert_eq!(
        pallet_membership_config.dependencies.pallet.package,
        "pallet-membership"
    );
    assert_eq!(
        pallet_membership_config.dependencies.pallet.alias,
        "pallet membership"
    );
    assert_eq!(
        pallet_membership_config
            .dependencies
            .pallet
            .git_repo
            .clone()
            .unwrap(),
        "https://github.com/paritytech/polkadot-sdk.git"
    );
    assert_eq!(
        pallet_membership_config
            .dependencies
            .pallet
            .tag
            .clone()
            .unwrap(),
        "polkadot-v1.14.0"
    );

    // Test runtime configuration
    let runtime_traits = &pallet_membership_config.runtime.pallet_traits;

    assert_eq!(runtime_traits.get("RuntimeEvent").unwrap(), "RuntimeEvent");
    assert_eq!(runtime_traits.get("AddOrigin").unwrap(), "EnsureOrigin<Self::RuntimeOrigin>");
    assert_eq!(runtime_traits.get("RemoveOrigin").unwrap(), "EnsureOrigin<Self::RuntimeOrigin>");
    assert_eq!(runtime_traits.get("SwapOrigin").unwrap(), "EnsureOrigin<Self::RuntimeOrigin>");
    assert_eq!(runtime_traits.get("ResetOrigin").unwrap(), "EnsureOrigin<Self::RuntimeOrigin>");
    assert_eq!(runtime_traits.get("PrimeOrigin").unwrap(), "EnsureOrigin<Self::RuntimeOrigin>");
    assert_eq!(runtime_traits.get("MembershipInitialized").unwrap(), "InitializeMembers<Self::AccountId>");
    assert_eq!(runtime_traits.get("MembershipChanged").unwrap(), "ChangeMembers<Self::AccountId>");
    assert_eq!(runtime_traits.get("MaxMembers").unwrap(), "Get<u32>");
    assert_eq!(runtime_traits.get("WeightInfo").unwrap(), "pallet_membership::weights::SubstrateWeight<Runtime>");

        // Test runtime construct configuration
        assert_eq!(
            pallet_membership_config
                .runtime
                .construct_runtime
                .index
                .unwrap(),
            13
        );
        assert_eq!(
            pallet_membership_config.runtime.construct_runtime.runtime.0,
            "Membership"
        );
        assert_eq!(
            pallet_membership_config.runtime.construct_runtime.runtime.1,
            "pallet_membership::Pallet<Runtime>"
        );
    }
}
