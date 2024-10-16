use super::super::pallet_index::IDENTITY;
use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletIdentityTraits {
    RuntimeEvent,
    Currency,
    BasicDeposit,
    ByteDeposit,
    SubAccountDeposit,
    MaxSubAccounts,
    IdentityInformation,
    MaxRegistrars,
    Slashed,
    ForceOrigin,
    RegistrarOrigin,
    OffchainSignature,
    SigningPublicKey,
    UsernameAuthorityOrigin,
    PendingUsernameExpiration,
    MaxSuffixLength,
    MaxUsernameLength,
    WeightInfo,
}

impl fmt::Display for PalletIdentityTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletIdentityTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletIdentityTraits::Currency => write!(f, "Currency"),
            PalletIdentityTraits::BasicDeposit => write!(f, "BasicDeposit"),
            PalletIdentityTraits::ByteDeposit => write!(f, "ByteDeposit"),
            PalletIdentityTraits::SubAccountDeposit => write!(f, "SubAccountDeposit"),
            PalletIdentityTraits::MaxSubAccounts => write!(f, "MaxSubAccounts"),
            PalletIdentityTraits::IdentityInformation => write!(f, "IdentityInformation"),
            PalletIdentityTraits::MaxRegistrars => write!(f, "MaxRegistrars"),
            PalletIdentityTraits::Slashed => write!(f, "Slashed"),
            PalletIdentityTraits::ForceOrigin => write!(f, "ForceOrigin"),
            PalletIdentityTraits::RegistrarOrigin => write!(f, "RegistrarOrigin"),
            PalletIdentityTraits::OffchainSignature => write!(f, "OffchainSignature"),
            PalletIdentityTraits::SigningPublicKey => write!(f, "SigningPublicKey"),
            PalletIdentityTraits::UsernameAuthorityOrigin => write!(f, "UsernameAuthorityOrigin"),
            PalletIdentityTraits::PendingUsernameExpiration => {
                write!(f, "PendingUsernameExpiration")
            }
            PalletIdentityTraits::MaxSuffixLength => write!(f, "MaxSuffixLength"),
            PalletIdentityTraits::MaxUsernameLength => write!(f, "MaxUsernameLength"),
            PalletIdentityTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletIdentityConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletIdentityConfig {
    fn default() -> Self {
        PalletIdentityConfig::new()
    }
}

