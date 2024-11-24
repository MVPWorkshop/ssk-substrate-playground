pub mod manifest;
pub mod runtime_lib;

use handlebars;
use serde::Serialize;
use thiserror::Error;

const TEMPLATE: &str = "template";
pub const HBS_SUFFIX: &str = "hbs";

#[derive(Error, Debug)]
pub enum TemplateRenderError {
    #[error("Failed to load template: {0}")]
    TemplateError(#[from] handlebars::TemplateError),
    #[error("Template path: ({0}) does not contain .hbs suffix ")]
    TemplatePathError(String),
    #[error("Failed a file operation: {0}")]
    FileOperationError(#[from] std::io::Error),
    #[error("Failed to render template: {0}")]
    RenderError(#[from] handlebars::RenderError),
}

pub fn render_handlebars_template_to_bytes<T>(
    template_path: &str,
    data: &T,
) -> Result<Vec<u8>, TemplateRenderError>
where
    T: Serialize,
{
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_file(TEMPLATE, template_path)?;
    Ok(handlebars.render(TEMPLATE, data)?.into_bytes())
}
