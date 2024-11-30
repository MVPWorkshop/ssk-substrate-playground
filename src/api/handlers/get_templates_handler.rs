use std::collections::HashMap;

use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};

use crate::services::code_generator::types::{ PalletConfig,TemplateType};

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
    // let template_types = vec![
    //     TemplateType::SoloChain,
    //     TemplateType::SoloFrontierChain,
    //     TemplateType::ParaChain,
    //     TemplateType::Minimal,
    //     TemplateType::Frontier,
    // ];
    let mut template_types: Vec<TemplateType> = pallet_configs
        .values()
        .flat_map(|pallet| pallet.metadata.supported_template.clone())
        .collect();
    template_types.sort();
    template_types.dedup();

    let templates: Vec<BlockchainTemplate> = template_types
    .into_iter()
    .map(|template_type| BlockchainTemplate {
        template_type: template_type.clone(),
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
            .filter(|(_, pallet)| {
                !pallet.metadata.is_essential
                    && pallet.metadata.supported_template.contains(&template_type)
            })
            .map(|(name, pallet)| Pallet {
                name: name.clone(),
                description: pallet.metadata.description.clone(),
                category: pallet
                    .metadata
                    .categorie
                    .as_ref()
                    .map_or_else(|| "Uncategorized".to_string(), |cat| cat.to_string()),
            })
            .collect::<Vec<_>>(),
        use_cases: vec![], 
        chain_type: vec![], 
    })
    .collect();
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
