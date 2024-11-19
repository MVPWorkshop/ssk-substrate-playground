use std::{collections::HashMap, sync::Arc};

use poem_openapi::{
    payload::{Json, PlainText},
    types::Example,
    ApiResponse, Object,
};
use scc::HashMap as ConcurrentHashMap;
use uuid::Uuid;

use crate::{
    code_generator::{generate_project, PalletConfigLoadError},
    types::PalletConfig,
};

#[derive(Object, Clone)]
pub struct ParameterConfiguration {
    /// The multiplier of the parameter
    pub multiplier: Option<i64>,
    /// The unit of the parameter
    pub unit: Option<String>,
}

impl Example for ParameterConfiguration {
    fn example() -> Self {
        Self {
            multiplier: Some(123),
            unit: Some("Some unit from provided list".to_string()),
        }
    }
}
#[derive(Object)]
pub struct NewProject {
    /// The name of the project
    name: String,
    /// The list of pallets to include in the project, where the key is the
    /// pallet name and the value is a optional map of configuration parameters
    pallets: HashMap<String, Option<HashMap<String, ParameterConfiguration>>>,
}

impl Example for NewProject {
    fn example() -> Self {
        let mut pallets = HashMap::new();
        let mut pallet_config = HashMap::new();
        pallet_config.insert("Some Config".to_string(), ParameterConfiguration::example());
        pallets.insert("Some Pallet".to_string(), Some(pallet_config));
        Self {
            name: "project_name".to_string(),
            pallets,
        }
    }
}

#[derive(ApiResponse)]
pub enum GenerateProjectResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<Uuid>),
    #[oai(status = 404)]
    PalletNotFound(PlainText<String>),
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

pub async fn generate_a_project_handler(
    config_pallets: &HashMap<String, PalletConfig>,
    task_status_map: Arc<ConcurrentHashMap<Uuid, Option<Result<(), PalletConfigLoadError>>>>,
    project: Json<NewProject>,
) -> GenerateProjectResponse {
    let mut project_name = project.name.clone();

    // Check if the pallets are supported
    for pallet_name in project.0.pallets.keys() {
        if !config_pallets.contains_key(pallet_name) {
            return GenerateProjectResponse::PalletNotFound(PlainText(format!(
                "Pallet {} not found",
                pallet_name
            )));
        }
    }

    // Get the required pallets for the pallets in the list
    let mut filtered: Vec<String> = config_pallets
        .iter()
        // Get the pallets that are in the list of pallet names
        .filter(|(name, _)| project.0.pallets.contains_key(*name))
        // Get the required pallets for each pallet
        .flat_map(|(pallet_name, pallet)| {
            let mut pallet_with_reqs = vec![pallet_name.clone()];
            if let Some(required_pallets) = pallet.dependencies.required_pallets.clone() {
                pallet_with_reqs.extend(required_pallets);
            }
            pallet_with_reqs
        })
        .collect::<Vec<String>>();

    let essential = config_pallets
        .iter()
        .filter(|pallet| pallet.1.metadata.is_essential)
        .map(|pallet| pallet.0.clone())
        .collect::<Vec<_>>();
    filtered.extend(essential);

    // create local coppy of the pallets
    let config_pallets = config_pallets
        .iter()
        .map(|(name, config)| (name.clone(), config.clone()))
        .collect::<HashMap<_, _>>();
    // Filter the pallets that are in the list of pallet names
    let mut filtered_configs = config_pallets
        .into_iter()
        .filter(|(pallet_name, _)| filtered.contains(pallet_name))
        .collect::<HashMap<_, _>>();
    project
        .0
        .pallets
        .into_iter()
        .filter_map(|(name, config)| config.map(|config| (name, config)))
        .for_each(|(name, config)| {
            let pallet_to_configure = filtered_configs.get_mut(&name).unwrap();
            for input in config {
                if let Some(parameter) = pallet_to_configure
                    .runtime
                    .optional_parameter_types
                    .as_mut()
                    .unwrap()
                    .get_mut(&input.0)
                {
                    parameter.expression.configured_multiplier = input.1.multiplier;
                    parameter.expression.configured_unit = input.1.unit;
                }
            }
        });
    // Append uuid to project name
    project_name = format!("{}_{}", project_name, Uuid::new_v4());
    let pallets = filtered_configs.values().cloned().collect::<Vec<_>>();
    let status_id = Uuid::new_v4();
    let _ = task_status_map.insert_async(status_id, None).await;
    tokio::spawn(async move {
        generate_project(&project_name, pallets, status_id, task_status_map).await
    });
    GenerateProjectResponse::Ok(Json(status_id))
}
