use super::super::pallet_index::ASSETS;
use super::super::types::*;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletAssetsTraits {
    RuntimeEvent,
    Balance,
    AssetId,
    AssetIdParameter,
    Currency,
    CreateOrigin,
    ForceOrigin,
    AssetDeposit,
    AssetAccountDeposit,
    MetadataDepositBase,
    MetadataDepositPerByte,
    ApprovalDeposit,
    StringLimit,
    Freezer,
    Extra,
    CallbackHandle,
    RemoveItemsLimit,
    BenchmarkHelper,
    WeightInfo,
}

impl fmt::Display for PalletAssetsTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletAssetsTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletAssetsTraits::Balance => write!(f, "Balance"),
            PalletAssetsTraits::AssetId => write!(f, "AssetId"),
            PalletAssetsTraits::AssetIdParameter => write!(f, "AssetIdParameter"),
            PalletAssetsTraits::Currency => write!(f, "Currency"),
            PalletAssetsTraits::CreateOrigin => write!(f, "CreateOrigin"),
            PalletAssetsTraits::ForceOrigin => write!(f, "ForceOrigin"),
            PalletAssetsTraits::AssetDeposit => write!(f, "AssetDeposit"),
            PalletAssetsTraits::AssetAccountDeposit => write!(f, "AssetAccountDeposit"),
            PalletAssetsTraits::MetadataDepositBase => write!(f, "MetadataDepositBase"),
            PalletAssetsTraits::MetadataDepositPerByte => write!(f, "MetadataDepositPerByte"),
            PalletAssetsTraits::ApprovalDeposit => write!(f, "ApprovalDeposit"),
            PalletAssetsTraits::StringLimit => write!(f, "StringLimit"),
            PalletAssetsTraits::Freezer => write!(f, "Freezer"),
            PalletAssetsTraits::Extra => write!(f, "Extra"),
            PalletAssetsTraits::CallbackHandle => write!(f, "CallbackHandle"),
            PalletAssetsTraits::WeightInfo => write!(f, "WeightInfo"),
            PalletAssetsTraits::RemoveItemsLimit => write!(f, "RemoveItemsLimit"),
            PalletAssetsTraits::BenchmarkHelper => write!(f, "BenchmarkHelper"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletAssetsConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletAssetsConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PalletAssetsConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "A simple, secure module for dealing with sets of assets implementing fungible traits.",
        ]
        .join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME Assets pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            //updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Assets]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-assets".to_string(),
                version: None,
                alias: "pallet assets".to_string(),
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
                index: Some(ASSETS),
                runtime: (
                    "Assets".to_string(),
                    "pallet_assets::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletAssetsTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (PalletAssetsTraits::Balance.to_string(), "u128".to_string()),
                (PalletAssetsTraits::AssetId.to_string(), "u32".to_string()),
                (
                    PalletAssetsTraits::AssetIdParameter.to_string(),
                    "codec::Compact<u32>".to_string(),
                ),
                (
                    PalletAssetsTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletAssetsTraits::CreateOrigin.to_string(),
                    "AsEnsureOriginWithArg<EnsureSigned<AccountId>>".to_string(),
                ),
                (
                    PalletAssetsTraits::ForceOrigin.to_string(),
                    "EnsureRoot<AccountId>".to_string(),
                ),
                (
                    PalletAssetsTraits::AssetDeposit.to_string(),
                    "AssetDeposit".to_string(),
                ),
                (
                    PalletAssetsTraits::AssetAccountDeposit.to_string(),
                    "ConstU128<DOLLARS>".to_string(),
                ),
                (
                    PalletAssetsTraits::MetadataDepositBase.to_string(),
                    "MetadataDepositBase".to_string(),
                ),
                (
                    PalletAssetsTraits::MetadataDepositPerByte.to_string(),
                    "MetadataDepositPerByte".to_string(),
                ),
                (
                    PalletAssetsTraits::ApprovalDeposit.to_string(),
                    "ApprovalDeposit".to_string(),
                ),
                (PalletAssetsTraits::Freezer.to_string(), "()".to_string()),
                (
                    PalletAssetsTraits::StringLimit.to_string(),
                    "StringLimit".to_string(),
                ),
                (PalletAssetsTraits::Extra.to_string(), "()".to_string()),
                (
                    PalletAssetsTraits::CallbackHandle.to_string(),
                    "()".to_string(),
                ),
                (
                    PalletAssetsTraits::WeightInfo.to_string(),
                    "pallet_assets::weights::SubstrateWeight<Runtime>".to_string(),
                ),
                (
                    PalletAssetsTraits::RemoveItemsLimit.to_string(),
                    "ConstU32<1000>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![String::from(
                "use frame_support::traits::AsEnsureOriginWithArg;",
            )]),
            runtime_api_code: None,
        };

        PalletAssetsConfig {
            name: "Pallet assets".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
	pub const AssetDeposit: Balance = 100 * DOLLARS;
	pub const ApprovalDeposit: Balance = 1 * DOLLARS;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10 * DOLLARS;
	pub const MetadataDepositPerByte: Balance = 1 * DOLLARS;
}
"
    .to_string()
}
