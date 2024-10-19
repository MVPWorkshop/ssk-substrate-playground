use super::super::pallet_index::MEMBERSHIP;
use super::super::types::*;
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

impl Default for PalletMembershipConfig {
    fn default() -> Self {
        PalletMembershipConfig::new()
    }
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
            is_essential: false,
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
            required_pallets: None,
        };
        let runtime = PalletRuntimeConfig {
            construct_runtime: PalletConstructRuntimeConfig {
                index: Some(MEMBERSHIP),
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
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletMembershipTraits::RemoveOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletMembershipTraits::SwapOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletMembershipTraits::ResetOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletMembershipTraits::PrimeOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletMembershipTraits::MembershipInitialized.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletMembershipTraits::MembershipChanged.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletMembershipTraits::MaxMembers.to_string(),
                    "TechnicalMaxMembers".to_string(),
                ),
                (
                    PalletMembershipTraits::WeightInfo.to_string(),
                    "pallet_membership::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            genesis_config: None,
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            additional_chain_spec_code: None,
            additional_runtime_lib_code: None,
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

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
	pub const TechnicalMaxMembers: u32 = 100;
}
"
    .to_string()
}
