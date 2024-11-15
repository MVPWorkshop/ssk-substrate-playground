use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;

use crate::code_generator::generate_project;
use crate::types::{PalletCategories, PalletConfig, ParameterType};

use chrono::Utc;
use dyn_fmt::AsStrFormatExt;
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, Enum, Object, OpenApi};
use serde::{Deserialize, Serialize};

// Define a struct for the project with a vector of pallets
#[derive(Serialize, Deserialize, Object)]
pub struct NewProject {
    name: String,
    pallets: Vec<String>,
}

// Pallet structure that will be returned as JSON
#[derive(Serialize, PartialEq, Eq, Debug, Object)]
pub struct Pallet {
    name: String,
    description: String,
    category: String,
}

// Chain use case structure that will be returned as JSON
#[derive(Serialize, PartialEq, Eq, Debug, Object)]
pub struct UseCase {
    name: String,
    description: String,
    pallets: Vec<String>,
}

#[derive(Serialize, PartialEq, Eq, Debug, Enum)]
pub enum TemplateType {
    SoloChain,
    ParaChain,
}

// Blockchain template structure.
#[derive(Serialize, PartialEq, Eq, Debug, Object)]
pub struct BlockchainTemplate {
    template_type: TemplateType,
    essential_pallets: Vec<Pallet>,
    supported_pallets: Vec<Pallet>,
    use_cases: Vec<UseCase>,
    chain_type: Vec<UseCase>,
}

pub async fn get_templates(
    pallet_configs: &[PalletConfig],
    query_template_type: Path<Option<TemplateType>>,
) -> GetTemplatesResponse {
    let templates: Vec<BlockchainTemplate> = vec![
        BlockchainTemplate {
            template_type: TemplateType::SoloChain,
            essential_pallets: pallet_configs
                .iter()
                .filter(|pallet| pallet.metadata.is_essential)
                .map(|pallet| Pallet {
                    name: pallet.name.clone(),
                    description: pallet.metadata.description.clone(),
                    category: "Core".to_string(),
                })
                .collect::<Vec<_>>(),
            supported_pallets: pallet_configs
                .iter()
                .filter(|pallet| !pallet.metadata.is_essential)
                .map(|pallet| Pallet {
                    name: pallet.name.clone(),
                    description: pallet.metadata.description.clone(),
                    category: pallet
                        .metadata
                        .categories
                        .as_ref()
                        .unwrap_or(&vec![PalletCategories::Runtime])[0]
                        .to_string(),
                })
                .collect::<Vec<_>>(),
            use_cases: vec![],
            chain_type: vec![],
        },
        BlockchainTemplate {
            template_type: TemplateType::ParaChain,
            essential_pallets: vec![],
            supported_pallets: vec![],
            use_cases: vec![],
            chain_type: vec![],
        },
    ];
    // Filtering the templates based on the `template_type` query parameter
    let filtered_templates: Vec<BlockchainTemplate> = match &query_template_type.0 {
        Some(template_type) => templates
            .into_iter()
            .filter(|t| t.template_type == *template_type)
            .collect(),
        _ => templates,
    };

    // Return JSON response
    GetTemplatesResponse::Ok(Json(filtered_templates))
}

pub fn get_config(pallet_configs: Vec<PalletConfig>, pallet: &str) -> String {
    if let Some(assets_config) = pallet_configs.iter().find(|config| config.name == pallet) {
        assets_config.metadata.description.to_string()
    } else {
        "Polkadot Frame Pallet".to_string()
    }
}

// A function to create a new project with a list of pallets
pub async fn generate_a_project(
    config_pallets: &[PalletConfig],
    project: Json<NewProject>,
) -> GenerateProjectResponse {
    let mut project_name = project.name.clone();
    let pallet_names = project.pallets.clone();
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    // Check if the pallets are supported
    for pallet in pallet_names.iter() {
        if !config_pallets.iter().any(|config| config.name == *pallet) {
            return GenerateProjectResponse::PalletNotFound(PlainText(format!(
                "Pallet {} not found",
                pallet
            )));
        }
    }
    // Get the required pallets for the pallets in the list
    let filtered = config_pallets
        .iter()
        // Get the pallets that are in the list of pallet names
        .filter(|pallet| pallet_names.contains(&pallet.name))
        // Get the required pallets for each pallet
        .flat_map(|pallet| {
            let mut palet_with_reqs = vec![pallet.name.clone()];
            if let Some(required_pallets) = pallet.dependencies.required_pallets.clone() {
                palet_with_reqs.extend(required_pallets);
            }
            palet_with_reqs
        })
        .collect::<Vec<_>>();

    // Filter the pallets that are in the list of pallet names
    let filtered_configs = config_pallets
        .iter()
        .filter(|pallet| filtered.contains(&pallet.name))
        .fold(HashMap::new(), |mut acc, pallet| {
            acc.insert(pallet.name.clone(), pallet.clone());
            acc
        });

    // Append the username and timestamp to the project name to ensure uniqueness
    project_name = format!("{}_{}", project_name, timestamp);
    let pallets = filtered_configs.values().cloned().collect::<Vec<_>>();
    match generate_project(&project_name, pallets).await {
        Ok(_) => {
            GenerateProjectResponse::Ok(PlainText(format!("Project {} created", project_name)))
        }
        Err(err) => GenerateProjectResponse::InternalServerError(PlainText(err.to_string())),
    }
}

