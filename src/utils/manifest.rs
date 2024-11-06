use crate::types::PalletConfig;
use serde::Serialize;

use super::{render_handlebars_template, TemplateRenderError};

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

pub fn generate_manifest_file(
    mainfest_file_path: &str,
    pallet_configs: &[PalletConfig],
) -> Result<(), TemplateRenderError> {
    let manifest_configs: Vec<ManifestConfig> =
        pallet_configs.iter().map(|pallet| pallet.into()).collect();
    render_handlebars_template(mainfest_file_path, &manifest_configs)
}
