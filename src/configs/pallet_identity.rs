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
            categories: Some(vec![PalletCategories::Runtime]),
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
                index: Some(9),
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
                    "1 * 1000".to_string(),
                ),
                (
                    PalletIdentityTraits::ByteDeposit.to_string(),
                    "1000".to_string(),
                ),
                (
                    PalletIdentityTraits::SubAccountDeposit.to_string(),
                    "100 * 1000".to_string(),
                ),
                (
                    PalletIdentityTraits::MaxSubAccounts.to_string(),
                    "100".to_string(),
                ),
                (
                    PalletIdentityTraits::IdentityInformation.to_string(),
                    "IdentityInfo<100>".to_string(),
                ),
                (
                    PalletIdentityTraits::MaxRegistrars.to_string(),
                    "20".to_string(),
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
                    "ConstU32<{ 7 * DAYS }>".to_string(),
                ),
                (
                    PalletIdentityTraits::PendingUsernameExpiration.to_string(),
                    "EnsureRoot<Self::AccountId>".to_string(),
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
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use pallet_identity::legacy::IdentityInfo;"),
                String::from("use frame_system::EnsureRoot;"),
            ]),
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