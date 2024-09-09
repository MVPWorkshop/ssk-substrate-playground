use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use substrate_runtime_builder::code_generator::generate_project;
use substrate_runtime_builder::types::ESupportedPallets;

// Define a struct for the user with a vector of places
#[derive(Serialize, Deserialize)]
struct NewProject {
    name: String,
    pallets: Vec<String>,
}

// A function to greet a user by their name (path parameter)
async fn greet_user(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    format!("Hello, {}!", name)
}

// A function to create a new user with a list of places
async fn generate_a_project(project: web::Json<NewProject>) -> impl Responder {
    // Initialize an empty vector of enum type `Message`
    let mut pallets: Vec<ESupportedPallets> = Vec::new();

    for pallet in &project.pallets {
        match ESupportedPallets::try_from(pallet.as_str()).unwrap_or(ESupportedPallets::Unknown) {
            ESupportedPallets::PalletUtility => {
                // Push enum variants into the vector
                pallets.push(ESupportedPallets::PalletUtility);
            }
            _ => continue,
        }
    }

    generate_project(project.name.clone(), pallets);
    return HttpResponse::Ok().body("New project with Utility pallet is generated on location");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print a message to indicate that the server is starting
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet_user))
            .route("/generate-project", web::post().to(generate_a_project))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
