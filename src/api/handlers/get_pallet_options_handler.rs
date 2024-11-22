use std::collections::HashMap;

use dyn_fmt::AsStrFormatExt;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object,
};

use crate::types::{PalletConfig, ParameterType};

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
        let unit = if !pt.expression.possible_units.is_empty() {
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
pub enum GetPalletOptionsResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<HashMap<String, Option<HashMap<String, Parameter>>>>),
    #[oai(status = 404)]
    PalletNotFound(PlainText<String>),
}

pub async fn get_pallet_options_handler(
    pallet_configs: &HashMap<String, PalletConfig>,
    pallets: Json<Vec<String>>,
) -> GetPalletOptionsResponse {
    // Check if the pallets are supported
    for pallet_name in pallets.iter() {
        if !pallet_configs.contains_key(pallet_name) {
            return GetPalletOptionsResponse::PalletNotFound(PlainText(format!(
                "Pallet {} not found",
                pallet_name
            )));
        }
    }
    // Get the required pallets for the pallets in the list
    let mut filtered = pallet_configs
        .iter()
        // Get the pallets that are in the list of pallet names
        .filter(|(name, _)| pallets.contains(name))
        // Get the required pallets for each pallet
        .flat_map(|(pallet_name, pallet)| {
            let mut palet_with_reqs = vec![pallet_name.clone()];
            if let Some(required_pallets) = pallet.dependencies.required_pallets.clone() {
                palet_with_reqs.extend(required_pallets);
            }
            palet_with_reqs
        })
        .collect::<Vec<_>>();

    let essential = pallet_configs
        .iter()
        .filter(|pallet| pallet.1.metadata.is_essential)
        .map(|pallet| pallet.0.clone())
        .collect::<Vec<_>>();
    filtered.extend(essential);

    let response_pallets = pallet_configs
        .iter()
        .filter(|(name, _)| filtered.contains(name))
        .map(
            |(name, pallet)| match pallet.runtime.optional_parameter_types {
                Some(ref optional_parameter_types) => {
                    let parameters = optional_parameter_types
                        .iter()
                        .map(|(name, pt)| (name.clone(), Parameter::from(pt)))
                        .collect::<HashMap<_, _>>();
                    (name.clone(), Some(parameters))
                }
                None => (name.clone(), None),
            },
        )
        .collect::<HashMap<_, _>>();
    GetPalletOptionsResponse::Ok(Json(response_pallets))
}
