use super::super::pallet_index::DEMOCRACY;
use super::super::types::*;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletDemocracyTraits {
    WeightInfo,
    RuntimeEvent,
    Scheduler,
    Preimages,
    Currency,
    EnactmentPeriod,
    LaunchPeriod,
    VotingPeriod,
    VoteLockingPeriod,
    MinimumDeposit,
    InstantAllowed,
    FastTrackVotingPeriod,
    CooloffPeriod,
    MaxVotes,
    MaxProposals,
    MaxDeposits,
    MaxBlacklisted,
    ExternalOrigin,
    ExternalMajorityOrigin,
    ExternalDefaultOrigin,
    SubmitOrigin,
    FastTrackOrigin,
    InstantOrigin,
    CancellationOrigin,
    BlacklistOrigin,
    CancelProposalOrigin,
    VetoOrigin,
    PalletsOrigin,
    Slash,
}

impl fmt::Display for PalletDemocracyTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletDemocracyTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletDemocracyTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletDemocracyTraits::Scheduler => write!(f, "Scheduler"),
            PalletDemocracyTraits::Preimages => write!(f, "Preimages"),
            PalletDemocracyTraits::Currency => write!(f, "Currency"),
            PalletDemocracyTraits::EnactmentPeriod => write!(f, "EnactmentPeriod"),
            PalletDemocracyTraits::LaunchPeriod => write!(f, "LaunchPeriod"),
            PalletDemocracyTraits::VotingPeriod => write!(f, "VotingPeriod"),
            PalletDemocracyTraits::VoteLockingPeriod => write!(f, "VoteLockingPeriod"),
            PalletDemocracyTraits::MinimumDeposit => write!(f, "MinimumDeposit"),
            PalletDemocracyTraits::InstantAllowed => write!(f, "InstantAllowed"),
            PalletDemocracyTraits::FastTrackVotingPeriod => write!(f, "FastTrackVotingPeriod"),
            PalletDemocracyTraits::CooloffPeriod => write!(f, "CooloffPeriod"),
            PalletDemocracyTraits::MaxVotes => write!(f, "MaxVotes"),
            PalletDemocracyTraits::MaxProposals => write!(f, "MaxProposals"),
            PalletDemocracyTraits::MaxDeposits => write!(f, "MaxDeposits"),
            PalletDemocracyTraits::MaxBlacklisted => write!(f, "MaxBlacklisted"),
            PalletDemocracyTraits::ExternalOrigin => write!(f, "ExternalOrigin"),
            PalletDemocracyTraits::ExternalMajorityOrigin => write!(f, "ExternalMajorityOrigin"),
            PalletDemocracyTraits::ExternalDefaultOrigin => write!(f, "ExternalDefaultOrigin"),
            PalletDemocracyTraits::SubmitOrigin => write!(f, "SubmitOrigin"),
            PalletDemocracyTraits::FastTrackOrigin => write!(f, "FastTrackOrigin"),
            PalletDemocracyTraits::InstantOrigin => write!(f, "InstantOrigin"),
            PalletDemocracyTraits::CancellationOrigin => write!(f, "CancellationOrigin"),
            PalletDemocracyTraits::BlacklistOrigin => write!(f, "BlacklistOrigin"),
            PalletDemocracyTraits::CancelProposalOrigin => write!(f, "CancelProposalOrigin"),
            PalletDemocracyTraits::VetoOrigin => write!(f, "VetoOrigin"),
            PalletDemocracyTraits::PalletsOrigin => write!(f, "PalletsOrigin"),
            PalletDemocracyTraits::Slash => write!(f, "Slash"),
        }
    }
}

