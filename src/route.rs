use crate::types::{ESupportedPallets, PalletCategories, PalletConfig};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// Pallet structure that will be returned as JSON
#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Pallet {
    name: String,
    description: String,
    category: String,
}

// Chain use case structure that will be returned as JSON
#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct UseCase {
    name: String,
    description: String,
    pallets: Vec<ESupportedPallets>,
}

// Blockchain template structure.
#[derive(Serialize, PartialEq, Eq, Debug)]
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

pub async fn get_templates(
    pallet_configs: Data<Vec<PalletConfig>>,
    query: web::Query<TemplateQuery>,
) -> impl Responder {
    let templates = get_templates_internal(pallet_configs.to_vec(), &query.template_type).await;
    HttpResponse::Ok().json(templates)
}

pub async fn get_templates_internal(
    pallet_configs: Vec<PalletConfig>,
    query_template_type: &Option<String>,
) -> Vec<BlockchainTemplate> {
    let templates: Vec<BlockchainTemplate> = vec![
        BlockchainTemplate {
            template_type: String::from("solochain"),
            essential_pallets: pallet_configs
                .iter()
                .filter(|pallet| pallet.metadata.is_essential)
                .map(|pallet| Pallet {
                    name: pallet.name.clone(),
                    description: pallet.metadata.description.clone(),
                    category: "Core".to_string(),
                })
                .collect::<Vec<_>>(),
            supported_pallets: pallet_configs
                .iter()
                .filter(|pallet| !pallet.metadata.is_essential)
                .map(|pallet| Pallet {
                    name: pallet.name.clone(),
                    description: pallet.metadata.description.clone(),
                    category: pallet
                        .metadata
                        .categories
                        .as_ref()
                        .unwrap_or(&vec![PalletCategories::Runtime])[0]
                        .to_string(),
                })
                .collect::<Vec<_>>(),
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
    let filtered_templates: Vec<BlockchainTemplate> = match query_template_type {
        Some(template_type) => templates
            .into_iter()
            .filter(|t| t.template_type == *template_type)
            .collect(),
        None => templates,
    };

    // Return JSON response
    filtered_templates
}

pub async fn get_templates_depricated(
    pallet_configs: Data<Vec<PalletConfig>>,
    query: web::Query<TemplateQuery>,
) -> impl Responder {
    let templates =
        get_templates_depricated_internal(pallet_configs.to_vec(), &query.template_type).await;
    HttpResponse::Ok().json(templates)
}
// Function that returns JSON based on the query parameter for templates.
pub async fn get_templates_depricated_internal(
    pallet_configs: Vec<PalletConfig>,
    query_template_type: &Option<String>,
) -> Vec<BlockchainTemplate> {
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
                    description: get_config(pallet_configs.to_vec(), "Pallet assets"),
                    category: String::from("Asset"),
                },
                Pallet {
                    name: String::from("Treasury"),
                    description: get_config(pallet_configs.to_vec(), "Pallet treasury"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Utility"),
                    description: get_config(pallet_configs.to_vec(), "Pallet utility"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Vesting"),
                    description: get_config(pallet_configs.to_vec(), "Pallet vesting"),
                    category: String::from("Asset"),
                },
                Pallet {
                    name: String::from("Uniques"),
                    description: get_config(pallet_configs.to_vec(), "Pallet uniques"),
                    category: String::from("Nfts"),
                },
                Pallet {
                    name: String::from("Nfts"),
                    description: get_config(pallet_configs.to_vec(), "Pallet nfts"),
                    category: String::from("Nfts"),
                },
                Pallet {
                    name: String::from("Society"),
                    description: get_config(pallet_configs.to_vec(), "Pallet society"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Proxy"),
                    description: get_config(pallet_configs.to_vec(), "Pallet proxy"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Multisig"),
                    description: get_config(pallet_configs.to_vec(), "Pallet multisig"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Identity"),
                    description: get_config(pallet_configs.to_vec(), "Pallet identity"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Scheduler"),
                    description: get_config(pallet_configs.to_vec(), "Pallet Scheduler"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Membership"),
                    description: get_config(pallet_configs.to_vec(), "Pallet membership"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Bounties"),
                    description: get_config(pallet_configs.to_vec(), "Pallet bounties"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Child Bounties"),
                    description: get_config(pallet_configs.to_vec(), "Pallet child bounties"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Collective"),
                    description: get_config(pallet_configs.to_vec(), "Pallet collective"),
                    category: String::from("Core"),
                },
                Pallet {
                    name: String::from("Democracy"),
                    description: get_config(pallet_configs.to_vec(), "Pallet Democracy"),
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
    let filtered_templates: Vec<BlockchainTemplate> = match query_template_type {
        Some(template_type) => templates
            .into_iter()
            .filter(|t| t.template_type == *template_type)
            .collect(),
        None => templates,
    };

    // Return JSON response
    filtered_templates
}

pub fn get_config(pallet_configs: Vec<PalletConfig>, pallet: &str) -> String {
    if let Some(assets_config) = pallet_configs.iter().find(|config| config.name == pallet) {
        assets_config.metadata.description.to_string()
    } else {
        "Polkadot Frame Pallet".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::code_generator::get_pallet_configs;

    use super::*;
    use strum::IntoEnumIterator;

    #[actix_rt::test]
    async fn test_get_templates() {
        let supported_configs = ESupportedPallets::iter().collect::<Vec<_>>();
        let toml_pallet_configs = get_pallet_configs(supported_configs).unwrap();
        let response =
            get_templates_internal(toml_pallet_configs.clone(), &Some("solochain".to_string()))
                .await;
        let response_deprecated =
            get_templates_depricated_internal(toml_pallet_configs, &Some("solochain".to_string()))
                .await;
        assert_eq!(response.len(), 1);
        assert_eq!(response_deprecated.len(), 1);
        let template = &response[0];
        let template_deprecated = &response_deprecated[0];
        assert_eq!(template.template_type, "solochain");
        assert_eq!(template_deprecated.template_type, "solochain");
        assert_eq!(template.essential_pallets.len(), 6);
        assert_eq!(template_deprecated.essential_pallets.len(), 6);
        assert_eq!(template.supported_pallets.len(), 16);
        assert_eq!(template_deprecated.supported_pallets.len(), 16);
        let supported_pallet_names = template
            .supported_pallets
            .iter()
            .map(|p| p.name.split(' ').collect::<Vec<_>>()[1].to_lowercase())
            .collect::<Vec<_>>();

        template_deprecated.supported_pallets.iter().for_each(|p| {
            let name = p.name.split(' ').collect::<Vec<_>>()[0].to_lowercase();
            assert!(supported_pallet_names.contains(&name));
        });
    }
}
