use super::super::pallet_index::TREASURY;
use super::super::types::*;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletTreasuryTraits {
    RuntimeEvent,
    PalletId,
    Currency,
    RejectOrigin,
    SpendPeriod,
    Burn,
    BurnDestination,
    SpendFunds,
    WeightInfo,
    MaxApprovals,
    SpendOrigin,
    AssetKind,
    Beneficiary,
    BeneficiaryLookup,
    Paymaster,
    BalanceConverter,
    PayoutPeriod,
}

impl fmt::Display for PalletTreasuryTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletTreasuryTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletTreasuryTraits::PalletId => write!(f, "PalletId"),
            PalletTreasuryTraits::Currency => write!(f, "Currency"),
            PalletTreasuryTraits::RejectOrigin => write!(f, "RejectOrigin"),
            PalletTreasuryTraits::SpendPeriod => write!(f, "SpendPeriod"),
            PalletTreasuryTraits::Burn => write!(f, "Burn"),
            PalletTreasuryTraits::BurnDestination => write!(f, "BurnDestination"),
            PalletTreasuryTraits::SpendFunds => write!(f, "SpendFunds"),
            PalletTreasuryTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletTreasuryTraits::MaxApprovals => write!(f, "MaxApprovals"),
            PalletTreasuryTraits::SpendOrigin => write!(f, "SpendOrigin"),
            PalletTreasuryTraits::AssetKind => write!(f, "AssetKind"),
            PalletTreasuryTraits::Beneficiary => write!(f, "Beneficiary"),
            PalletTreasuryTraits::BeneficiaryLookup => write!(f, "BeneficiaryLookup"),
            PalletTreasuryTraits::Paymaster => write!(f, "Paymaster"),
            PalletTreasuryTraits::BalanceConverter => write!(f, "BalanceConverter"),
            PalletTreasuryTraits::PayoutPeriod => write!(f, "PayoutPeriod"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletTreasuryConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletTreasuryConfig {
    fn default() -> Self {
        PalletTreasuryConfig::new()
    }
}

impl PalletTreasuryConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "The Treasury pallet provides a pot of funds that can be managed by stakeholders in the system and a structure for making spending proposals from this pot.",
            "The Treasury Pallet itself provides the pot to store funds, and a means for stakeholders to propose and claim expenditures (aka spends). The chain will need to provide a method to approve spends (e.g. public referendum) and a method for collecting funds (e.g. inflation, fees).",
        ]
            .join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME Treasury pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            is_essential: false,
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Assets]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-treasury".to_string(),
                version: None,
                alias: "pallet treasury".to_string(),
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
                index: Some(TREASURY),
                runtime: (
                    "Treasury".to_string(),
                    "pallet_treasury::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletTreasuryTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletTreasuryTraits::PalletId.to_string(),
                    "TreasuryPalletId".to_string(),
                ),
                (
                    PalletTreasuryTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletTreasuryTraits::RejectOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletTreasuryTraits::SpendPeriod.to_string(),
                    "SpendPeriod".to_string(),
                ),
                (PalletTreasuryTraits::Burn.to_string(), "Burn".to_string()),
                (
                    PalletTreasuryTraits::BurnDestination.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletTreasuryTraits::SpendFunds.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletTreasuryTraits::WeightInfo.to_string(),
                    "pallet_treasury::weights::SubstrateWeight<Runtime>".to_string(),
                ),
                (
                    PalletTreasuryTraits::MaxApprovals.to_string(),
                    "MaxApprovals".to_string(),
                ),
                (
                    PalletTreasuryTraits::SpendOrigin.to_string(),
                    "EnsureWithSuccess<EnsureRoot<AccountId>, AccountId, MaxBalance>".to_string(),
                ),
                (
                    PalletTreasuryTraits::AssetKind.to_string(),
                    "u32".to_string(),
                ),
                (
                    PalletTreasuryTraits::Beneficiary.to_string(),
                    "AccountId".to_string(),
                ),
                (
                    PalletTreasuryTraits::BeneficiaryLookup.to_string(),
                    "Self::Lookup".to_string(),
                ),
                (
                    PalletTreasuryTraits::Paymaster.to_string(),
                    "PayAssetFromAccount<Assets, TreasuryAccount>".to_string(),
                ),
                (
                    PalletTreasuryTraits::BalanceConverter.to_string(),
                    "UnityAssetBalanceConversion".to_string(),
                ),
                (
                    PalletTreasuryTraits::PayoutPeriod.to_string(),
                    "SpendPayoutPeriod".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use sp_runtime::Percent;"),
                String::from("use frame_system::EnsureWithSuccess;"),
                String::from("use frame_support::traits::tokens::pay::PayAssetFromAccount;"),
                String::from("use frame_support::traits::tokens::UnityAssetBalanceConversion;"),
            ]),
            runtime_api_code: None,
        };

        PalletTreasuryConfig {
            name: "Pallet treasury".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
	pub const SpendPeriod: BlockNumber = 1 * DAYS;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: BlockNumber = 1 * DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: Balance = 1 * DOLLARS;
	pub const DataDepositPerByte: Balance = 1 * CENTS;
	pub const TreasuryPalletId: PalletId = PalletId(*b\"py/trsry\");
	pub const MaximumReasonLength: u32 = 300;
	pub const MaxApprovals: u32 = 100;
	pub const MaxBalance: Balance = Balance::max_value();
	pub const SpendPayoutPeriod: BlockNumber = 30 * DAYS;
	pub TreasuryAccount: AccountId = Treasury::account_id();
}
"
    .to_string()
}
