use std::collections::HashMap;

use crate::services::code_generator::types::{PalletConfig, PalletMetadata};
use poem_openapi::payload::Json;
use poem_openapi::ApiResponse;

#[derive(ApiResponse)]
pub enum ListSupportedPalletsResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<HashMap<String, PalletMetadata>>),
}

pub async fn list_supported_pallets_handler(
    pallet_configs: &HashMap<String, PalletConfig>,
) -> ListSupportedPalletsResponse {
    ListSupportedPalletsResponse::Ok(Json(
        pallet_configs
            .iter()
            .map(|(name, config)| (name.clone(), config.metadata.clone()))
            .collect::<HashMap<_, _>>(),
    ))
}
