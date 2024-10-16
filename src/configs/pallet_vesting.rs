use super::super::pallet_index::VESTING;
use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletVestingTraits {
    RuntimeEvent,
    Currency,
    MinVestedTransfer,
    WeightInfo,
    UnvestedFundsAllowedWithdrawReasons,
    BlockNumberProvider,
    BlockNumberToBalance,
}

impl fmt::Display for PalletVestingTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletVestingTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletVestingTraits::Currency => write!(f, "Currency"),
            PalletVestingTraits::BlockNumberToBalance => write!(f, "BlockNumberToBalance"),
            PalletVestingTraits::MinVestedTransfer => write!(f, "MinVestedTransfer"),
            PalletVestingTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletVestingTraits::UnvestedFundsAllowedWithdrawReasons => {
                write!(f, "UnvestedFundsAllowedWithdrawReasons")
            }
            PalletVestingTraits::BlockNumberProvider => write!(f, "BlockNumberProvider"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PalletVestingConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletVestingConfig {
    fn default() -> Self {
        PalletVestingConfig::new()
    }
}

impl PalletVestingConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "A simple pallet providing a means of placing a linear curve on an account's locked balance. This pallet ensures that there is a lock in place preventing the balance to drop below the *unvested* amount for any reason other than the ones specified in `UnvestedFundsAllowedWithdrawReasons` configuration value.",
            "As the amount vested increases over time, the amount unvested reduces. However, locks remain in place and explicit action is needed on behalf of the user to ensure that the amount locked is equivalent to the amount remaining to be vested. This is done through a dispatchable function, either `vest` (in typical case where the sender is calling on their own behalf) or `vest_other` in case the sender is calling on another account's behalf.",
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME vesting pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Runtime]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-vesting".to_string(),
                version: None,
                alias: "pallet vesting".to_string(),
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
                index: Some(VESTING),
                runtime: (
                    "Vesting".to_string(),
                    "pallet_vesting::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![].into_iter().collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use frame_support::traits::WithdrawReasons;"),
                String::from("use sp_runtime::traits::ConvertInto;"),
            ]),
            runtime_api_code: None,
        };

        PalletVestingConfig {
            name: "Pallet vesting".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
	pub const MinVestedTransfer: Balance = 100 * DOLLARS;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}
impl pallet_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type BlockNumberProvider = System;
	const MAX_VESTING_SCHEDULES: u32 = 28;
}
"
    .to_string()
}
