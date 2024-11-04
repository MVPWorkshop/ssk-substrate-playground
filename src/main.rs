use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::Error;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use substrate_runtime_builder::code_generator::{
    generate_project, get_all_pallet_configs_from_dir,
};
use substrate_runtime_builder::route::get_templates;
use substrate_runtime_builder::types::PalletConfig;
use substrate_runtime_builder::utils::file_manager::create_github_repo;
use substrate_runtime_builder::utils::file_manager::download_project;
use substrate_runtime_builder::utils::file_manager::push_to_github;
use substrate_runtime_builder::CONFIG_DIR;

// Define a struct for the project with a vector of pallets
#[derive(Serialize, Deserialize)]
struct NewProject {
    name: String,
    pallets: Vec<String>,
    push_to_git: Option<bool>,
    github_username: String,
    github_email: String,
    github_token: String,
}

// A function to greet a user by their name (path parameter)
async fn greet_user(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello, {}!", name))
}

// A function to create a new project with a list of pallets
async fn generate_a_project(
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

    let result = actix_web::web::block({
        let project_name = project_name.clone();
        move || {
            let pallets = filtered_configs.values().cloned().collect::<Vec<_>>();
            // TODO: make generate_project async (it's IO bound, no need to block)
            if generate_project(&project_name, pallets).is_ok() {
                Ok(format!("{} project generated successfully", project_name))
            } else {
                Err("Error generating project".to_string())
            }
        }
    })
    .await?
    .map(|res| Ok(HttpResponse::Ok().body(res)))
    .map_err(actix_web::error::ErrorInternalServerError)?;

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
async fn list_supported_pallets(config_pallets: Data<Vec<PalletConfig>>) -> impl Responder {
    let supported_pallets: Vec<String> = config_pallets
        .iter()
        .map(|pallet| pallet.name.clone())
        .collect();
    HttpResponse::Ok().json(supported_pallets)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print a message to indicate that the server is starting
    println!("Starting server at http://0.0.0.0:8080");

    //insert_pallet_data_to_db().await;
    let data = Data::from(Arc::new(
        get_all_pallet_configs_from_dir(CONFIG_DIR)
            .map_err(|err| std::io::Error::new(ErrorKind::Other, err.to_string()))?,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .route("/", web::get().to(greet_user))
            .route("/generate-project", web::post().to(generate_a_project))
            .route(
                "/download-project/{project_name}",
                web::get().to(download_project),
            )
            .route("/supported-pallets", web::get().to(list_supported_pallets))
            .route("/templates", web::get().to(get_templates))
    })
    .workers(4)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
