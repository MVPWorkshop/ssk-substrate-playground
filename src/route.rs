use std::collections::HashMap;

use crate::code_generator::generate_project;
use crate::types::{PalletCategories, PalletConfig};
use crate::utils::file_manager::{create_github_repo, push_to_github};
use actix_web::web::Data;
use actix_web::Error;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};

// Define a struct for the project with a vector of pallets
#[derive(Serialize, Deserialize)]
pub struct NewProject {
    name: String,
    pallets: Vec<String>,
    push_to_git: Option<bool>,
    github_username: String,
    github_email: String,
    github_token: String,
}

// Pallet structure that will be returned as JSON
#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct Pallet {
    name: String,
    description: String,
    category: String,
}

// Chain use case structure that will be returned as JSON
#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct UseCase {
    name: String,
    description: String,
    pallets: Vec<String>,
}

// Blockchain template structure.
#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct BlockchainTemplate {
    template_type: String,
    essential_pallets: Vec<Pallet>,
    supported_pallets: Vec<Pallet>,
    use_cases: Vec<UseCase>,
    chain_type: Vec<UseCase>,
}

// Query structure for extracting query parameters from the URL
#[derive(Deserialize)]
pub struct TemplateQuery {
    template_type: Option<String>, // Query parameter
}

pub async fn get_templates(
    pallet_configs: Data<Vec<PalletConfig>>,
    query: web::Query<TemplateQuery>,
) -> impl Responder {
    let templates = get_templates_internal(pallet_configs.to_vec(), &query.template_type).await;
    HttpResponse::Ok().json(templates)
}

pub async fn get_templates_internal(
    pallet_configs: Vec<PalletConfig>,
    query_template_type: &Option<String>,
) -> Vec<BlockchainTemplate> {
    let templates: Vec<BlockchainTemplate> = vec![
        BlockchainTemplate {
            template_type: String::from("solochain"),
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
            template_type: String::from("parachain"),
            essential_pallets: vec![],
            supported_pallets: vec![],
            use_cases: vec![],
            chain_type: vec![],
        },
    ];
    // Filtering the templates based on the `template_type` query parameter
    let filtered_templates: Vec<BlockchainTemplate> = match query_template_type {
        Some(template_type) => templates
            .into_iter()
            .filter(|t| t.template_type == *template_type)
            .collect(),
        None => templates,
    };

    // Return JSON response
    filtered_templates
}

pub fn get_config(pallet_configs: Vec<PalletConfig>, pallet: &str) -> String {
    if let Some(assets_config) = pallet_configs.iter().find(|config| config.name == pallet) {
        assets_config.metadata.description.to_string()
    } else {
        "Polkadot Frame Pallet".to_string()
    }
}

// A function to greet a user by their name (path parameter)
pub async fn greet_user(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello, {}!", name))
}

// A function to create a new project with a list of pallets
pub async fn generate_a_project(
    config_pallets: Data<Vec<PalletConfig>>,
    project: web::Json<NewProject>,
) -> actix_web::Result<HttpResponse, Error> {
    let mut project_name = project.name.clone();
    let pallet_names = project.pallets.clone();
    let push_to_git = project.push_to_git.unwrap_or(false);
    let github_username = project.github_username.clone();
    let github_email = project.github_email.clone();
    let github_token = project.github_token.clone();
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    // Extended list of pallets to include the with the required pallets
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

    let filtered_configs = config_pallets
        .iter()
        .filter(|pallet| filtered.contains(&pallet.name))
        .fold(HashMap::new(), |mut acc, pallet| {
            acc.insert(pallet.name.clone(), pallet.clone());
            acc
        });

    // Append the username and timestamp to the project name to ensure uniqueness
    project_name = format!("{}_{}_{}", project_name, github_username, timestamp);
    let pallets = filtered_configs.values().cloned().collect::<Vec<_>>();
    let result = match generate_project(&project_name, pallets).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().body(format!("{} project generated successfully", project_name)))
        }
        Err(err) => {
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Error generating project: {}",
                err
            )));
        }
    };

    // If push_to_git is true, create a GitHub repository and push the code
    if push_to_git {
        // Create a GitHub repository using the username, token, and project name
        match create_github_repo(&github_username, &github_token, &project_name).await {
            Ok(_) => println!("GitHub repo created"),
            Err(err) => {
                return Err(actix_web::error::ErrorInternalServerError(format!(
                    "Error creating GitHub repo: {}",
                    err
                )));
            }
        }
        // Attempt to push the code to GitHub
        match push_to_github(
            &project_name,
            &github_username,
            &github_email,
            &github_token,
        ) {
            Ok(_) => println!("Successfully pushed to GitHub"), // Log success when the push is successful
            Err(err) => {
                return Err(actix_web::error::ErrorInternalServerError(format!(
                    "Error pushing to GitHub: {}",
                    err,
                )));
            }
        }
    }
    result
}

// A function to return the list of supported pallets
pub async fn list_supported_pallets(config_pallets: Data<Vec<PalletConfig>>) -> impl Responder {
    let supported_pallets: Vec<String> = config_pallets
        .iter()
        .map(|pallet| pallet.name.clone())
        .collect();
    HttpResponse::Ok().json(supported_pallets)
}

#[cfg(test)]
mod tests {}
