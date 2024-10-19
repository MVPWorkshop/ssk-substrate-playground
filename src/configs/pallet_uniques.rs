use super::super::pallet_index::UNIQUES;
use super::super::types::*;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletUniquesTraits {
    RuntimeEvent,
    CollectionId,
    ItemId,
    Currency,
    ForceOrigin,
    CreateOrigin,
    Locker,
    CollectionDeposit,
    ItemDeposit,
    MetadataDepositBase,
    AttributeDepositBase,
    DepositPerByte,
    StringLimit,
    KeyLimit,
    ValueLimit,
    Helper,
    WeightInfo,
}

impl fmt::Display for PalletUniquesTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletUniquesTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletUniquesTraits::CollectionId => write!(f, "CollectionId"),
            PalletUniquesTraits::ItemId => write!(f, "ItemId"),
            PalletUniquesTraits::Currency => write!(f, "Currency"),
            PalletUniquesTraits::ForceOrigin => write!(f, "ForceOrigin"),
            PalletUniquesTraits::CreateOrigin => write!(f, "CreateOrigin"),
            PalletUniquesTraits::Locker => write!(f, "Locker"),
            PalletUniquesTraits::CollectionDeposit => write!(f, "CollectionDeposit"),
            PalletUniquesTraits::ItemDeposit => write!(f, "ItemDeposit"),
            PalletUniquesTraits::MetadataDepositBase => write!(f, "MetadataDepositBase"),
            PalletUniquesTraits::AttributeDepositBase => write!(f, "AttributeDepositBase"),
            PalletUniquesTraits::DepositPerByte => write!(f, "DepositPerByte"),
            PalletUniquesTraits::StringLimit => write!(f, "StringLimit"),
            PalletUniquesTraits::KeyLimit => write!(f, "KeyLimit"),
            PalletUniquesTraits::ValueLimit => write!(f, "ValueLimit"),
            PalletUniquesTraits::Helper => write!(f, "Helper"),
            PalletUniquesTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}

pub struct PalletUniquesConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletUniquesConfig {
    fn default() -> Self {
        PalletUniquesConfig::new()
    }
}

impl PalletUniquesConfig {
    pub fn new() -> Self {
        let pallet_description = [ "A module for managing non-fungible tokens (NFTs) and collections, providing a secure and flexible framework."
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME uniques pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            is_essential: false,
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::NFT]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-uniques".to_string(),
                version: None,
                alias: "pallet uniques".to_string(),
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
                index: Some(UNIQUES),
                runtime: (
                    "Uniques".to_string(),
                    "pallet_uniques::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletUniquesTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletUniquesTraits::CollectionId.to_string(),
                    "u32".to_string(),
                ),
                (PalletUniquesTraits::ItemId.to_string(), "u32".to_string()),
                (
                    PalletUniquesTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletUniquesTraits::ForceOrigin.to_string(),
                    "EnsureRoot<Self::AccountId>".to_string(),
                ),
                (
                    PalletUniquesTraits::CreateOrigin.to_string(),
                    "EnsureSigned<Self::AccountId>".to_string(),
                ),
                (PalletUniquesTraits::Locker.to_string(), "()".to_string()),
                (
                    PalletUniquesTraits::CollectionDeposit.to_string(),
                    "ConstU128<{ 10 * 1000 }>".to_string(),
                ),
                (
                    PalletUniquesTraits::ItemDeposit.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletUniquesTraits::MetadataDepositBase.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletUniquesTraits::AttributeDepositBase.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletUniquesTraits::DepositPerByte.to_string(),
                    "ConstU128<10>".to_string(),
                ),
                (
                    PalletUniquesTraits::StringLimit.to_string(),
                    "ConstU32<256>".to_string(),
                ),
                (
                    PalletUniquesTraits::KeyLimit.to_string(),
                    "ConstU32<64>".to_string(),
                ),
                (
                    PalletUniquesTraits::ValueLimit.to_string(),
                    "ConstU32<256>".to_string(),
                ),
                (
                    PalletUniquesTraits::WeightInfo.to_string(),
                    "pallet_uniques::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: None,
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: None,
            runtime_api_code: None,
        };

        PalletUniquesConfig {
            name: "Pallet uniques".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}
