use std::collections::HashMap;

use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object,
};

use crate::services::code_generator::types::{PalletConfig, TemplateType};

#[derive(Object)]
pub struct GetDependenciesRequest {
    pub template: TemplateType,
    pub pallets: Option<Vec<String>>,
}

#[derive(ApiResponse)]
pub enum GetDependenciesResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<HashMap<String, Vec<String>>>),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

pub async fn get_dependencies_handler(
    pallet_configs: &HashMap<String, PalletConfig>,
    request: Json<GetDependenciesRequest>,
) -> GetDependenciesResponse {
    // Check if the pallets are supported
    if let Some(pallets) = &request.pallets {
        for pallet_name in pallets.iter() {
            if !pallet_configs.contains_key(pallet_name) {
                return GetDependenciesResponse::NotFound(PlainText(format!(
                    "Pallet {} not found",
                    pallet_name
                )));
            }
        }
    }
    let essential = pallet_configs
        .iter()
        .filter_map(|(name, pallet)| match &pallet.metadata.is_essential {
            None => None,
            Some(templates) => {
                if templates.contains(&request.template) {
                    Some(name.clone())
                } else {
                    None
                }
            }
        })
        .collect::<Vec<String>>();
    let filtered = match &request.pallets {
        None => pallet_configs,
        Some(pallets) => &pallet_configs
            .iter()
            .filter(|(name, _)| pallets.contains(name))
            .map(|(name, config)| (name.clone(), config.clone()))
            .collect(),
    };
    let response = filtered
        .iter()
        .map(|(name, config)| {
            let mut dependencies = essential.clone();
            dependencies.push(name.clone());
            if let Some(req_deps) = &config.dependencies.required_pallets {
                dependencies.extend(req_deps.iter().cloned());
            }
            (name.clone(), dependencies)
        })
        .collect::<HashMap<_, _>>();
    GetDependenciesResponse::Ok(Json(response))
}
