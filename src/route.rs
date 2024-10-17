use crate::code_generator::get_pallet_configs;
use crate::types::{ESupportedPallets, PalletConfig};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

// Pallet structure that will be returned as JSON
#[derive(Serialize)]
pub struct Pallet {
    name: String,
    description: String,
    category: String,
}

// Chain use case structure that will be returned as JSON
#[derive(Serialize)]
pub struct UseCase {
    name: String,
    description: String,
    pallets: Vec<ESupportedPallets>,
}

// Blockchain template structure.
#[derive(Serialize)]
pub struct BlockchainTemplate {
    template_type: String,
    essential_pallets: Vec<Pallet>,
    supported_pallets: Vec<Pallet>,
    use_cases: Vec<UseCase>,
    chain_type: Vec<UseCase>,
}

// Query structure for extracting query parameters from the URL
#[derive(Deserialize)]
pub struct TemplateQuery {
    template_type: Option<String>, // Query parameter
}

// Function that returns JSON based on the query parameter for templates.
pub async fn get_templates(query: web::Query<TemplateQuery>) -> impl Responder {
    let pallets: Vec<_> = ESupportedPallets::iter().collect();
    // TODO: handle error case
    let pallet_configs = get_pallet_configs(pallets).unwrap();

    let templates = vec![
        BlockchainTemplate {
            template_type: String::from("solochain"),
            essential_pallets: vec![
                Pallet {
                    name: String::from("Balances"),
                    description: String::from("The Balances pallet provides functionality for handling accounts and balances for a single token."),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Timestamp"),
                    description: String::from("The Timestamp pallet is designed to create a consensus-based time source. This helps ensure that nodes maintain a synchronized view of time that all network participants can agree on."),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Transaction-Payment"),
                    description: String::from("This pallet provides the basic logic needed to pay the absolute minimum amount needed for a transaction to be included."),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Aura"),
                    description: String::from("The Aura module extends Aura consensus by managing offline reporting."),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Grandpa"),
                    description: String::from("This manages the GRANDPA authority set ready for the native code."),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Sudo"),
                    description: String::from("A pallet to provide a way to execute privileged runtime calls using a specified sudo (“superuser do”) account."),
                    category: String::from("Core"),
                },
            ],
            supported_pallets: vec![
                Pallet {
                    name: String::from("Assets"),
                    description: get_config(pallet_configs.clone(), "Pallet assets"),
                    category: String::from("Asset"),
                },
                Pallet {
                    name: String::from("Treasury"),
                    description: get_config(pallet_configs.clone(), "Pallet treasury"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Utility"),
                    description: get_config(pallet_configs.clone(), "Pallet utility"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Vesting"),
                    description: get_config(pallet_configs.clone(), "Pallet vesting"),
                    category: String::from("Asset"),
                },
                Pallet {
                    name: String::from("Uniques"),
                    description: get_config(pallet_configs.clone(), "Pallet uniques"),
                    category: String::from("Nfts"),
                },
                Pallet {
                    name: String::from("Nfts"),
                    description: get_config(pallet_configs.clone(), "Pallet nfts"),
                    category: String::from("Nfts"),
                },
                Pallet {
                    name: String::from("Society"),
                    description: get_config(pallet_configs.clone(), "Pallet society"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Proxy"),
                    description: get_config(pallet_configs.clone(), "Pallet proxy"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Multisig"),
                    description: get_config(pallet_configs.clone(), "Pallet multisig"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Identity"),
                    description: get_config(pallet_configs.clone(), "Pallet identity"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Scheduler"),
                    description: get_config(pallet_configs.clone(), "Pallet scheduler"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Membership"),
                    description: get_config(pallet_configs.clone(), "Pallet membership"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Bounties"),
                    description: get_config(pallet_configs.clone(), "Pallet bounties"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Child Bounties"),
                    description: get_config(pallet_configs.clone(), "Pallet child bounties"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Collective"),
                    description: get_config(pallet_configs.clone(), "Pallet collective"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Democracy"),
                    description: get_config(pallet_configs, "Pallet democracy"),
                    category: String::from("Core"),
                },
            ],
            use_cases: vec![],
            chain_type: vec![],
        },
        BlockchainTemplate {
            template_type: String::from("parachain"),
            essential_pallets: vec![],
            supported_pallets: vec![],
            use_cases: vec![],
            chain_type: vec![],
        },
    ];

    // Filtering the templates based on the `template_type` query parameter
    let filtered_templates: Vec<BlockchainTemplate> = match &query.template_type {
        Some(template_type) if template_type == "solochain" => templates
            .into_iter()
            .filter(|t| t.template_type == "solochain")
            .collect(),
        Some(template_type) if template_type == "parachain" => templates
            .into_iter()
            .filter(|t| t.template_type == "parachain")
            .collect(),
        _ => templates,
    };

    // Return JSON response
    HttpResponse::Ok().json(filtered_templates)
}

pub fn get_config(pallet_configs: Vec<PalletConfig>, pallet: &str) -> String {
    if let Some(assets_config) = pallet_configs.iter().find(|config| config.name == pallet) {
        assets_config.metadata.description.to_string()
    } else {
        "Polkadot Frame Pallet".to_string()
    }
}