impl PalletIdentityConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "A federated naming system, allowing for multiple registrars to be added from a specified origin.",
            "Registrars can set a fee to provide identity-verification service. Anyone can put forth a proposed identity for a fixed deposit and ask for review by any number of registrars (paying each of their fees). Registrar judgements are given as an `enum`, allowing for sophisticated, multi-tier opinions."
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME identity pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Identity]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-identity".to_string(),
                version: None,
                alias: "pallet identity".to_string(),
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
                index: Some(IDENTITY),
                runtime: (
                    "Identity".to_string(),
                    "pallet_identity::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletIdentityTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletIdentityTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletIdentityTraits::BasicDeposit.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletIdentityTraits::ByteDeposit.to_string(),
                    "ConstU128<{ 1 * 1000 }>".to_string(),
                ),
                (
                    PalletIdentityTraits::SubAccountDeposit.to_string(),
                    "ConstU128<{ 100 * 1000 }>".to_string(),
                ),
                (
                    PalletIdentityTraits::MaxSubAccounts.to_string(),
                    "ConstU32<100>".to_string(),
                ),
                (
                    PalletIdentityTraits::IdentityInformation.to_string(),
                    "IdentityInfo<ConstU32<100>>".to_string(),
                ),
                (
                    PalletIdentityTraits::MaxRegistrars.to_string(),
                    "ConstU32<20>".to_string(),
                ),
                (PalletIdentityTraits::Slashed.to_string(), "()".to_string()),
                (
                    PalletIdentityTraits::ForceOrigin.to_string(),
                    "EnsureRoot<Self::AccountId>".to_string(),
                ),
                (
                    PalletIdentityTraits::RegistrarOrigin.to_string(),
                    "EnsureRoot<Self::AccountId>".to_string(),
                ),
                (
                    PalletIdentityTraits::OffchainSignature.to_string(),
                    "Signature".to_string(),
                ),
                (
                    PalletIdentityTraits::SigningPublicKey.to_string(),
                    "<Signature as sp_runtime::traits::Verify>::Signer".to_string(),
                ),
                (
                    PalletIdentityTraits::UsernameAuthorityOrigin.to_string(),
                    "EnsureRoot<Self::AccountId>".to_string(),
                ),
                (
                    PalletIdentityTraits::PendingUsernameExpiration.to_string(),
                    "ConstU32<{ 7 * DAYS }>".to_string(),
                ),
                (
                    PalletIdentityTraits::MaxSuffixLength.to_string(),
                    "ConstU32<7>".to_string(),
                ),
                (
                    PalletIdentityTraits::MaxUsernameLength.to_string(),
                    "ConstU32<32>".to_string(),
                ),
                (
                    PalletIdentityTraits::WeightInfo.to_string(),
                    "pallet_identity::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: None,
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![String::from(
                "use pallet_identity::legacy::IdentityInfo;",
            )]),
            runtime_api_code: None,
        };

        PalletIdentityConfig {
            name: "Pallet identity".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for PalletIdentityTraits enum display implementation
    #[test]
    fn test_pallet_identity_traits_display() {
        assert_eq!(
            PalletIdentityTraits::RuntimeEvent.to_string(),
            "RuntimeEvent"
        );
        assert_eq!(PalletIdentityTraits::Currency.to_string(), "Currency");
        assert_eq!(
            PalletIdentityTraits::BasicDeposit.to_string(),
            "BasicDeposit"
        );
        assert_eq!(PalletIdentityTraits::ByteDeposit.to_string(), "ByteDeposit");
        assert_eq!(
            PalletIdentityTraits::SubAccountDeposit.to_string(),
            "SubAccountDeposit"
        );
        assert_eq!(
            PalletIdentityTraits::MaxSubAccounts.to_string(),
            "MaxSubAccounts"
        );
        assert_eq!(
            PalletIdentityTraits::IdentityInformation.to_string(),
            "IdentityInformation"
        );
        assert_eq!(
            PalletIdentityTraits::MaxRegistrars.to_string(),
            "MaxRegistrars"
        );
        assert_eq!(PalletIdentityTraits::Slashed.to_string(), "Slashed");
        assert_eq!(PalletIdentityTraits::ForceOrigin.to_string(), "ForceOrigin");
        assert_eq!(
            PalletIdentityTraits::RegistrarOrigin.to_string(),
            "RegistrarOrigin"
        );
        assert_eq!(
            PalletIdentityTraits::OffchainSignature.to_string(),
            "OffchainSignature"
        );
        assert_eq!(
            PalletIdentityTraits::SigningPublicKey.to_string(),
            "SigningPublicKey"
        );
        assert_eq!(
            PalletIdentityTraits::UsernameAuthorityOrigin.to_string(),
            "UsernameAuthorityOrigin"
        );
        assert_eq!(
            PalletIdentityTraits::PendingUsernameExpiration.to_string(),
            "PendingUsernameExpiration"
        );
        assert_eq!(
            PalletIdentityTraits::MaxSuffixLength.to_string(),
            "MaxSuffixLength"
        );
        assert_eq!(
            PalletIdentityTraits::MaxUsernameLength.to_string(),
            "MaxUsernameLength"
        );
        assert_eq!(PalletIdentityTraits::WeightInfo.to_string(), "WeightInfo");
    }

    // Test case for PalletIdentityConfig::new() method
    #[test]
    fn test_pallet_identity_config_new() {
        let pallet_identity_config = PalletIdentityConfig::new();

        // Test the name
        assert_eq!(pallet_identity_config.name, "Pallet identity");

        // Test metadata
        assert_eq!(
            pallet_identity_config.metadata.short_description,
            "FRAME identity pallet"
        );
        assert_eq!(pallet_identity_config.metadata.size, 10500);
        assert_eq!(
            pallet_identity_config.metadata.authors[0],
            CommonAuthors::ParityTechnologies
        );
        assert_eq!(
            pallet_identity_config.metadata.categories.clone().unwrap()[0],
            PalletCategories::Identity
        );
        assert_eq!(
            pallet_identity_config.metadata.license.clone().unwrap(),
            "Apache-2.0"
        );

        // Ensure description matches
        let expected_description = [
            "A federated naming system, allowing for multiple registrars to be added from a specified origin.",
            "Registrars can set a fee to provide identity-verification service. Anyone can put forth a proposed identity for a fixed deposit and ask for review by any number of registrars (paying each of their fees). Registrar judgements are given as an `enum`, allowing for sophisticated, multi-tier opinions."
        ].join("\n");
        assert_eq!(
            pallet_identity_config.metadata.description,
            expected_description
        );

        // Test dependencies
        assert_eq!(
            pallet_identity_config.dependencies.pallet.package,
            "pallet-identity"
        );
        assert_eq!(
            pallet_identity_config.dependencies.pallet.alias,
            "pallet identity"
        );
        assert_eq!(
            pallet_identity_config
                .dependencies
                .pallet
                .git_repo
                .clone()
                .unwrap(),
            "https://github.com/paritytech/polkadot-sdk.git"
        );
        assert_eq!(
            pallet_identity_config
                .dependencies
                .pallet
                .tag
                .clone()
                .unwrap(),
            "polkadot-v1.14.0"
        );

        // Test runtime configuration
        let runtime_traits = &pallet_identity_config.runtime.pallet_traits;
        assert_eq!(runtime_traits.get("RuntimeEvent").unwrap(), "RuntimeEvent");
        assert_eq!(runtime_traits.get("Currency").unwrap(), "Balances");
        assert_eq!(
            runtime_traits.get("BasicDeposit").unwrap(),
            "ConstU128<{ 1 * 1000 }>"
        );
        assert_eq!(
            runtime_traits.get("ByteDeposit").unwrap(),
            "ConstU128<{ 1 * 1000 }>"
        );
        assert_eq!(
            runtime_traits.get("SubAccountDeposit").unwrap(),
            "ConstU128<{ 100 * 1000 }>"
        );
        assert_eq!(
            runtime_traits.get("MaxSubAccounts").unwrap(),
            "ConstU32<100>"
        );
        assert_eq!(
            runtime_traits.get("IdentityInformation").unwrap(),
            "IdentityInfo<ConstU32<100>>"
        );
        assert_eq!(runtime_traits.get("MaxRegistrars").unwrap(), "ConstU32<20>");
        assert_eq!(runtime_traits.get("Slashed").unwrap(), "()");
        assert_eq!(
            runtime_traits.get("ForceOrigin").unwrap(),
            "EnsureRoot<Self::AccountId>"
        );
        assert_eq!(
            runtime_traits.get("RegistrarOrigin").unwrap(),
            "EnsureRoot<Self::AccountId>"
        );
        assert_eq!(
            runtime_traits.get("OffchainSignature").unwrap(),
            "Signature"
        );
        assert_eq!(
            runtime_traits.get("SigningPublicKey").unwrap(),
            "<Signature as sp_runtime::traits::Verify>::Signer"
        );
        assert_eq!(
            runtime_traits.get("UsernameAuthorityOrigin").unwrap(),
            "EnsureRoot<Self::AccountId>"
        );
        assert_eq!(
            runtime_traits.get("PendingUsernameExpiration").unwrap(),
            "ConstU32<{ 7 * DAYS }>"
        );
        assert_eq!(
            runtime_traits.get("MaxSuffixLength").unwrap(),
            "ConstU32<7>"
        );
        assert_eq!(
            runtime_traits.get("MaxUsernameLength").unwrap(),
            "ConstU32<32>"
        );
        assert_eq!(
            runtime_traits.get("WeightInfo").unwrap(),
            "pallet_identity::weights::SubstrateWeight<Runtime>"
        );

        // Test runtime construct configuration
        assert_eq!(
            pallet_identity_config
                .runtime
                .construct_runtime
                .index
                .unwrap(),
            9
        );
        assert_eq!(
            pallet_identity_config.runtime.construct_runtime.runtime.0,
            "Identity"
        );
        assert_eq!(
            pallet_identity_config.runtime.construct_runtime.runtime.1,
            "pallet_identity::Pallet<Runtime>"
        );

        // Test additional_runtime_lib_code
        let additional_runtime_lib_code = pallet_identity_config
            .runtime
            .additional_runtime_lib_code
            .clone()
            .unwrap();
        assert_eq!(
            additional_runtime_lib_code[0],
            "use pallet_identity::legacy::IdentityInfo;"
        );
    }
}
