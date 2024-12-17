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
                .map_or(false, |essential_templates| {
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

    let gaming_name = "Gaming".to_string();
    let gaming_desc = "Default pallets necessary for Gaming use case".to_string();
    let gaming_pallets = get_use_case_pallets(
        pallet_configs,
        &query_template_type,
        gaming_name,
        gaming_desc,
    );

    let defi_name = "DeFi".to_string();
    let defi_desc = "Default pallets necessary for DeFi use case".to_string();
    let defi_pallets =
        get_use_case_pallets(pallet_configs, &query_template_type, defi_name, defi_desc);

    let nft_name = "NFT".to_string();
    let nft_desc = "Default pallets necessary for NFT use case".to_string();
    let nft_pallets =
        get_use_case_pallets(pallet_configs, &query_template_type, nft_name, nft_desc);

    let gov_name = "Governance".to_string();
    let gov_desc = "Default pallets necessary for Governance use case".to_string();
    let gov_pallets =
        get_use_case_pallets(pallet_configs, &query_template_type, gov_name, gov_desc);

    let supply_name = "SupplyChain".to_string();
    let supply_desc = "Default pallets necessary for Supply Chain use case".to_string();
    let supply_pallets = get_use_case_pallets(
        pallet_configs,
        &query_template_type,
        supply_name,
        supply_desc,
    );

    GetTemplatesResponse::Ok(Json(BlockchainTemplate {
        template_type: query_template_type.0,
        essential_pallets: blockchain_essential_templates,
        supported_pallets: blockchain_supported_pallets,
        use_cases: vec![
            gaming_pallets,
            defi_pallets,
            nft_pallets,
            gov_pallets,
            supply_pallets,
        ],
        chain_type: vec![],
    }))
}

fn get_use_case_pallets(
    pallet_configs: &HashMap<String, PalletConfig>,
    query_template_type: &Path<TemplateType>,
    name: String,
    description: String,
) -> UseCase {
    let pallets: Vec<String> = pallet_configs
        .iter()
        .filter(|(_, pallet)| {
            pallet
                .metadata
                .use_cases
                .as_ref()
                .map_or(false, |use_case| use_case.contains(&name))
                || pallet
                    .metadata
                    .is_essential
                    .as_ref()
                    .map_or(false, |essential_templates| {
                        essential_templates.contains(&query_template_type.0)
                    })
        })
        .map(|(name, _)| name.to_string())
        .collect();

    UseCase {
        name,
        description,
        pallets,
    }
}