pub struct PalletDemocracyConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletDemocracyConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// TODO Configurations needs to updated rationally by keeping every pallet use-case in mind.
impl PalletDemocracyConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Democracy pallet enables stakeholder voting on referenda, managing proposals from public and external queues, and utilizing time-lock voting with conviction-based vote power scaling."
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME democracy pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            is_essential: false,
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Runtime]),
        };
        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-democracy".to_string(),
                version: None,
                alias: "pallet democracy".to_string(),
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
                index: Some(DEMOCRACY),
                runtime: (
                    "Democracy".to_string(),
                    "pallet_democracy::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                    (
                        PalletDemocracyTraits::WeightInfo.to_string(),
                        "pallet_democracy::weights::SubstrateWeight<Runtime>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::RuntimeEvent.to_string(),
                        "RuntimeEvent".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::Scheduler.to_string(),
                        "Scheduler".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::Preimages.to_string(),
                        "()".to_string(), // don't have preimage pallet
                    ),
                    (
                        PalletDemocracyTraits::Currency.to_string(),
                        "Balances".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::EnactmentPeriod.to_string(),
                        "Period".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::LaunchPeriod.to_string(),
                        "Period".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::VotingPeriod.to_string(),
                        "Period".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::VoteLockingPeriod.to_string(),
                        "ConstU32<{ 2 * MINUTES }>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::MinimumDeposit.to_string(),
                        "MinimumDeposit".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::InstantAllowed.to_string(),
                        "ConstBool<true>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::FastTrackVotingPeriod.to_string(),
                        "FastTrackVotingPeriod".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::CooloffPeriod.to_string(),
                        "CooloffPeriod".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::MaxVotes.to_string(),
                        "MaxAll".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::MaxProposals.to_string(),
                        "MaxAll".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::MaxDeposits.to_string(),
                        "MaxAll".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::MaxBlacklisted.to_string(),
                        "MaxAll".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::ExternalOrigin.to_string(),
                        "pallet_collective::EnsureProportionAtLeast<AccountId, (), 1, 2>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::ExternalMajorityOrigin.to_string(),
                        "pallet_collective::EnsureProportionAtLeast<AccountId, (), 1, 2>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::ExternalDefaultOrigin.to_string(),
                        "pallet_collective::EnsureProportionAtLeast<AccountId, (), 1, 1>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::SubmitOrigin.to_string(),
                        "EnsureSigned<AccountId>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::FastTrackOrigin.to_string(),
                        "pallet_collective::EnsureProportionAtLeast<AccountId, (), 2, 3>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::InstantOrigin.to_string(),
                        "pallet_collective::EnsureProportionAtLeast<AccountId, (), 1, 1>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::CancellationOrigin.to_string(),
                        "EitherOfDiverse<EnsureRoot<AccountId>, pallet_collective::EnsureProportionAtLeast<AccountId, (), 2, 3>>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::BlacklistOrigin.to_string(),
                        "EnsureRoot<AccountId>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::CancelProposalOrigin.to_string(),
                        "EitherOfDiverse<EnsureRoot<AccountId>, pallet_collective::EnsureProportionAtLeast<AccountId, (), 1, 1>>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::VetoOrigin.to_string(),
                        "pallet_collective::EnsureMember<AccountId, ()>".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::PalletsOrigin.to_string(),
                        "OriginCaller".to_string(),
                    ),
                    (
                        PalletDemocracyTraits::Slash.to_string(),
                        "()".to_string(),
                    ),
                ]
                .into_iter()
                .collect(),
                additional_pallet_impl_code: Some(get_additional_implementation_code()),
                genesis_config: None,
                additional_chain_spec_code: None,
                additional_runtime_lib_code: Some(vec![String::from(
                    "use frame_support::traits::EitherOfDiverse;",
                )]),
                runtime_api_code: None,
            };
        PalletDemocracyConfig {
            name: "Pallet Democracy".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}
fn get_additional_implementation_code() -> String {
    "
    parameter_types! { 
            pub const MinimumDeposit: u128 = 100 * 1_000_000_000_000;
            pub const Period: u32 = 5 * MINUTES;
            pub const FastTrackVotingPeriod: u32 = MINUTES / 2;
            pub const CooloffPeriod: u32 = 2 * MINUTES;
            pub const MaxAll: u32 = 128;
        }
    "
    .to_string()
}
