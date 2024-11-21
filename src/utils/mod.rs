pub mod file_manager;
pub mod manifest;
pub mod runtime_lib;

use std::fs::File;

use handlebars;
use serde::Serialize;
use thiserror::Error;

const TEMPLATE: &str = "template";
const HBS_SUFFIX: &str = ".hbs";

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

pub fn render_handlebars_template<T>(
    template_path: &str, // output path is template_path with .hbs removed
    data: &T,
) -> Result<(), TemplateRenderError>
where
    T: Serialize,
{
    let output_file_path =
        template_path
            .strip_suffix(HBS_SUFFIX)
            .ok_or(TemplateRenderError::TemplatePathError(
                template_path.to_string(),
            ))?;
    let mut handlebars = handlebars::Handlebars::new();
    handlebars.register_template_file(TEMPLATE, template_path)?;

    let mut output_file = File::create(output_file_path)?;
    handlebars.render_to_write(TEMPLATE, data, &mut output_file)?;
    std::fs::remove_file(template_path)?;
    Ok(())
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
