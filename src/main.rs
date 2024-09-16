use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use substrate_runtime_builder::code_generator::generate_project;
use substrate_runtime_builder::types::ESupportedPallets;
use substrate_runtime_builder::utils::file_manager::download_project;

// Define a struct for the project with a vector of pallets
#[derive(Serialize, Deserialize)]
struct NewProject {
    name: String,
    pallets: Vec<String>,
}

// A function to greet a user by their name (path parameter)
async fn greet_user(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello, {}!", name))
}

// A function to create a new project with a list of pallets
async fn generate_a_project(project: web::Json<NewProject>) -> impl Responder {
    let project_name = project.name.clone();
    let pallet_names = project.pallets.clone();

    let result = actix_web::web::block(move || {
        let mut pallets: Vec<ESupportedPallets> = Vec::new();

        for pallet in &pallet_names {
            match ESupportedPallets::try_from(pallet.as_str()).unwrap_or(ESupportedPallets::Unknown)
            {
                ESupportedPallets::PalletUtility => {
                    pallets.push(ESupportedPallets::PalletUtility);
                },
                ESupportedPallets::PalletIdentity => {
                    pallets.push(ESupportedPallets::PalletIdentity);
                }
                _ => continue,
            }
        }

        generate_project(project_name.clone(), pallets);
        format!("{} project generated successfully", project_name)
    })
    .await;

    match result {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(_) => HttpResponse::InternalServerError().body("Error generating the project"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print a message to indicate that the server is starting
    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet_user))
            .route("/generate-project", web::post().to(generate_a_project))
            .route(
                "/download-project/{project_name}",
                web::get().to(download_project),
            )
    })
    .workers(4)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
