use std::sync::Arc;

use crate::services::{
    code_generator::{ CodeGenerator, CodeGeneratorServiceError},
    object_store::ObjectStoreService,
};
use handlers::get_pallet_options_handler::PalletOptionsRequest;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use scc::HashMap as ConcurrentHashMap;
use uuid::Uuid;

pub mod handlers;

pub struct Api {
    pub task_handles:
        Arc<ConcurrentHashMap<Uuid, Option<Result<String, CodeGeneratorServiceError>>>>,
    pub object_store_service: Arc<dyn ObjectStoreService>,
    pub code_generator_service: Arc<dyn CodeGenerator>,
}
#[OpenApi]
impl Api {
    pub fn new(
        object_store_service: Arc<dyn ObjectStoreService>,
        code_generator_service: Arc<dyn CodeGenerator>,
    ) -> Self {
        Self {
            task_handles: Arc::new(ConcurrentHashMap::new()),
            object_store_service,
            code_generator_service,
        }
    }
    #[oai(path = "/list-supported-pallets", method = "get")]
    pub async fn list_supported_pallets(
        &self,
    ) -> handlers::list_supported_pallets_handler::ListSupportedPalletsResponse {
        handlers::list_supported_pallets_handler::list_supported_pallets_handler(
            self.code_generator_service.pallet_configs(),
        )
        .await
    }
    #[oai(path = "/generate-project", method = "post")]
    pub async fn generate_a_project(
        &self,
        project: Json<handlers::generate_project_handler::NewProject>,
    ) -> handlers::generate_project_handler::GenerateProjectResponse {
        handlers::generate_project_handler::generate_a_project_handler(
            self.task_handles.clone(),
            self.object_store_service.clone(),
            self.code_generator_service.clone(),
            project,
        )
        .await
    }
    #[oai(path = "/get-templates/:template_type", method = "get")]
    pub async fn get_templates(
        &self,
        template_type: Path<Option<crate::services::code_generator::types::TemplateType>>,
    ) -> handlers::get_templates_handler::GetTemplatesResponse {
        handlers::get_templates_handler::get_templates_handler(
            self.code_generator_service.pallet_configs(),
            template_type,
            self.code_generator_service.templates().clone(),
        )
        .await
    }
    #[oai(path = "/get-pallet-options", method = "post")]
    pub async fn get_pallet_options(
        &self,
        request: Json<PalletOptionsRequest>
    ) -> handlers::get_pallet_options_handler::GetPalletOptionsResponse {
        handlers::get_pallet_options_handler::get_pallet_options_handler(
            self.code_generator_service.pallet_configs(),
            request
        )
        .await
    }
    #[oai(path = "/get-status/:task_id", method = "get")]
    pub async fn get_status(
        &self,
        task_id: Path<Uuid>,
    ) -> handlers::get_status_handler::GetStatusResponse {
        handlers::get_status_handler::get_status_handler(self.task_handles.clone(), task_id).await
    }
}
