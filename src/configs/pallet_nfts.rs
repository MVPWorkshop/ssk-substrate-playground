use super::super::pallet_index::NFTS;
use super::super::types::*;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletNftsTraits {
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
    ApprovalsLimit,
    ItemAttributesApprovalsLimit,
    MaxTips,
    MaxDeadlineDuration,
    MaxAttributesPerCall,
    Features,
    OffchainSignature,
    OffchainPublic,
    Helper,
    WeightInfo,
}

impl fmt::Display for PalletNftsTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletNftsTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletNftsTraits::CollectionId => write!(f, "CollectionId"),
            PalletNftsTraits::ItemId => write!(f, "ItemId"),
            PalletNftsTraits::Currency => write!(f, "Currency"),
            PalletNftsTraits::ForceOrigin => write!(f, "ForceOrigin"),
            PalletNftsTraits::CreateOrigin => write!(f, "CreateOrigin"),
            PalletNftsTraits::Locker => write!(f, "Locker"),
            PalletNftsTraits::CollectionDeposit => write!(f, "CollectionDeposit"),
            PalletNftsTraits::ItemDeposit => write!(f, "ItemDeposit"),
            PalletNftsTraits::MetadataDepositBase => write!(f, "MetadataDepositBase"),
            PalletNftsTraits::AttributeDepositBase => write!(f, "AttributeDepositBase"),
            PalletNftsTraits::DepositPerByte => write!(f, "DepositPerByte"),
            PalletNftsTraits::StringLimit => write!(f, "StringLimit"),
            PalletNftsTraits::KeyLimit => write!(f, "KeyLimit"),
            PalletNftsTraits::ValueLimit => write!(f, "ValueLimit"),
            PalletNftsTraits::ApprovalsLimit => write!(f, "ApprovalsLimit"),
            PalletNftsTraits::ItemAttributesApprovalsLimit => {
                write!(f, "ItemAttributesApprovalsLimit")
            }
            PalletNftsTraits::MaxTips => write!(f, "MaxTips"),
            PalletNftsTraits::MaxDeadlineDuration => write!(f, "MaxDeadlineDuration"),
            PalletNftsTraits::MaxAttributesPerCall => write!(f, "MaxAttributesPerCall"),
            PalletNftsTraits::Features => write!(f, "Features"),
            PalletNftsTraits::OffchainSignature => write!(f, "OffchainSignature"),
            PalletNftsTraits::OffchainPublic => write!(f, "OffchainPublic"),
            PalletNftsTraits::Helper => write!(f, "Helper"),
            PalletNftsTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}

pub struct PalletNftsConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletNftsConfig {
    fn default() -> Self {
        PalletNftsConfig::new()
    }
}

