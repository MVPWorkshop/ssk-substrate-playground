pub mod service;
pub mod types;
pub mod utils;

use std::collections::HashMap;

use async_trait::async_trait;
use thiserror::Error;
use types::PalletConfig;

use utils::load_configs::LoadConfigsError;
use utils::load_templates::LoadTemplatesError;

use crate::api::handlers::generate_project_handler::ParameterConfiguration;

use super::archiver::ArchiverError;

pub type Result<T> = std::result::Result<T, CodeGeneratorServiceError>;
// Define the CodeGeneratorServiceError
#[derive(Error, Debug, Clone)]
pub enum CodeGeneratorServiceError {
    #[error("{0}")]
    LoadConfigsError(#[from] LoadConfigsError),
    #[error("{0}")]
    LoadTemplatesError(#[from] LoadTemplatesError),
    #[error("{0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Pallet not found: {0}")]
    PalletNotFoundError(String),
    #[error("{0}")]
    ArchiveError(#[from] ArchiverError),
    #[error("{0}")]
    OtherError(String),
}

#[async_trait]
pub trait CodeGenerator: Send + Sync {
    fn pallet_configs(&self) -> &HashMap<String, PalletConfig>;
    fn templates(&self) -> &HashMap<String, HashMap<String, String>>;
    async fn generate_project_archive(
        &self,
        pallets: &HashMap<String, Option<HashMap<String, ParameterConfiguration>>>,
        template: (String, String),
    ) -> Result<Vec<u8>>;
}
