use super::super::types::*;
use chrono::Utc;
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

impl PalletUniquesConfig {
    pub fn new() -> Self {
        let pallet_description = [ "A module for managing non-fungible tokens (NFTs) and collections, providing a secure and flexible framework."
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME uniques pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Uniques]),
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
                index: Some(12),
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
                    "ConstU32<100>".to_string(),
                ),
                (
                    PalletUniquesTraits::ItemId.to_string(),
                    "ConstU32<1000>".to_string(),
                ),
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
                (
                    PalletUniquesTraits::Locker.to_string(),
                    "pallet_uniques::Locker".to_string(),
                ),
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
                    "ConstU128<{ 10 }>".to_string(),
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
                (PalletUniquesTraits::Helper.to_string(), "()".to_string()),
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
            additional_runtime_lib_code: Some(vec![
                String::from("use pallet_uniques::legacy::UniquesInfo;"),
                String::from("use frame_system::EnsureRoot;"),
            ]),
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

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for PalletUniquesTraits enum display implementation
    #[test]
    fn test_pallet_uniques_traits_display() {
        assert_eq!(
            PalletUniquesTraits::RuntimeEvent.to_string(),
            "RuntimeEvent"
        );
        assert_eq!(
            PalletUniquesTraits::CollectionId.to_string(),
            "CollectionId"
        );
        assert_eq!(PalletUniquesTraits::ItemId.to_string(), "ItemId");
        assert_eq!(PalletUniquesTraits::Currency.to_string(), "Currency");
        assert_eq!(PalletUniquesTraits::ForceOrigin.to_string(), "ForceOrigin");
        assert_eq!(
            PalletUniquesTraits::CreateOrigin.to_string(),
            "CreateOrigin"
        );
        assert_eq!(PalletUniquesTraits::Locker.to_string(), "Locker");
        assert_eq!(
            PalletUniquesTraits::CollectionDeposit.to_string(),
            "CollectionDeposit"
        );
        assert_eq!(PalletUniquesTraits::ItemDeposit.to_string(), "ItemDeposit");
        assert_eq!(
            PalletUniquesTraits::MetadataDepositBase.to_string(),
            "MetadataDepositBase"
        );
        assert_eq!(
            PalletUniquesTraits::AttributeDepositBase.to_string(),
            "AttributeDepositBase"
        );
        assert_eq!(
            PalletUniquesTraits::DepositPerByte.to_string(),
            "DepositPerByte"
        );
        assert_eq!(PalletUniquesTraits::StringLimit.to_string(), "StringLimit");
        assert_eq!(PalletUniquesTraits::KeyLimit.to_string(), "KeyLimit");
        assert_eq!(PalletUniquesTraits::ValueLimit.to_string(), "ValueLimit");
        assert_eq!(PalletUniquesTraits::Helper.to_string(), "Helper");
        assert_eq!(PalletUniquesTraits::WeightInfo.to_string(), "WeightInfo");
    }

    // Test case for PalletUniquesConfig::new() method (assuming PalletUniquesConfig struct exists)
    #[test]
    fn test_pallet_uniques_config_new() {
        let pallet_uniques_config = PalletUniquesConfig::new();

        // Test the name
        assert_eq!(pallet_uniques_config.name, "Pallet uniques");

        // Test metadata
        assert_eq!(
            pallet_uniques_config.metadata.short_description,
            "FRAME uniques pallet"
        );
        assert_eq!(pallet_uniques_config.metadata.size, 10500); // Assuming a size value
        assert_eq!(
            pallet_uniques_config.metadata.authors[0],
            CommonAuthors::ParityTechnologies
        );
        assert_eq!(
            pallet_uniques_config.metadata.categories.clone().unwrap()[0],
            PalletCategories::Uniques
        );
        assert_eq!(
            pallet_uniques_config.metadata.license.clone().unwrap(),
            "Apache-2.0"
        );

        // Ensure description matches
        let expected_description = [
            "A module for managing non-fungible tokens (NFTs) and collections, providing a secure and flexible framework."
              ].join("\n");
        assert_eq!(
            pallet_uniques_config.metadata.description,
            expected_description
        );

        // Test dependencies
        assert_eq!(
            pallet_uniques_config.dependencies.pallet.package,
            "pallet-uniques"
        );
        assert_eq!(
            pallet_uniques_config.dependencies.pallet.alias,
            "pallet uniques"
        );
        assert_eq!(
            pallet_uniques_config
                .dependencies
                .pallet
                .git_repo
                .clone()
                .unwrap(),
            "https://github.com/paritytech/polkadot-sdk.git"
        );
        assert_eq!(
            pallet_uniques_config
                .dependencies
                .pallet
                .tag
                .clone()
                .unwrap(),
            "polkadot-v1.14.0"
        );

        // Test runtime configuration
        let runtime_traits = &pallet_uniques_config.runtime.pallet_traits;
        assert_eq!(runtime_traits.get("RuntimeEvent").unwrap(), "RuntimeEvent");
        assert_eq!(runtime_traits.get("CollectionId").unwrap(), "ConstU32<100>");
        assert_eq!(runtime_traits.get("ItemId").unwrap(), "ConstU32<1000>");
        assert_eq!(runtime_traits.get("Currency").unwrap(), "Balances");
        assert_eq!(
            runtime_traits.get("ForceOrigin").unwrap(),
            "EnsureRoot<Self::AccountId>"
        );
        assert_eq!(
            runtime_traits.get("CreateOrigin").unwrap(),
            "EnsureSigned<Self::AccountId>"
        );
        assert_eq!(
            runtime_traits.get("Locker").unwrap(),
            "pallet_uniques::Locker"
        );
        assert_eq!(
            runtime_traits.get("CollectionDeposit").unwrap(),
            "ConstU128<{ 10 * 1000 }>"
        );
        assert_eq!(
            runtime_traits.get("ItemDeposit").unwrap(),
            "ConstU128<{ 1 * 1000 }>"
        );
        assert_eq!(
            runtime_traits.get("MetadataDepositBase").unwrap(),
            "ConstU128<{ 1 * 1000 }>"
        );
        assert_eq!(
            runtime_traits.get("AttributeDepositBase").unwrap(),
            "ConstU128<{ 1 * 1000 }>"
        );
        assert_eq!(
            runtime_traits.get("DepositPerByte").unwrap(),
            "ConstU128<{ 10 }>"
        );
        assert_eq!(runtime_traits.get("StringLimit").unwrap(), "ConstU32<256>");
        assert_eq!(runtime_traits.get("KeyLimit").unwrap(), "ConstU32<64>");
        assert_eq!(runtime_traits.get("ValueLimit").unwrap(), "ConstU32<256>");
        assert_eq!(runtime_traits.get("Helper").unwrap(), "()");
        assert_eq!(
            runtime_traits.get("WeightInfo").unwrap(),
            "pallet_uniques::weights::SubstrateWeight<Runtime>"
        );

        // Test runtime construct configuration
        assert_eq!(
            pallet_uniques_config
                .runtime
                .construct_runtime
                .index
                .unwrap(),
            12
        );
        assert_eq!(
            pallet_uniques_config.runtime.construct_runtime.runtime.0,
            "Uniques"
        );
        assert_eq!(
            pallet_uniques_config.runtime.construct_runtime.runtime.1,
            "pallet_uniques::Pallet<Runtime>"
        );
    }
}
