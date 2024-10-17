use super::super::pallet_index::STAKING;
use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletStakingTraits {
    RuntimeEvent,
    Currency,
    CurrencyBalance,
    UnixTime,
    CurrencyToVote,
    RewardRemainder,
    Slash,
    Reward,
    SessionsPerEra,
    BondingDuration,
    SlashDeferDuration,
    AdminOrigin,
    SessionInterface,
    EraPayout,
    NextNewSession,
    MaxExposurePageSize,
    ElectionProvider,
    GenesisElectionProvider,
    VoterList,
    NominationsQuota,
    TargetList,
    MaxUnlockingChunks,
    MaxControllersInDeprecationBatch,
    HistoryDepth,
    EventListeners,
    BenchmarkingConfig,
    DisablingStrategy,
    WeightInfo,
}

impl fmt::Display for PalletStakingTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletStakingTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletStakingTraits::Currency => write!(f, "Currency"),
            PalletStakingTraits::CurrencyBalance => write!(f, "CurrencyBalance"),
            PalletStakingTraits::UnixTime => write!(f, "UnixTime"),
            PalletStakingTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletStakingTraits::CurrencyToVote => write!(f, "CurrencyToVote"),
            PalletStakingTraits::RewardRemainder => write!(f, "RewardRemainder"),
            PalletStakingTraits::Slash => write!(f, "Slash"),
            PalletStakingTraits::Reward => write!(f, "Reward"),
            PalletStakingTraits::SessionsPerEra => write!(f, "SessionsPerEra"),
            PalletStakingTraits::BondingDuration => write!(f, "BondingDuration"),
            PalletStakingTraits::SlashDeferDuration => write!(f, "SlashDeferDuration"),
            PalletStakingTraits::AdminOrigin => write!(f, "AdminOrigin"),
            PalletStakingTraits::SessionInterface => write!(f, "SessionInterface"),
            PalletStakingTraits::EraPayout => write!(f, "EraPayout"),
            PalletStakingTraits::NextNewSession => write!(f, "NextNewSession"),
            PalletStakingTraits::MaxExposurePageSize => write!(f, "MaxExposurePageSize"),
            PalletStakingTraits::ElectionProvider => write!(f, "ElectionProvider"),
            PalletStakingTraits::GenesisElectionProvider => write!(f, "GenesisElectionProvider"),
            PalletStakingTraits::VoterList => write!(f, "VoterList"),
            PalletStakingTraits::NominationsQuota => write!(f, "NominationsQuota"),
            PalletStakingTraits::TargetList => write!(f, "TargetList"),
            PalletStakingTraits::MaxUnlockingChunks => write!(f, "MaxUnlockingChunks"),
            PalletStakingTraits::MaxControllersInDeprecationBatch => {
                write!(f, "MaxControllersInDeprecationBatch")
            }
            PalletStakingTraits::HistoryDepth => write!(f, "HistoryDepth"),
            PalletStakingTraits::EventListeners => write!(f, "EventListeners"),
            PalletStakingTraits::BenchmarkingConfig => write!(f, "BenchmarkingConfig"),
            PalletStakingTraits::DisablingStrategy => write!(f, "DisablingStrategy"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletStakingConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletStakingConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Staking pallet is used to manage funds at stake by network maintainers.",
            "The Staking pallet is the means by which a set of network maintainers (known as _authorities_ in some contexts and _validators_ in others) are chosen based upon those who voluntarily place funds under deposit. Under deposit, those funds are rewarded under normal operation but are held at pain of _slash_ (expropriation) should the staked maintainer be found not to be discharging its duties properly.",
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME staking pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-staking".to_string(),
                version: None,
                alias: "pallet staking".to_string(),
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
                index: Some(STAKING),
                runtime: (
                    "Staking".to_string(),
                    "pallet_staking::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletStakingTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletStakingTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletStakingTraits::UnixTime.to_string(),
                    "Timestamp".to_string(),
                ),
                (
                    PalletStakingTraits::CurrencyToVote.to_string(),
                    "sp_staking::currency_to_vote::U128CurrencyToVote".to_string(),
                ),
                (
                    PalletStakingTraits::RewardRemainder.to_string(),
                    "Treasury".to_string(),
                ),
                (
                    PalletStakingTraits::Slash.to_string(),
                    "Treasury".to_string(),
                ),
                (PalletStakingTraits::Reward.to_string(), "()".to_string()),
                (
                    PalletStakingTraits::SessionsPerEra.to_string(),
                    "SessionsPerEra".to_string(),
                ),
                (
                    PalletStakingTraits::BondingDuration.to_string(),
                    "BondingDuration".to_string(),
                ),
                (
                    PalletStakingTraits::SlashDeferDuration.to_string(),
                    "SlashDeferDuration".to_string(),
                ),
                (
                    PalletStakingTraits::AdminOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletStakingTraits::SessionInterface.to_string(),
                    "Self".to_string(),
                ),
                (
                    PalletStakingTraits::EraPayout.to_string(),
                    "pallet_staking::ConvertCurve<RewardCurve>".to_string(),
                ),
                (
                    PalletStakingTraits::NextNewSession.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletStakingTraits::MaxExposurePageSize.to_string(),
                    "ConstU32<256>".to_string(),
                ),
                (
                    PalletStakingTraits::ElectionProvider.to_string(),
                    "ElectionProviderMultiPhase".to_string(),
                ),
                (
                    PalletStakingTraits::GenesisElectionProvider.to_string(),
                    "onchain::OnChainExecution<OnChainSeqPhragmen>".to_string(),
                ),
                (
                    PalletStakingTraits::VoterList.to_string(),
                    "VoterList".to_string(),
                ),
                (
                    PalletStakingTraits::NominationsQuota.to_string(),
                    "pallet_staking::FixedNominationsQuota<MAX_QUOTA_NOMINATIONS>".to_string(),
                ),
                (
                    PalletStakingTraits::TargetList.to_string(),
                    "pallet_staking::UseValidatorsMap<Self>".to_string(),
                ),
                (
                    PalletStakingTraits::MaxUnlockingChunks.to_string(),
                    "ConstU32<32>".to_string(),
                ),
                (
                    PalletStakingTraits::MaxControllersInDeprecationBatch.to_string(),
                    "MaxControllersInDeprecationBatch".to_string(),
                ),
                (
                    PalletStakingTraits::HistoryDepth.to_string(),
                    "HistoryDepth".to_string(),
                ),
                (
                    PalletStakingTraits::EventListeners.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletStakingTraits::WeightInfo.to_string(),
                    "pallet_staking::weights::SubstrateWeight<Runtime>".to_string(),
                ),
                (
                    PalletStakingTraits::BenchmarkingConfig.to_string(),
                    "StakingBenchmarkingConfig".to_string(),
                ),
                (
                    PalletStakingTraits::DisablingStrategy.to_string(),
                    "pallet_staking::UpToLimitDisablingStrategy".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![String::from(
                "use sp_runtime::curve::PiecewiseLinear;",
            )]),
            runtime_api_code: Some(get_runtime_api_code()),
        };

        PalletStakingConfig {
            name: "Pallet staking".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
use frame_election_provider_support::{
    bounds::{ElectionBounds, ElectionBoundsBuilder},
    onchain, BalancingConfig, ElectionDataProvider, SequentialPhragmen, VoteWeight,
};
pallet_staking_reward_curve::build! {
    const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_025_000,
        max_inflation: 0_100_000,
        ideal_stake: 0_500_000,
        falloff: 0_050_000,
        max_piece_count: 40,
        test_precision: 0_005_000,
    );
}
parameter_types! {
    pub const SessionsPerEra: sp_staking::SessionIndex = 6;
    pub const BondingDuration: sp_staking::EraIndex = 24 * 28;
    pub const SlashDeferDuration: sp_staking::EraIndex = 24 * 7; // 1/4 the bonding duration.
    pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
    pub const MaxNominators: u32 = 64;
    pub const MaxControllersInDeprecationBatch: u32 = 5900;
    pub OffchainRepeat: BlockNumber = 5;
    pub HistoryDepth: u32 = 84;
}
/// Upper limit on the number of NPOS nominations.
const MAX_QUOTA_NOMINATIONS: u32 = 16;
pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
    type MaxNominators = ConstU32<1000>;
    type MaxValidators = ConstU32<1000>;
}
impl pallet_staking::Config for Runtime {
    type Currency = Balances;
    type CurrencyBalance = Balance;
    type UnixTime = Timestamp;
    type CurrencyToVote = sp_staking::currency_to_vote::U128CurrencyToVote;
    type RewardRemainder = Treasury;
    type RuntimeEvent = RuntimeEvent;
    type Slash = Treasury; // send the slashed funds to the treasury.
    type Reward = ();
    type SessionsPerEra = SessionsPerEra;
    type BondingDuration = BondingDuration;
    type SlashDeferDuration = SlashDeferDuration;
    /// A super-majority of the council can cancel the slash.
    type AdminOrigin = EnsureRoot<AccountId>;
    type SessionInterface = Self;
    type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
    type NextNewSession = ();
    type MaxExposurePageSize = ConstU32<256>;
    type ElectionProvider = ElectionProviderMultiPhase;
    type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
    type VoterList = VoterList;
    type NominationsQuota = pallet_staking::FixedNominationsQuota<MAX_QUOTA_NOMINATIONS>;
    // This a placeholder, to be introduced in the next PR as an instance of bags-list
    type TargetList = pallet_staking::UseValidatorsMap<Self>;
    type MaxUnlockingChunks = ConstU32<32>;
    type MaxControllersInDeprecationBatch = MaxControllersInDeprecationBatch;
    type HistoryDepth = HistoryDepth;
    type EventListeners = ();
    type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
    type BenchmarkingConfig = StakingBenchmarkingConfig;
    type DisablingStrategy = pallet_staking::UpToLimitDisablingStrategy;
}
"
    .to_string()
}

fn get_runtime_api_code() -> String {
    "
    impl pallet_staking_runtime_api::StakingApi<Block, Balance, AccountId> for Runtime {
        fn nominations_quota(balance: Balance) -> u32 {
            Staking::api_nominations_quota(balance)
        }
        fn eras_stakers_page_count(era: sp_staking::EraIndex, account: AccountId) -> sp_staking::Page {
            Staking::api_eras_stakers_page_count(era, account)
        }
        fn pending_rewards(era: sp_staking::EraIndex, account: AccountId) -> bool {
            Staking::api_pending_rewards(era, account)
        }
    }
".to_string()
}