impl PalletNftsConfig {
    pub fn new() -> Self {
        let pallet_description = [ "A pallet for dealing with non-fungible assets.",
        "The NFTs pallet provides functionality for non-fungible tokens' management, including:Collection Creation, NFT Minting, NFT Transfers and Atomic Swaps, NFT Trading methods, Attributes Management, NFT Burning"
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME nfts pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            //updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::NFT]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-nfts".to_string(),
                version: None,
                alias: "pallet nfts".to_string(),
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
                index: Some(NFTS),
                runtime: (
                    "Nfts".to_string(),
                    "pallet_nfts::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletNftsTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletNftsTraits::CollectionId.to_string(),
                    "u32".to_string(),
                ),
                (PalletNftsTraits::ItemId.to_string(), "u32".to_string()),
                (
                    PalletNftsTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletNftsTraits::ForceOrigin.to_string(),
                    "EnsureRoot<Self::AccountId>".to_string(),
                ),
                (
                    PalletNftsTraits::CreateOrigin.to_string(),
                    "EnsureSigned<Self::AccountId>".to_string(),
                ),
                (PalletNftsTraits::Locker.to_string(), "()".to_string()),
                (
                    PalletNftsTraits::CollectionDeposit.to_string(),
                    "ConstU128<{ 10 * 1000 }>".to_string(),
                ),
                (
                    PalletNftsTraits::ItemDeposit.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletNftsTraits::MetadataDepositBase.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletNftsTraits::AttributeDepositBase.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletNftsTraits::DepositPerByte.to_string(),
                    "ConstU128<10>".to_string(),
                ),
                (
                    PalletNftsTraits::StringLimit.to_string(),
                    "ConstU32<256>".to_string(),
                ),
                (
                    PalletNftsTraits::KeyLimit.to_string(),
                    "ConstU32<64>".to_string(),
                ),
                (
                    PalletNftsTraits::ValueLimit.to_string(),
                    "ConstU32<256>".to_string(),
                ),
                (
                    PalletNftsTraits::ApprovalsLimit.to_string(),
                    "ConstU32<100>".to_string(),
                ),
                (
                    PalletNftsTraits::ItemAttributesApprovalsLimit.to_string(),
                    "ConstU32<100>".to_string(),
                ),
                (
                    PalletNftsTraits::MaxTips.to_string(),
                    "ConstU32<10>".to_string(),
                ),
                (
                    PalletNftsTraits::MaxDeadlineDuration.to_string(),
                    "ConstU32<1000>".to_string(),
                ),
                (
                    PalletNftsTraits::MaxAttributesPerCall.to_string(),
                    "ConstU32<5>".to_string(),
                ),
                (
                    PalletNftsTraits::Features.to_string(),
                    "NftsPalletFeatures".to_string(),
                ),
                (
                    PalletNftsTraits::OffchainSignature.to_string(),
                    "Signature".to_string(),
                ),
                (
                    PalletNftsTraits::OffchainPublic.to_string(),
                    "<Signature as sp_runtime::traits::Verify>::Signer".to_string(),
                ),
                (
                    PalletNftsTraits::WeightInfo.to_string(),
                    "pallet_nfts::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![String::from(
                "use pallet_nfts::PalletFeatures;",
            )]),
            runtime_api_code: None,
        };

        PalletNftsConfig {
            name: "Pallet nfts".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
        pub NftsPalletFeatures: PalletFeatures = PalletFeatures::all_enabled();
}
"
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for PalletNftsTraits enum display implementation
    #[test]
    fn test_pallet_nfts_traits_display() {
        assert_eq!(PalletNftsTraits::RuntimeEvent.to_string(), "RuntimeEvent");
        assert_eq!(PalletNftsTraits::CollectionId.to_string(), "CollectionId");
        assert_eq!(PalletNftsTraits::ItemId.to_string(), "ItemId");
        assert_eq!(PalletNftsTraits::Currency.to_string(), "Currency");
        assert_eq!(PalletNftsTraits::ForceOrigin.to_string(), "ForceOrigin");
        assert_eq!(PalletNftsTraits::CreateOrigin.to_string(), "CreateOrigin");
        assert_eq!(PalletNftsTraits::Locker.to_string(), "Locker");
        assert_eq!(
            PalletNftsTraits::CollectionDeposit.to_string(),
            "CollectionDeposit"
        );
        assert_eq!(PalletNftsTraits::ItemDeposit.to_string(), "ItemDeposit");
        assert_eq!(
            PalletNftsTraits::MetadataDepositBase.to_string(),
            "MetadataDepositBase"
        );
        assert_eq!(
            PalletNftsTraits::AttributeDepositBase.to_string(),
            "AttributeDepositBase"
        );
        assert_eq!(
            PalletNftsTraits::DepositPerByte.to_string(),
            "DepositPerByte"
        );
        assert_eq!(PalletNftsTraits::StringLimit.to_string(), "StringLimit");
        assert_eq!(PalletNftsTraits::KeyLimit.to_string(), "KeyLimit");
        assert_eq!(PalletNftsTraits::ValueLimit.to_string(), "ValueLimit");
        assert_eq!(
            PalletNftsTraits::ApprovalsLimit.to_string(),
            "ApprovalsLimit"
        );
        assert_eq!(
            PalletNftsTraits::ItemAttributesApprovalsLimit.to_string(),
            "ItemAttributesApprovalsLimit"
        );
        assert_eq!(PalletNftsTraits::MaxTips.to_string(), "MaxTips");
        assert_eq!(
            PalletNftsTraits::MaxDeadlineDuration.to_string(),
            "MaxDeadlineDuration"
        );
        assert_eq!(
            PalletNftsTraits::MaxAttributesPerCall.to_string(),
            "MaxAttributesPerCall"
        );
        assert_eq!(PalletNftsTraits::Features.to_string(), "Features");
        assert_eq!(
            PalletNftsTraits::OffchainSignature.to_string(),
            "OffchainSignature"
        );
        assert_eq!(
            PalletNftsTraits::OffchainPublic.to_string(),
            "OffchainPublic"
        );
        assert_eq!(PalletNftsTraits::Helper.to_string(), "Helper");
        assert_eq!(PalletNftsTraits::WeightInfo.to_string(), "WeightInfo");
    }

    // Test case for PalletNftsConfig::new() method (assuming PalletNftsConfig struct exists)
    #[test]
    fn test_pallet_nfts_config_new() {
        let pallet_nfts_config = PalletNftsConfig::new();

        // Test the name
        assert_eq!(pallet_nfts_config.name, "Pallet nfts");

        // Test metadata
        assert_eq!(
            pallet_nfts_config.metadata.short_description,
            "FRAME nfts pallet"
        );
        assert_eq!(pallet_nfts_config.metadata.size, 10500);
        assert_eq!(
            pallet_nfts_config.metadata.authors[0],
            CommonAuthors::ParityTechnologies
        );
        assert_eq!(
            pallet_nfts_config.metadata.categories.clone().unwrap()[0],
            PalletCategories::NFT
        );
        assert_eq!(
            pallet_nfts_config.metadata.license.clone().unwrap(),
            "Apache-2.0"
        );
        // Ensure description matches
        let expected_description = [
                "A pallet for dealing with non-fungible assets.",
                "The NFTs pallet provides functionality for non-fungible tokens' management, including:Collection Creation, NFT Minting, NFT Transfers and Atomic Swaps, NFT Trading methods, Attributes Management, NFT Burning"
             ].join("\n");
        assert_eq!(
            pallet_nfts_config.metadata.description,
            expected_description
        );
        // Test dependencies
        assert_eq!(
            pallet_nfts_config.dependencies.pallet.package,
            "pallet-nfts"
        );
        assert_eq!(pallet_nfts_config.dependencies.pallet.alias, "pallet nfts");
        assert_eq!(
            pallet_nfts_config
                .dependencies
                .pallet
                .git_repo
                .clone()
                .unwrap(),
            "https://github.com/paritytech/polkadot-sdk.git"
        );
        assert_eq!(
            pallet_nfts_config.dependencies.pallet.tag.clone().unwrap(),
            "polkadot-v1.14.0"
        );
        // Test runtime configuration
        let runtime_traits = &pallet_nfts_config.runtime.pallet_traits;

        assert_eq!(runtime_traits.get("RuntimeEvent").unwrap(), "RuntimeEvent");
        // assert_eq!(runtime_traits.get("ItemId").unwrap(), "ConstU32<1000>");
        assert_eq!(runtime_traits.get("Currency").unwrap(), "Balances");
        assert_eq!(
            runtime_traits.get("ForceOrigin").unwrap(),
            "EnsureRoot<Self::AccountId>"
        );
        assert_eq!(
            runtime_traits.get("CreateOrigin").unwrap(),
            "EnsureSigned<Self::AccountId>"
        );
        // assert_eq!(runtime_traits.get("Locker").unwrap(), "pallet_nfts::Locker");
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
        assert_eq!(runtime_traits.get("StringLimit").unwrap(), "ConstU32<256>");
        assert_eq!(runtime_traits.get("KeyLimit").unwrap(), "ConstU32<64>");
        assert_eq!(runtime_traits.get("ValueLimit").unwrap(), "ConstU32<256>");
        assert_eq!(
            runtime_traits.get("ApprovalsLimit").unwrap(),
            "ConstU32<100>"
        );
        assert_eq!(
            runtime_traits.get("ItemAttributesApprovalsLimit").unwrap(),
            "ConstU32<100>"
        );
        assert_eq!(runtime_traits.get("MaxTips").unwrap(), "ConstU32<10>");
        assert_eq!(
            runtime_traits.get("MaxDeadlineDuration").unwrap(),
            "ConstU32<1000>"
        );
        assert_eq!(
            runtime_traits.get("MaxAttributesPerCall").unwrap(),
            "ConstU32<5>"
        );
        assert_eq!(
            runtime_traits.get("OffchainSignature").unwrap(),
            "Signature"
        );
        assert_eq!(
            runtime_traits.get("OffchainPublic").unwrap(),
            "<Signature as sp_runtime::traits::Verify>::Signer"
        );
        assert_eq!(
            runtime_traits.get("WeightInfo").unwrap(),
            "pallet_nfts::weights::SubstrateWeight<Runtime>"
        );

        // Test runtime construct configuration
        assert_eq!(
            pallet_nfts_config.runtime.construct_runtime.index.unwrap(),
            18
        );
        assert_eq!(
            pallet_nfts_config.runtime.construct_runtime.runtime.0,
            "Nfts"
        );
        assert_eq!(
            pallet_nfts_config.runtime.construct_runtime.runtime.1,
            "pallet_nfts::Pallet<Runtime>"
        );
    }
}
