use super::super::pallet_index::PROXY;
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

impl Default for PalletProxyConfig {
    fn default() -> Self {
        PalletProxyConfig::new()
    }
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
                index: Some(PROXY),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pallet_proxy_traits_display() {
        assert_eq!(PalletProxyTraits::RuntimeEvent.to_string(), "RuntimeEvent");
        assert_eq!(PalletProxyTraits::RuntimeCall.to_string(), "RuntimeCall");
        assert_eq!(PalletProxyTraits::Currency.to_string(), "Currency");
        assert_eq!(PalletProxyTraits::ProxyType.to_string(), "ProxyType");
        assert_eq!(
            PalletProxyTraits::ProxyDepositBase.to_string(),
            "ProxyDepositBase"
        );
        assert_eq!(
            PalletProxyTraits::ProxyDepositFactor.to_string(),
            "ProxyDepositFactor"
        );
        assert_eq!(PalletProxyTraits::MaxProxies.to_string(), "MaxProxies");
        assert_eq!(PalletProxyTraits::MaxPending.to_string(), "MaxPending");
        assert_eq!(PalletProxyTraits::CallHasher.to_string(), "CallHasher");
        assert_eq!(
            PalletProxyTraits::AnnouncementDepositBase.to_string(),
            "AnnouncementDepositBase"
        );
        assert_eq!(
            PalletProxyTraits::AnnouncementDepositFactor.to_string(),
            "AnnouncementDepositFactor"
        );
        assert_eq!(PalletProxyTraits::WeightInfo.to_string(), "WeightInfo");
    }

    // Test case for PalletProxyConfig::new() method
    #[test]
    fn test_pallet_proxy_config_new() {
        let pallet_proxy_config = PalletProxyConfig::new();

        // Test the name
        assert_eq!(pallet_proxy_config.name, "Pallet proxy");

        // Test metadata
        assert_eq!(
            pallet_proxy_config.metadata.short_description,
            "FRAME proxy pallet"
        );
        assert_eq!(pallet_proxy_config.metadata.size, 10500);
        assert_eq!(
            pallet_proxy_config.metadata.authors[0],
            CommonAuthors::ParityTechnologies
        );
        assert_eq!(
            pallet_proxy_config.metadata.categories.clone().unwrap()[0],
            PalletCategories::Runtime
        );
        assert_eq!(
            pallet_proxy_config.metadata.license.clone().unwrap(),
            "Apache-2.0"
        );

        // Ensure description matches
        let expected_description = [
            "A pallet allowing accounts to give permission to other accounts to dispatch types of calls from their signed origin."
        ].join("\n");
        assert_eq!(
            pallet_proxy_config.metadata.description,
            expected_description
        );

        // Test dependencies
        assert_eq!(
            pallet_proxy_config.dependencies.pallet.package,
            "pallet-proxy"
        );
        assert_eq!(
            pallet_proxy_config.dependencies.pallet.alias,
            "pallet proxy"
        );
        assert_eq!(
            pallet_proxy_config
                .dependencies
                .pallet
                .git_repo
                .clone()
                .unwrap(),
            "https://github.com/paritytech/polkadot-sdk.git"
        );
        assert_eq!(
            pallet_proxy_config.dependencies.pallet.tag.clone().unwrap(),
            "polkadot-v1.14.0"
        );

        // Test runtime configuration
        let runtime_traits = &pallet_proxy_config.runtime.pallet_traits;
        assert_eq!(runtime_traits.get("RuntimeEvent").unwrap(), "RuntimeEvent");
        assert_eq!(runtime_traits.get("RuntimeCall").unwrap(), "RuntimeCall");
        assert_eq!(runtime_traits.get("Currency").unwrap(), "Balances");
        assert_eq!(runtime_traits.get("ProxyType").unwrap(), "ProxyType");
        assert_eq!(
            runtime_traits.get("ProxyDepositBase").unwrap(),
            "ProxyDepositBase"
        );
        assert_eq!(
            runtime_traits.get("ProxyDepositFactor").unwrap(),
            "ProxyDepositFactor"
        );
        assert_eq!(runtime_traits.get("MaxProxies").unwrap(), "ConstU32<32>");
        assert_eq!(runtime_traits.get("MaxPending").unwrap(), "ConstU32<32>");
        assert_eq!(runtime_traits.get("CallHasher").unwrap(), "BlakeTwo256");
        assert_eq!(
            runtime_traits.get("AnnouncementDepositBase").unwrap(),
            "AnnouncementDepositBase"
        );
        assert_eq!(
            runtime_traits.get("AnnouncementDepositFactor").unwrap(),
            "AnnouncementDepositFactor"
        );
        assert_eq!(
            runtime_traits.get("WeightInfo").unwrap(),
            "pallet_proxy::weights::SubstrateWeight<Runtime>"
        );

        // Test runtime construct configuration
        assert_eq!(
            pallet_proxy_config.runtime.construct_runtime.index.unwrap(),
            11
        );
        assert_eq!(
            pallet_proxy_config.runtime.construct_runtime.runtime.0,
            "Proxy"
        );
        assert_eq!(
            pallet_proxy_config.runtime.construct_runtime.runtime.1,
            "pallet_proxy::Pallet<Runtime>"
        );
    }

    // Test case for additional runtime library code
    #[test]
    fn test_pallet_proxy_additional_runtime_lib_code() {
        let pallet_proxy_config = PalletProxyConfig::new();
        let additional_runtime_lib_code = pallet_proxy_config
            .runtime
            .additional_runtime_lib_code
            .clone()
            .unwrap();

        assert!(additional_runtime_lib_code
            .contains(&String::from("use codec::{Decode, Encode, MaxEncodedLen};")));
        assert!(additional_runtime_lib_code
            .contains(&String::from("use frame_support::traits::InstanceFilter;")));
        assert!(
            additional_runtime_lib_code.contains(&String::from("use sp_runtime::RuntimeDebug;"))
        );
    }

    // Test case for additional implementation code
    #[test]
    fn test_pallet_proxy_additional_implementation_code() {
        let additional_implementation_code = get_additional_implementation_code();

        assert!(additional_implementation_code
            .contains("pub const ProxyDepositBase: Balance = deposit(1, 8);"));
        assert!(additional_implementation_code
            .contains("pub const ProxyDepositFactor: Balance = deposit(0, 33);"));
        assert!(additional_implementation_code
            .contains("pub const AnnouncementDepositBase: Balance = deposit(1, 8);"));
        assert!(additional_implementation_code
            .contains("pub const AnnouncementDepositFactor: Balance = deposit(0, 66);"));

        assert!(additional_implementation_code.contains("impl Default for ProxyType {"));
        assert!(additional_implementation_code.contains("fn default() -> Self {"));
        assert!(additional_implementation_code
            .contains("impl InstanceFilter<RuntimeCall> for ProxyType {"));
    }
}
