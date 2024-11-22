use serde::Serialize;

use crate::services::code_generator::types::PalletConfig;

use super::{render_handlebars_template_to_bytes, TemplateRenderError};

#[derive(Debug, Serialize)]
pub struct ManifestConfig {
    pub name: String,
    pub git_path: String,
    pub tag: String,
    pub use_default: bool,
}

impl From<&PalletConfig> for ManifestConfig {
    fn from(pallet: &PalletConfig) -> Self {
        Self {
            name: pallet.dependencies.pallet.name_cebab_case(),
            git_path: pallet.dependencies.pallet.git_repo.clone().unwrap(),
            tag: pallet.dependencies.pallet.tag.clone().unwrap(),
            use_default: pallet.dependencies.pallet.default_features,
        }
    }
}

pub fn generate_manifest_file_to_bytes(
    mainfest_file_path: &str,
    pallet_configs: &[PalletConfig],
) -> Result<Vec<u8>, TemplateRenderError> {
    let manifest_configs: Vec<ManifestConfig> =
        pallet_configs.iter().map(|pallet| pallet.into()).collect();
    render_handlebars_template_to_bytes(mainfest_file_path, &manifest_configs)
}
