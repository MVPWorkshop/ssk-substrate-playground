use super::super::types::*;
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PalletProxyTraits {
    RuntimeEvent,
    RuntimeCall,
    Currency,
    ProxyType,
    ProxyDepositBase,
    ProxyDepositFactor,
    MaxProxies,
    MaxPending,
    CallHasher,
    AnnouncementDepositBase,
    AnnouncementDepositFactor,
    WeightInfo,
}

impl fmt::Display for PalletProxyTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PalletProxyTraits::RuntimeEvent => write!(f, "RuntimeEvent"),
            PalletProxyTraits::RuntimeCall => write!(f, "RuntimeCall"),
            PalletProxyTraits::Currency => write!(f, "Currency"),
            PalletProxyTraits::ProxyType => write!(f, "ProxyType"),
            PalletProxyTraits::ProxyDepositBase => write!(f, "ProxyDepositBase"),
            PalletProxyTraits::ProxyDepositFactor => write!(f, "ProxyDepositFactor"),
            PalletProxyTraits::MaxProxies => write!(f, "MaxProxies"),
            PalletProxyTraits::MaxPending => write!(f, "MaxPending"),
            PalletProxyTraits::CallHasher => write!(f, "CallHasher"),
            PalletProxyTraits::AnnouncementDepositBase => write!(f, "AnnouncementDepositBase"),
            PalletProxyTraits::AnnouncementDepositFactor => write!(f, "AnnouncementDepositFactor"),
            PalletProxyTraits::WeightInfo => write!(f, "WeightInfo"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PalletProxyConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl PalletProxyConfig {
    pub fn new() -> Self {
        let pallet_description = [
            "A pallet allowing accounts to give permission to other accounts to dispatch types of calls from their signed origin.",
            ].join("\n");

        let metadata = PalletMetadata {
            description: pallet_description,
            short_description: "FRAME proxy pallet".to_string(),
            compatibility: SubstrateVersion::Two,
            size: 10500,
            updated: Utc::now().timestamp().to_string(),
            license: Some("Apache-2.0".to_string()),
            authors: vec![CommonAuthors::ParityTechnologies],
            categories: Some(vec![PalletCategories::Runtime]),
        };

        let dependencies = PalletDependencyConfig {
            pallet: CargoComplexDependency {
                package: "pallet-proxy".to_string(),
                version: None,
                alias: "pallet proxy".to_string(),
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
                index: Some(11),
                runtime: (
                    "Proxy".to_string(),
                    "pallet_proxy::Pallet<Runtime>".to_string(),
                ),
            },
            pallet_traits: vec![
                (
                    PalletProxyTraits::RuntimeEvent.to_string(),
                    "RuntimeEvent".to_string(),
                ),
                (
                    PalletProxyTraits::RuntimeCall.to_string(),
                    "RuntimeCall".to_string(),
                ),
                (
                    PalletProxyTraits::Currency.to_string(),
                    "Balances".to_string(),
                ),
                (
                    PalletProxyTraits::ProxyType.to_string(),
                    "ProxyType".to_string(),
                ),
                (
                    PalletProxyTraits::ProxyDepositBase.to_string(),
                    "ProxyDepositBase".to_string(),
                ),
                (
                    PalletProxyTraits::ProxyDepositFactor.to_string(),
                    "ProxyDepositFactor".to_string(),
                ),
                (
                    PalletProxyTraits::MaxProxies.to_string(),
                    "ConstU32<32>".to_string(),
                ),
                (
                    PalletProxyTraits::MaxPending.to_string(),
                    "ConstU32<32>".to_string(),
                ),
                (
                    PalletProxyTraits::CallHasher.to_string(),
                    "BlakeTwo256".to_string(),
                ),
                (
                    PalletProxyTraits::AnnouncementDepositBase.to_string(),
                    "AnnouncementDepositBase".to_string(),
                ),
                (
                    PalletProxyTraits::AnnouncementDepositFactor.to_string(),
                    "AnnouncementDepositFactor".to_string(),
                ),
                (
                    PalletProxyTraits::WeightInfo.to_string(),
                    "pallet_proxy::weights::SubstrateWeight<Runtime>".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            additional_pallet_impl_code: Some(get_additional_implementation_code()),
            genesis_config: None,
            additional_chain_spec_code: None,
            additional_runtime_lib_code: Some(vec![
                String::from("use codec::{Decode, Encode, MaxEncodedLen};"),
                String::from("use frame_support::traits::InstanceFilter;"),
                String::from("use sp_runtime::RuntimeDebug;"),
            ]),
            runtime_api_code: None,
        };

        PalletProxyConfig {
            name: "Pallet proxy".to_string(),
            metadata,
            runtime,
            dependencies,
        }
    }
}

fn get_additional_implementation_code() -> String {
    "
parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33);
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, _c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
		}
	}
}
"
    .to_string()
}
