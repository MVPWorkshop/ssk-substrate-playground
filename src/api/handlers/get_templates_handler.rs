use std::collections::HashMap;

use poem_openapi::{param::Path, payload::Json, ApiResponse, Enum, Object};

use crate::services::code_generator::types::{PalletCategories, PalletConfig};

// Pallet structure that will be returned as JSON
#[derive(PartialEq, Eq, Debug, Object)]
pub struct Pallet {
    name: String,
    description: String,
    category: String,
}

// Chain use case structure that will be returned as JSON
#[derive(PartialEq, Eq, Debug, Object)]
pub struct UseCase {
    name: String,
    description: String,
    pallets: Vec<String>,
}

#[derive(PartialEq, Eq, Debug, Enum)]
pub enum TemplateType {
    SoloChain,
    ParaChain,
}

// Blockchain template structure.
#[derive(PartialEq, Eq, Debug, Object)]
pub struct BlockchainTemplate {
    template_type: TemplateType,
    essential_pallets: Vec<Pallet>,
    supported_pallets: Vec<Pallet>,
    use_cases: Vec<UseCase>,
    chain_type: Vec<UseCase>,
}

#[derive(ApiResponse)]
pub enum GetTemplatesResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<Vec<BlockchainTemplate>>),
}

pub async fn get_templates_handler(
    pallet_configs: &HashMap<String, PalletConfig>,
    query_template_type: Path<Option<TemplateType>>,
) -> GetTemplatesResponse {
    let templates: Vec<BlockchainTemplate> = vec![
        BlockchainTemplate {
            template_type: TemplateType::SoloChain,
            essential_pallets: pallet_configs
                .iter()
                .filter(|(_, pallet)| pallet.metadata.is_essential)
                .map(|(name, pallet)| Pallet {
                    name: name.clone(),
                    description: pallet.metadata.description.clone(),
                    category: "Core".to_string(),
                })
                .collect::<Vec<_>>(),
            supported_pallets: pallet_configs
                .iter()
                .filter(|(_, pallet)| !pallet.metadata.is_essential)
                .map(|(name, pallet)| Pallet {
                    name: name.clone(),
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
            template_type: TemplateType::ParaChain,
            essential_pallets: vec![],
            supported_pallets: vec![],
            use_cases: vec![],
            chain_type: vec![],
        },
    ];
    // Filtering the templates based on the `template_type` query parameter
    let filtered_templates: Vec<BlockchainTemplate> = match &query_template_type.0 {
        Some(template_type) => templates
            .into_iter()
            .filter(|t| t.template_type == *template_type)
            .collect(),
        _ => templates,
    };

    // Return JSON response
    GetTemplatesResponse::Ok(Json(filtered_templates))
}
