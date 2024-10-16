use actix_web::Error;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use substrate_runtime_builder::code_generator::generate_project;
use substrate_runtime_builder::route::get_templates;
use substrate_runtime_builder::types::ESupportedPallets;
use substrate_runtime_builder::utils::file_manager::create_github_repo;
use substrate_runtime_builder::utils::file_manager::download_project;
use substrate_runtime_builder::utils::file_manager::push_to_github;
use substrate_runtime_builder::utils::util::*;

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
    project: web::Json<NewProject>,
) -> actix_web::Result<HttpResponse, Error> {
    let mut project_name = project.name.clone();
    let pallet_names = project.pallets.clone();
    let push_to_git = project.push_to_git.unwrap_or(false);
    let github_username = project.github_username.clone();
    let github_email = project.github_email.clone();
    let github_token = project.github_token.clone();
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();

    // Append the username and timestamp to the project name to ensure uniqueness
    project_name = format!("{}_{}_{}", project_name, github_username, timestamp);

    let result = actix_web::web::block({
        let project_name = project_name.clone();
        move || {
            let mut pallets: Vec<ESupportedPallets> = Vec::new();

            for pallet in &pallet_names {
                match ESupportedPallets::try_from(pallet.as_str())
                    .unwrap_or(ESupportedPallets::Unknown)
                {
                    ESupportedPallets::PalletUtility => {
                        pallets.push(ESupportedPallets::PalletUtility);
                    }
                    ESupportedPallets::PalletIdentity => {
                        pallets.push(ESupportedPallets::PalletIdentity);
                    }
                    ESupportedPallets::PalletMultisig => {
                        pallets.push(ESupportedPallets::PalletMultisig);
                    }
                    ESupportedPallets::PalletProxy => {
                        pallets.push(ESupportedPallets::PalletProxy);
                    }
                    ESupportedPallets::PalletUniques => {
                        pallets.push(ESupportedPallets::PalletUniques);
                    }
                    ESupportedPallets::PalletNfts => {
                        pallets.push(ESupportedPallets::PalletNfts);
                    }
                    ESupportedPallets::PalletMembership => {
                        pallets.push(ESupportedPallets::PalletMembership);
                    }
                    ESupportedPallets::PalletAssets => {
                        pallets.push(ESupportedPallets::PalletAssets);
                    }
                    ESupportedPallets::PalletBounties => {
                        pallets.push(ESupportedPallets::PalletBounties);

                        if !pallets.contains(&ESupportedPallets::PalletAssets) {
                            pallets.push(ESupportedPallets::PalletAssets);
                        }

                        if !pallets.contains(&ESupportedPallets::PalletTreasury) {
                            pallets.push(ESupportedPallets::PalletTreasury);
                        }
                    }
                    ESupportedPallets::PalletTreasury => {
                        pallets.push(ESupportedPallets::PalletTreasury);
                    }
                    ESupportedPallets::PalletChildBounties => {
                        pallets.push(ESupportedPallets::PalletChildBounties);

                        if !pallets.contains(&ESupportedPallets::PalletBounties) {
                            pallets.push(ESupportedPallets::PalletBounties);
                        }
                    }
                    ESupportedPallets::PalletVesting => {
                        pallets.push(ESupportedPallets::PalletVesting);
                    }
                    ESupportedPallets::PalletSociety => {
                        pallets.push(ESupportedPallets::PalletSociety);
                    }
                    ESupportedPallets::PalletCollective => {
                        pallets.push(ESupportedPallets::PalletCollective);
                    }
                    ESupportedPallets::PalletScheduler => {
                        pallets.push(ESupportedPallets::PalletScheduler);
                    }
                    ESupportedPallets::PalletDemocracy => {
                        pallets.push(ESupportedPallets::PalletDemocracy);

                        if !pallets.contains(&ESupportedPallets::PalletScheduler) {
                            pallets.push(ESupportedPallets::PalletScheduler);
                        }

                        if !pallets.contains(&ESupportedPallets::PalletCollective) {
                            pallets.push(ESupportedPallets::PalletCollective);
                        }
                    }
                    _ => continue,
                }
            }

            remove_duplicate_pallets(&mut pallets);

            // Calls the function to generate the project with the given name and pallets
            generate_project(project_name.clone(), pallets);
            format!("{} project generated successfully", project_name)
        }
    })
    .await
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
async fn list_supported_pallets() -> impl Responder {
    let supported_pallets = vec![
        "Assets",
        "Bounties",
        "Treasury",
        "Vesting",
        "Society",
        "Utility",
        "Identity",
        "Multisig",
        "Proxy",
        "Nfts",
        "Uniques",
        "Membership",
        "ChildBounties",
        "Collective",
        "Scheduler",
        "Democracy",
    ];

    HttpResponse::Ok().json(supported_pallets)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print a message to indicate that the server is starting
    println!("Starting server at http://0.0.0.0:8080");

    //insert_pallet_data_to_db().await;

    HttpServer::new(|| {
        App::new()
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