pub struct Api {
    pub pallet_configs: Arc<Vec<PalletConfig>>,
}

impl Api {
    pub fn new(pallet_configs: Arc<Vec<PalletConfig>>) -> Self {
        Self { pallet_configs }
    }
}

#[derive(ApiResponse)]
pub enum ListSupportedPalletsResponse<'a> {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<&'a Vec<PalletConfig>>),
}

#[derive(ApiResponse)]
pub enum GenerateProjectResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(PlainText<String>),
    #[oai(status = 404)]
    PalletNotFound(PlainText<String>),
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

#[derive(ApiResponse)]
pub enum GetTemplatesResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<Vec<BlockchainTemplate>>),
}

#[derive(Object)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub possible_unit_names: Vec<String>,
    pub multiplier_configurable: bool,
    pub example: String,
}

impl From<&ParameterType> for Parameter {
    fn from(pt: &ParameterType) -> Self {
        let unit = if pt.expression.possible_units.len() > 0 {
            "{{unit}}"
        } else {
            ""
        };
        let example_expression = pt.expression.format.format(&[unit, "{{multiplier}}"]);
        Self {
            name: pt.name.clone(),
            description: pt.description.clone(),
            possible_unit_names: pt.expression.possible_units.clone(),
            multiplier_configurable: pt.expression.multiplier_configurable,
            example: format!(
                "pub{}{}: {} = {};",
                pt.prefix, pt.name, pt.p_type, example_expression
            ),
        }
    }
}

#[derive(ApiResponse)]
pub enum GetConfigurableParametersResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<Vec<Parameter>>),
    #[oai(status = 404)]
    PalletNotFound(PlainText<String>),
}

#[OpenApi]
impl Api {
    #[oai(path = "/hello/:name", method = "get")]
    pub async fn greet_user(
        &self,
        #[oai(name = "name", validator(max_length = 30, min_length = 1))] name: Path<String>,
    ) -> PlainText<String> {
        PlainText(format!("hello, {}!", name.0))
    }
    #[oai(path = "/list-supported-pallets", method = "get")]
    pub async fn list_supported_pallets(&self) -> ListSupportedPalletsResponse {
        ListSupportedPalletsResponse::Ok(Json(self.pallet_configs.as_ref()))
    }
    #[oai(path = "/generate-project", method = "post")]
    pub async fn generate_a_project(&self, project: Json<NewProject>) -> GenerateProjectResponse {
        generate_a_project(&self.pallet_configs, project).await
    }
    #[oai(path = "/get-templates/:template_type", method = "get")]
    pub async fn get_templates(
        &self,
        template_type: Path<Option<TemplateType>>,
    ) -> GetTemplatesResponse {
        get_templates(&self.pallet_configs, template_type).await
    }
    #[oai(path = "/get-configurable-parameters/:pallet", method = "get")]
    pub async fn get_configurable_parameters(
        &self,
        pallet: Path<String>,
    ) -> GetConfigurableParametersResponse {
        let pallet = match self
            .pallet_configs
            .iter()
            .find(|config| config.name == pallet.0)
        {
            Some(pallet) => pallet,
            None => {
                return GetConfigurableParametersResponse::PalletNotFound(PlainText(format!(
                    "Pallet {} not found",
                    pallet.0
                )))
            }
        };
        let parameters =
            if let Some(optional_parameter_types) = &pallet.runtime.optional_parameter_types {
                optional_parameter_types
                    .iter()
                    .map(Parameter::from)
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };
        GetConfigurableParametersResponse::Ok(Json(parameters))
    }
}
#[cfg(test)]
mod tests {}
