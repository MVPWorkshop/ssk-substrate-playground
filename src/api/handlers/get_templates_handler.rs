use std::collections::HashMap;

use poem_openapi::{param::Path, payload::Json, ApiResponse, Object};

use crate::services::code_generator::types::{PalletConfig, TemplateType};

// Pallet structure that will be returned as JSON
#[derive(PartialEq, Eq, Debug, Object)]
pub struct Pallet {
    pub name: String,
    pub description: String,
    pub category: String,
}

// Chain use case structure that will be returned as JSON
#[derive(PartialEq, Eq, Debug, Object)]
pub struct UseCase {
    name: String,
    description: String,
    pallets: Vec<String>,
}

impl From<(&HashMap<String, PalletConfig>, String)> for UseCase {
    fn from(value: (&HashMap<String, PalletConfig>, String)) -> Self {
        let (pallet_configs, use_case) = value;

        let use_cases = pallet_configs
            .iter()
            .filter_map(|(name, config)| match &config.metadata.use_cases {
                None => None,
                Some(config_use_cases) => {
                    if config_use_cases.contains(&use_case) {
                        Some(name.clone())
                    } else {
                        None
                    }
                }
            })
            .collect();

        UseCase {
            name: use_case.clone(),
            description: format!("Default pallets for use case {}", use_case),
            pallets: use_cases,
        }
    }
}

// Blockchain template structure.
#[derive(PartialEq, Eq, Debug, Object)]
pub struct BlockchainTemplate {
    pub template_type: TemplateType,
    pub essential_pallets: Vec<Pallet>,
    pub supported_pallets: Vec<Pallet>,
    pub use_cases: Vec<UseCase>,
    pub chain_type: Vec<UseCase>,
}

#[derive(ApiResponse)]
pub enum GetTemplatesResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<BlockchainTemplate>),
    #[oai(status = 404)]
    NotFound(Json<String>),
}

pub async fn get_templates_handler(
    pallet_configs: &HashMap<String, PalletConfig>,
    query_template_type: Path<TemplateType>,
    supported_templates: Vec<TemplateType>,
) -> GetTemplatesResponse {
    if !supported_templates.contains(&query_template_type.0) {
        return GetTemplatesResponse::NotFound(Json(format!(
            "Template {:?} is not supported.",
            &query_template_type.0
        )));
    }
    let blockchain_supported_pallets: Vec<Pallet> = pallet_configs
        .iter()
        .filter(|(_, pallet)| {
            pallet
                .metadata
                .supported_template
                .contains(&query_template_type.0)
        })
        .map(|(name, pallet)| Pallet {
            name: name.clone(),
            description: pallet.metadata.description.clone(),
            category: pallet
                .metadata
                .category
                .as_ref()
                .map_or_else(|| "Uncategorized".to_string(), |cat| cat.to_string()),
        })
        .collect();
    let blockchain_essential_templates: Vec<Pallet> = pallet_configs
        .iter()
        .filter(|(_, pallet)| {
            pallet
                .metadata
                .is_essential
                .as_ref()
                .is_some_and(|essential_templates| {
                    essential_templates.contains(&query_template_type.0)
                })
        })
        .map(|(name, pallet)| Pallet {
            name: name.clone(),
            description: pallet.metadata.description.clone(),
            category: pallet
                .metadata
                .category
                .as_ref()
                .map_or_else(|| "Uncategorized".to_string(), |cat| cat.to_string()),
        })
        .collect();

    let use_cases = vec![
        (pallet_configs, "Gaming".to_string()).into(),
        (pallet_configs, "NFT".to_string()).into(),
        (pallet_configs, "Governance".to_string()).into(),
        (pallet_configs, "DeFi".to_string()).into(),
        (pallet_configs, "SupplyChain".to_string()).into(),
    ];

    GetTemplatesResponse::Ok(Json(BlockchainTemplate {
        template_type: query_template_type.0,
        essential_pallets: blockchain_essential_templates,
        supported_pallets: blockchain_supported_pallets,
        use_cases,
        chain_type: vec![],
    }))
}
