use std::{collections::HashMap, sync::Arc};

use poem_openapi::{param::Path, payload::Json, OpenApi};

use crate::types::PalletConfig;

pub mod handlers;

pub struct Api {
    pub pallet_configs: Arc<HashMap<String, PalletConfig>>,
}
#[OpenApi]
impl Api {
    pub fn new(pallet_configs: Arc<HashMap<String, PalletConfig>>) -> Self {
        Self { pallet_configs }
    }
    #[oai(path = "/list-supported-pallets", method = "get")]
    pub async fn list_supported_pallets(
        &self,
    ) -> handlers::list_supported_pallets_handler::ListSupportedPalletsResponse {
        handlers::list_supported_pallets_handler::list_supported_pallets_handler(
            &self.pallet_configs,
        )
        .await
    }
    #[oai(path = "/generate-project", method = "post")]
    pub async fn generate_a_project(
        &self,
        project: Json<handlers::generate_project_handler::NewProject>,
    ) -> handlers::generate_project_handler::GenerateProjectResponse {
        handlers::generate_project_handler::generate_a_project_handler(
            &self.pallet_configs,
            project,
        )
        .await
    }
    #[oai(path = "/get-templates/:template_type", method = "get")]
    pub async fn get_templates(
        &self,
        template_type: Path<Option<handlers::get_templates_handler::TemplateType>>,
    ) -> handlers::get_templates_handler::GetTemplatesResponse {
        handlers::get_templates_handler::get_templates_handler(&self.pallet_configs, template_type)
            .await
    }
    #[oai(path = "/get-pallet-options", method = "post")]
    pub async fn get_pallet_options(
        &self,
        pallets: Json<Vec<String>>,
    ) -> handlers::get_pallet_options_handler::GetPalletOptionsResponse {
        handlers::get_pallet_options_handler::get_pallet_options_handler(
            &self.pallet_configs,
            pallets,
        )
        .await
    }
}