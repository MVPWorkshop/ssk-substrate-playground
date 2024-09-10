use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use substrate_runtime_builder::code_generator::generate_project;
use substrate_runtime_builder::types::ESupportedPallets;

// Define a struct for the project with a vector of pallets
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

// A function to create a new project with a list of pallets
async fn generate_a_project(project: web::Json<NewProject>) -> impl Responder {
    // Clone project data for use in the blocking thread
    let project_name = project.name.clone();
    let pallet_names = project.pallets.clone();

    // Spawn a blocking task for generating the project
    let result = web::block(move || {
        // Initialize an empty vector of enum type `Message`
        let mut pallets: Vec<ESupportedPallets> = Vec::new();

        for pallet in &pallet_names {
            match ESupportedPallets::try_from(pallet.as_str()).unwrap_or(ESupportedPallets::Unknown) {
                ESupportedPallets::PalletUtility => {
                    // Push enum variants into the vector
                    pallets.push(ESupportedPallets::PalletUtility);
                }
                _ => continue,
            }
        }

        // Generate the project (blocking operation)
        generate_project(project_name.clone(), pallets);
        let message = format!("{} project generated successfully", project_name);
        Ok::<_, ()>(message)
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
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet_user))
            .route("/generate-project", web::post().to(generate_a_project))
    })
        .workers(4) // Set the number of workers (threads) to handle requests
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
