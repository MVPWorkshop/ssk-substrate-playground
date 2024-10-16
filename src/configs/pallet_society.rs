use super::super::pallet_index::SOCIETY;
use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletSocietyTraits {
    RuntimeEvent,
    PalletId,
    Currency,
    Randomness,
    GraceStrikes,
    PeriodSpend,
    VotingPeriod,
    ClaimPeriod,
    MaxLockDuration,
    FounderSetOrigin,
    ChallengePeriod,
    MaxPayouts,
    MaxBids,
    WeightInfo,
}

impl fmt::Display for PalletSocietyTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletSocietyTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletSocietyTraits::PalletId => write!(f, "PalletId"),
            PalletSocietyTraits::Currency => write!(f, "Currency"),
            PalletSocietyTraits::Randomness => write!(f, "Randomness"),
            PalletSocietyTraits::GraceStrikes => write!(f, "GraceStrikes"),
            PalletSocietyTraits::PeriodSpend => write!(f, "PeriodSpend"),
            PalletSocietyTraits::VotingPeriod => write!(f, "VotingPeriod"),
            PalletSocietyTraits::ClaimPeriod => write!(f, "ClaimPeriod"),
            PalletSocietyTraits::MaxLockDuration => write!(f, "MaxLockDuration"),
            PalletSocietyTraits::FounderSetOrigin => write!(f, "FounderSetOrigin"),
            PalletSocietyTraits::ChallengePeriod => write!(f, "ChallengePeriod"),
            PalletSocietyTraits::MaxPayouts => write!(f, "MaxPayouts"),
            PalletSocietyTraits::MaxBids => write!(f, "MaxBids"),
            PalletSocietyTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PalletSocietyConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletSocietyConfig {
    fn default() -> Self {
        PalletSocietyConfig::new()
    }
}

impl PalletSocietyConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Society module is an economic game which incentivizes users to participate and maintain a membership society.",
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME society pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Runtime]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-society".to_string(),
                version: None,
                alias: "pallet society".to_string(),
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
                index: Some(SOCIETY),
                runtime: (
                    "Society".to_string(),
                    "pallet_society::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletSocietyTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletSocietyTraits::PalletId.to_string(),
                    "SocietyPalletId".to_string(),
                ),
                (
                    PalletSocietyTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletSocietyTraits::Randomness.to_string(),
                    "RandomnessCollectiveFlip".to_string(),
                ),
                (
                    PalletSocietyTraits::GraceStrikes.to_string(),
                    "GraceStrikes".to_string(),
                ),
                (
                    PalletSocietyTraits::PeriodSpend.to_string(),
                    "PeriodSpend".to_string(),
                ),
                (
                    PalletSocietyTraits::VotingPeriod.to_string(),
                    "SocietyVotingPeriod".to_string(),
                ),
                (
                    PalletSocietyTraits::ClaimPeriod.to_string(),
                    "ClaimPeriod".to_string(),
                ),
                (
                    PalletSocietyTraits::MaxLockDuration.to_string(),
                    "MaxLockDuration".to_string(),
                ),
                (
                    PalletSocietyTraits::FounderSetOrigin.to_string(),
                    "EnsureRoot<Self::AccountId>".to_string(),
                ),
                (
                    PalletSocietyTraits::ChallengePeriod.to_string(),
                    "ChallengePeriod".to_string(),
                ),
                (
                    PalletSocietyTraits::MaxPayouts.to_string(),
                    "MaxPayouts".to_string(),
                ),
                (
                    PalletSocietyTraits::MaxBids.to_string(),
                    "MaxBids".to_string(),
                ),
                (
                    PalletSocietyTraits::WeightInfo.to_string(),
                    "pallet_society::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: None,
            runtime_api_code: None,
        };

        PalletSocietyConfig {
            name: "Pallet society".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
    pub const GraceStrikes: u32 = 10;
    pub const SocietyVotingPeriod: BlockNumber = 80 * HOURS;
    pub const ClaimPeriod: BlockNumber = 80 * HOURS;
    pub const PeriodSpend: Balance = 500 * DOLLARS;
    pub const MaxLockDuration: BlockNumber = 36 * 30 * DAYS;
    pub const ChallengePeriod: BlockNumber = 7 * DAYS;
    pub const MaxPayouts: u32 = 10;
    pub const MaxBids: u32 = 10;
    pub const SocietyPalletId: PalletId = PalletId(*b\"py/socie\");
}
"
    .to_string()
}
