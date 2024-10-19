use super::super::types::*;

#[derive(Debug, Clone, Copy)]
pub enum PalletSudoTraits {
    RuntimeEvent,
    RuntimeCall,
    WeightInfo,
}

#[derive(Debug, Clone)]
pub struct PalletSudoConfig {
    pub name: String,
    pub metadata: PalletMetadata,
    pub runtime: PalletRuntimeConfig,
    pub dependencies: PalletDependencyConfig,
}

impl Default for PalletSudoConfig {
    fn default() -> Self {
        PalletSudoConfig::new()
    }
}

impl PalletSudoConfig {
    pub fn new() -> Self {
        let pallet_description: &str = "A pallet to provide a way to execute privileged runtime calls using a specified sudo (“superuser do”) account.";

        PalletSudoConfig {
            name: String::from("pallet_sudo"),
            metadata: PalletMetadata {
                size: 5078,
                is_essential: true,
                license: Some(String::from("Apache-2.0")),
                compatibility: SubstrateVersion::Two,
                authors: vec![CommonAuthors::ParityTechnologies],
                categories: Some(vec![PalletCategories::Consensus]),
                short_description: String::from("FRAME Sudo consensus pallet"),
                description: String::from(pallet_description),
            },
            runtime: PalletRuntimeConfig {
                pallet_traits: vec![
                    (String::from("RuntimeEvent"), String::from("RuntimeEvent")),
                    (String::from("RuntimeCall"), String::from("RuntimeCall")),
                    (
                        String::from("WeightInfo"),
                        String::from("pallet_sudo::weights::SubstrateWeight<Runtime>"),
                    ),
                ]
                .into_iter()
                .collect(),
                additional_pallet_impl_code: None,
                additional_runtime_lib_code: None,
                construct_runtime: PalletConstructRuntimeConfig {
                    index: None,
                    runtime: ("Sudo".to_string(), "pallet_sudo".to_string()),
                },
                genesis_config: Some(PalletGenesisConfig {
                    config_struct_name: String::from("sudo"),
                    struct_fields: vec![("key".to_string(), "Some(root_key)".to_string())]
                        .into_iter()
                        .collect(),
                }),
                additional_chain_spec_code: None,
                runtime_api_code: None,
            },
            dependencies: PalletDependencyConfig {
                pallet: CargoComplexDependency {
                    package: String::from("pallet-sudo"),
                    version: None,
                    alias: String::from("pallet-sudo"),
                    default_features: false,
                    git_repo: Some("https://github.com/paritytech/polkadot-sdk.git".to_string()),
                    tag: Some("polkadot-v1.14.0".to_string()),
                    branch: None,
                },
                additional_pallets: None,
                additional_deps: None,
            },
        }
    }
}
