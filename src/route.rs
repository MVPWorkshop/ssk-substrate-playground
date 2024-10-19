use crate::types::{PalletCategories, PalletConfig};
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
    pallets: Vec<String>,
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

pub fn get_config(pallet_configs: Vec<PalletConfig>, pallet: &str) -> String {
    if let Some(assets_config) = pallet_configs.iter().find(|config| config.name == pallet) {
        assets_config.metadata.description.to_string()
    } else {
        "Polkadot Frame Pallet".to_string()
    }
}

#[cfg(test)]
mod tests {}
