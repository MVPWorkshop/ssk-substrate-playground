use super::super::pallet_index::ELECTION_PROVIDER_MULTI_PHASE;
use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletElectionProviderMultiPhaseTraits {
    RuntimeEvent,
    Currency,
    EstimateCallFee,
    SignedPhase,
    UnsignedPhase,
    BetterSignedThreshold,
    OffchainRepeat,
    MinerTxPriority,
    MinerConfig,
    SignedMaxSubmissions,
    SignedRewardBase,
    SignedDepositBase,
    SignedDepositByte,
    SignedMaxRefunds,
    SignedDepositWeight,
    SignedMaxWeight,
    SlashHandler,
    RewardHandler,
    DataProvider,
    Fallback,
    GovernanceFallback,
    Solver,
    ForceOrigin,
    MaxWinners,
    ElectionBounds,
    BenchmarkingConfig,
    WeightInfo,
}

impl fmt::Display for PalletElectionProviderMultiPhaseTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletElectionProviderMultiPhaseTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletElectionProviderMultiPhaseTraits::Currency => write!(f, "Currency"),
            PalletElectionProviderMultiPhaseTraits::EstimateCallFee => write!(f, "EstimateCallFee"),
            PalletElectionProviderMultiPhaseTraits::SignedPhase => write!(f, "SignedPhase"),
            PalletElectionProviderMultiPhaseTraits::UnsignedPhase => write!(f, "UnsignedPhase"),
            PalletElectionProviderMultiPhaseTraits::BetterSignedThreshold => {
                write!(f, "BetterSignedThreshold")
            }
            PalletElectionProviderMultiPhaseTraits::OffchainRepeat => write!(f, "OffchainRepeat"),
            PalletElectionProviderMultiPhaseTraits::MinerTxPriority => write!(f, "MinerTxPriority"),
            PalletElectionProviderMultiPhaseTraits::MinerConfig => write!(f, "MinerConfig"),
            PalletElectionProviderMultiPhaseTraits::SignedMaxSubmissions => {
                write!(f, "SignedMaxSubmissions")
            }
            PalletElectionProviderMultiPhaseTraits::SignedRewardBase => {
                write!(f, "SignedRewardBase")
            }
            PalletElectionProviderMultiPhaseTraits::SignedDepositBase => {
                write!(f, "SignedDepositBase")
            }
            PalletElectionProviderMultiPhaseTraits::SignedDepositByte => {
                write!(f, "SignedDepositByte")
            }
            PalletElectionProviderMultiPhaseTraits::SignedMaxRefunds => {
                write!(f, "SignedMaxRefunds")
            }
            PalletElectionProviderMultiPhaseTraits::SignedDepositWeight => {
                write!(f, "SignedDepositWeight")
            }
            PalletElectionProviderMultiPhaseTraits::SignedMaxWeight => write!(f, "SignedMaxWeight"),
            PalletElectionProviderMultiPhaseTraits::SlashHandler => write!(f, "SlashHandler"),
            PalletElectionProviderMultiPhaseTraits::RewardHandler => write!(f, "RewardHandler"),
            PalletElectionProviderMultiPhaseTraits::DataProvider => write!(f, "DataProvider"),
            PalletElectionProviderMultiPhaseTraits::Fallback => write!(f, "Fallback"),
            PalletElectionProviderMultiPhaseTraits::GovernanceFallback => {
                write!(f, "GovernanceFallback")
            }
            PalletElectionProviderMultiPhaseTraits::Solver => write!(f, "Solver"),
            PalletElectionProviderMultiPhaseTraits::ForceOrigin => write!(f, "ForceOrigin"),
            PalletElectionProviderMultiPhaseTraits::MaxWinners => write!(f, "MaxWinners"),
            PalletElectionProviderMultiPhaseTraits::ElectionBounds => write!(f, "ElectionBounds"),
            PalletElectionProviderMultiPhaseTraits::BenchmarkingConfig => {
                write!(f, "BenchmarkingConfig")
            }
            PalletElectionProviderMultiPhaseTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PalletElectionProviderMultiPhaseConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletElectionProviderMultiPhaseConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PalletElectionProviderMultiPhaseConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "Currently, this election-provider has two distinct phases (see [`Phase`]), **signed** and **unsigned**.",
            "In the signed phase, solutions (of type [`RawSolution`]) are submitted and queued on chain. A deposit is reserved, based on the size of the solution, for the cost of keeping this solution on-chain for a number of blocks, and the potential weight of the solution upon being checked. A maximum of `pallet::Config::SignedMaxSubmissions` solutions are stored. The queue is always sorted based on score (worse to best).",
            "The unsigned phase will always follow the signed phase, with the specified duration. In this phase, only validator nodes can submit solutions. A validator node who has offchain workers enabled will start to mine a solution in this phase and submits it back to the chain as an unsigned transaction, thus the name _unsigned_ phase. This unsigned transaction can never be valid if propagated, and it acts similar to an inherent.",
        ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME Multi phase, offchain election provider pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Governance]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-election-provider-multi-phase".to_string(),
                version: None,
                alias: "pallet election provider multi phase".to_string(),
                default_features: false,
                git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                tag: Some("polkadot-v1.14.0".to_string()),
                branch: None,
            },
            additional_pallets: Some(vec![CargoComplexDependency {
                package: "sp-io".to_string(),
                version: None,
                alias: "sp io".to_string(),
                default_features: false,
                git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                tag: Some("polkadot-v1.14.0".to_string()),
                branch: None,
            }]),
            additional_deps: None,
        };

        let runtime = PalletRuntimeConfig {
            construct_runtime: PalletConstructRuntimeConfig {
                index: Some(ELECTION_PROVIDER_MULTI_PHASE),
                runtime: (
                    "ElectionProviderMultiPhase".to_string(),
                    "pallet_election_provider_multi_phase::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletElectionProviderMultiPhaseTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::EstimateCallFee.to_string(),
                    "TransactionPayment".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedPhase.to_string(),
                    "SignedPhase".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::UnsignedPhase.to_string(),
                    "UnsignedPhase".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::BetterSignedThreshold.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::OffchainRepeat.to_string(),
                    "OffchainRepeat".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::MinerTxPriority.to_string(),
                    "MultiPhaseUnsignedPriority".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::MinerConfig.to_string(),
                    "Self".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedMaxSubmissions.to_string(),
                    "ConstU32<10>".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedRewardBase.to_string(),
                    "SignedRewardBase".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedDepositBase.to_string(),
                    "GeometricDepositBase<Balance, SignedFixedDeposit, SignedDepositIncreaseFactor>".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedDepositByte.to_string(),
                    "SignedDepositByte".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedMaxRefunds.to_string(),
                    "ConstU32<3>".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedDepositWeight.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SignedMaxWeight.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::SlashHandler.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::RewardHandler.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::DataProvider.to_string(),
                    "Staking".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::Fallback.to_string(),
                    "onchain::OnChainExecution<OnChainSeqPhragmen>".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::GovernanceFallback.to_string(),
                    "onchain::OnChainExecution<OnChainSeqPhragmen>".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::Solver.to_string(),
                    "SequentialPhragmen<AccountId, SolutionAccuracyOf<Self>, OffchainRandomBalancing>".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::ForceOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::MaxWinners.to_string(),
                    "MaxActiveValidators".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::ElectionBounds.to_string(),
                    "ElectionBoundsMultiPhase".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::BenchmarkingConfig.to_string(),
                    "ElectionProviderBenchmarkConfig".to_string(),
                ),
                (
                    PalletElectionProviderMultiPhaseTraits::WeightInfo.to_string(),
                    "pallet_election_provider_multi_phase::weights::SubstrateWeight<Self>".to_string(),
                ),

            ]
                .into_iter()
                .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use frame_support::pallet_prelude::Get;"),
                String::from("use frame_support::dispatch::DispatchClass;"),
                String::from("use sp_runtime::transaction_validity::TransactionPriority;"),
                String::from("use pallet_election_provider_multi_phase::SolutionAccuracyOf;"),
                String::from("pub use pallet_election_provider_multi_phase::{Call as EPMCall, GeometricDepositBase};"),
            ]),
            runtime_api_code: None,
        };

        PalletElectionProviderMultiPhaseConfig {
            name: "Pallet election provider multi phase".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
	pub ElectionBoundsMultiPhase: ElectionBounds = ElectionBoundsBuilder::default()
		.voters_count(10_000.into()).targets_count(1_500.into()).build();
	pub ElectionBoundsOnChain: ElectionBounds = ElectionBoundsBuilder::default()
		.voters_count(5_000.into()).targets_count(1_250.into()).build();
	pub MaxElectingVotersSolution: u32 = 40_000;
	pub MaxActiveValidators: u32 = 1000;
	pub const SignedPhase: u32 = 200 / 4; // EPOCH_DURATION_IN_BLOCKS / 4;
	pub const UnsignedPhase: u32 = 200 / 4; // EPOCH_DURATION_IN_BLOCKS / 4;
    pub const MultiPhaseUnsignedPriority: TransactionPriority = StakingUnsignedPriority::get() - 1u64;
	// Solution can occupy 90% of normal block size
	pub MinerMaxLength: u32 = Perbill::from_rational(9u32, 10) *
		*BlockLength::get().max.get(DispatchClass::Normal);
    pub const SignedRewardBase: Balance = 1 * DOLLARS;
	pub const SignedFixedDeposit: Balance = 1 * DOLLARS;
	pub const SignedDepositIncreaseFactor: Percent = Percent::from_percent(10);
	pub const SignedDepositByte: Balance = 1 * CENTS;
    pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
}
pub struct ElectionProviderBenchmarkConfig;
impl pallet_election_provider_multi_phase::BenchmarkingConfig for ElectionProviderBenchmarkConfig {
    const VOTERS: [u32; 2] = [1000, 2000];
    const TARGETS: [u32; 2] = [500, 1000];
    const ACTIVE_VOTERS: [u32; 2] = [500, 800];
    const DESIRED_TARGETS: [u32; 2] = [200, 400];
    const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
    const MINER_MAXIMUM_VOTERS: u32 = 1000;
    const MAXIMUM_TARGETS: u32 = 300;
}
/// Maximum number of iterations for balancing that will be executed in the embedded OCW
/// miner of election provider multi phase.
pub const MINER_MAX_ITERATIONS: u32 = 10;
/// A source of random balance for NposSolver, which is meant to be run by the OCW election miner.
pub struct OffchainRandomBalancing;
impl Get<Option<BalancingConfig>> for OffchainRandomBalancing {
    fn get() -> Option<BalancingConfig> {
        use sp_runtime::traits::TrailingZeroInput;
        let iterations = match MINER_MAX_ITERATIONS {
            0 => 0,
            max => {
                let seed = sp_io::offchain::random_seed();
                let random = <u32>::decode(&mut TrailingZeroInput::new(&seed))
                    .expect(\"input is padded with zeroes; qed\") %
                    max.saturating_add(1);
                random as usize
            },
        };
        let config = BalancingConfig { iterations, tolerance: 0 };
        Some(config)
    }
}
pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
    type System = Runtime;
    type Solver = SequentialPhragmen<
        AccountId,
        pallet_election_provider_multi_phase::SolutionAccuracyOf<Runtime>,
    >;
    type DataProvider = <Runtime as pallet_election_provider_multi_phase::Config>::DataProvider;
    type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>;
    type MaxWinners = <Runtime as pallet_election_provider_multi_phase::Config>::MaxWinners;
    type Bounds = ElectionBoundsOnChain;
}
impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
    type AccountId = AccountId;
    type MaxLength = MinerMaxLength;
    type MaxWeight = ();
    type Solution = NposSolution16;
    type MaxVotesPerVoter =
    <<Self as pallet_election_provider_multi_phase::Config>::DataProvider as ElectionDataProvider>::MaxVotesPerVoter;
    type MaxWinners = MaxActiveValidators;
    // The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
    // weight estimate function is wired to this call's weight.
    fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
        <
        <Self as pallet_election_provider_multi_phase::Config>::WeightInfo
        as
        pallet_election_provider_multi_phase::WeightInfo
        >::submit_unsigned(v, t, a, d)
    }
}
impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    RuntimeCall: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = RuntimeCall;
}
frame_election_provider_support::generate_solution_type!(
	#[compact]
	pub struct NposSolution16::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = sp_runtime::PerU16,
		MaxVoters = MaxElectingVotersSolution,
	>(16)
);
"
        .to_string()
}
