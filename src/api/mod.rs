use std::sync::Arc;

use crate::services::{
    code_generator::{types::TemplateType, CodeGenerator, CodeGeneratorServiceError},
    traits::{object_store::ObjectStoreService, version_control::VersionControlService},
};
use handlers::{
    get_pallet_options_handler::PalletOptionsRequest, get_status_handler::GetStatusResponse,
};
use poem_openapi::{param::Path, payload::Json, OpenApi};
use prometheus::{opts, register_int_counter_vec_with_registry, IntCounterVec, Registry};
use scc::HashMap as ConcurrentHashMap;
use uuid::Uuid;

pub mod handlers;

pub struct Api {
    pub task_handles:
        Arc<ConcurrentHashMap<Uuid, Option<Result<String, CodeGeneratorServiceError>>>>,
    pub object_store_service: Arc<dyn ObjectStoreService>,
    pub code_generator_service: Arc<dyn CodeGenerator>,
    pub version_control_service: Arc<dyn VersionControlService>,
    pub get_template_counter: Arc<IntCounterVec>,
    pub generate_project_counter: Arc<IntCounterVec>,
    pub pallet_counter: Arc<IntCounterVec>,
    pub status_counter: Arc<IntCounterVec>,
}
#[OpenApi]
impl Api {
    pub fn new(
        object_store_service: Arc<dyn ObjectStoreService>,
        code_generator_service: Arc<dyn CodeGenerator>,
        version_control_service: Arc<dyn VersionControlService>,
        prometheus_registry: &Registry,
    ) -> Self {
        let get_template_counter = register_int_counter_vec_with_registry!(
            opts!(
                "get_template_called",
                "Number of times get template endpoint was called with chain_type."
            ),
            &["chain_type"],
            prometheus_registry
        )
        .unwrap();
        let generate_project_counter = register_int_counter_vec_with_registry!(
            opts!(
                "generate_project_called",
                "Number of times generate_project endpoint was called with chain_type."
            ),
            &["chain_type"],
            prometheus_registry
        )
        .unwrap();
        let pallet_counter = register_int_counter_vec_with_registry!(
            opts!(
                "pallet_counter",
                "Number of times a pallet was added to a generate_project"
            ),
            &["pallet"],
            prometheus_registry
        )
        .unwrap();
        let status_counter = register_int_counter_vec_with_registry!(
            opts!("status_counter", "Project generation status",),
            &["status"],
            prometheus_registry,
        )
        .unwrap();
        Self {
            task_handles: Arc::new(ConcurrentHashMap::new()),
            object_store_service,
            code_generator_service,
            version_control_service,
            get_template_counter: Arc::new(get_template_counter),
            generate_project_counter: Arc::new(generate_project_counter),
            pallet_counter: Arc::new(pallet_counter),
            status_counter: Arc::new(status_counter),
        }
    }
    #[oai(path = "/generate-project", method = "post")]
    pub async fn generate_a_project(
        &self,
        project: Json<handlers::generate_project_handler::NewProject>,
    ) -> handlers::generate_project_handler::GenerateProjectResponse {
        for pallet in project.0.pallets.keys() {
            self.pallet_counter
                .with_label_values(&[pallet.as_str()])
                .inc();
        }
        self.generate_project_counter
            .with_label_values(&[&project.0.template.to_string()])
            .inc();
        handlers::generate_project_handler::generate_a_project_handler(
            self.task_handles.clone(),
            self.object_store_service.clone(),
            self.code_generator_service.clone(),
            self.version_control_service.clone(),
            project,
        )
        .await
    }
    #[oai(path = "/get-templates/:template_type", method = "get")]
    pub async fn get_templates(
        &self,
        template_type: Path<TemplateType>,
    ) -> handlers::get_templates_handler::GetTemplatesResponse {
        self.get_template_counter
            .with_label_values(&[&template_type.0.to_string()])
            .inc();
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
        request: Json<PalletOptionsRequest>,
    ) -> handlers::get_pallet_options_handler::GetPalletOptionsResponse {
        handlers::get_pallet_options_handler::get_pallet_options_handler(
            self.code_generator_service.pallet_configs(),
            request,
        )
        .await
    }
    #[oai(path = "/get-status/:task_id", method = "get")]
    pub async fn get_status(
        &self,
        task_id: Path<Uuid>,
    ) -> handlers::get_status_handler::GetStatusResponse {
        match handlers::get_status_handler::get_status_handler(self.task_handles.clone(), task_id)
            .await
        {
            GetStatusResponse::Ok(x) => {
                self.status_counter
                    .with_label_values(&[&"success".to_string()])
                    .inc();
                GetStatusResponse::Ok(x)
            }
            GetStatusResponse::TaskNotFound(x) => GetStatusResponse::TaskNotFound(x),
            GetStatusResponse::InternalServerError(e) => {
                self.status_counter
                    .with_label_values(&[&"fail".to_string()])
                    .inc();
                GetStatusResponse::InternalServerError(e)
            }
        }
    }
    #[oai(path = "/get-dependencies", method = "post")]
    pub async fn get_dependencies(
        &self,
        request: Json<handlers::get_dependencies_handler::GetDependenciesRequest>,
    ) -> handlers::get_dependencies_handler::GetDependenciesResponse {
        handlers::get_dependencies_handler::get_dependencies_handler(
            self.code_generator_service.pallet_configs(),
            request,
        )
        .await
    }
}
