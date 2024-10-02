use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletCollectiveTraits {
    RuntimeOrigin,
    Proposal,
    RuntimeEvent,
    MotionDuration,
    MaxProposals,
    MaxMembers,
    DefaultVote,
    WeightInfo,
    SetMembersOrigin,
    MaxProposalWeight,
    DisapproveOrigin,
    KillOrigin,
    Consideration,
}
impl fmt::Display for PalletCollectiveTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletCollectiveTraits::RuntimeOrigin => write!(f, "RuntimeOrigin"),
            PalletCollectiveTraits::Proposal => write!(f, "Proposal"),
            PalletCollectiveTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletCollectiveTraits::MotionDuration => write!(f, "MotionDuration"),
            PalletCollectiveTraits::MaxProposals => write!(f, "MaxProposals"),
            PalletCollectiveTraits::MaxMembers => write!(f, "MaxMembers"),
            PalletCollectiveTraits::DefaultVote => write!(f, "DefaultVote"),
            PalletCollectiveTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletCollectiveTraits::SetMembersOrigin => write!(f, "SetMembersOrigin"),
            PalletCollectiveTraits::MaxProposalWeight => write!(f, "MaxProposalWeight"),
            PalletCollectiveTraits::DisapproveOrigin => write!(f, "DisapproveOrigin"),
            PalletCollectiveTraits::KillOrigin => write!(f, "KillOrigin"),
            PalletCollectiveTraits::Consideration => write!(f, "Consideration"),
        }
    }
}

pub struct PalletCollectiveConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletCollectiveConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Collective pallet allows a group of account IDs to make collective decisions through voting on proposals. Voting happens over a defined period, and a  member can influence the default vote. Proposals are executed once the required number of approvals is reached. ",
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME collective pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500, 
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),
        };
        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-collective".to_string(),
                version: None,
                alias: "pallet collective".to_string(),
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
                    "Collective".to_string(),
                    "pallet_collective::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletCollectiveTraits::RuntimeOrigin.to_string(),
                    "From<RawOrigin<Self::AccountId, I>>".to_string(),
                ),
                (
                    PalletCollectiveTraits::Proposal.to_string(),
                    "Dispatchable<RuntimeOrigin = <Self as Config<I>>::RuntimeOrigin>".to_string(),
                ),
                (
                    PalletCollectiveTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletCollectiveTraits::MotionDuration.to_string(),
                    "Get<BlockNumberFor<Self>>".to_string(),
                ),
                (
                    PalletCollectiveTraits::MaxProposals.to_string(),
                    "Get<ProposalIndex>".to_string(),
                ),
                (
                    PalletCollectiveTraits::MaxMembers.to_string(),
                    "Get<MemberCount>".to_string(),
                ),
                (
                    PalletCollectiveTraits::DefaultVote.to_string(),
                    "DefaultVote".to_string(),
                ),
                (
                    PalletCollectiveTraits::WeightInfo.to_string(),
                    "allet_collective::weights::SubstrateWeight<Runtime>".to_string(),
                ),
                (
                    PalletCollectiveTraits::SetMembersOrigin.to_string(),
                    "EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>".to_string(),
                ),
                (
                    PalletCollectiveTraits::MaxProposalWeight.to_string(),
                    "Get<Weight>".to_string(),
                ),
                (
                    PalletCollectiveTraits::DisapproveOrigin.to_string(),
                    "EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>".to_string(),
                ),
                (
                    PalletCollectiveTraits::KillOrigin.to_string(),
                    "EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>".to_string(),
                ),
                (
                    PalletCollectiveTraits::Consideration.to_string(),
                    "MaybeConsideration<Self::AccountId, u32>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: None,
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use pallet_collective::legacy::CollectiveInfo;"),
                String::from("use frame_system::EnsureRoot;"),
            ]),
            runtime_api_code: None,
        };
        
        PalletCollectiveConfig {
            name: "Pallet collective".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}