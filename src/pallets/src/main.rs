// use actix_web::{web};
use substrate_runtime_builder::code_generator::generate_project;
use substrate_runtime_builder::types::ESupportedPallets;
use std::process;

async fn start_server() {
    HttpServer::new(|| {
        App::new()
        // ... setup routes
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .expect("REASON")
}


use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// Define a struct for the user with a vector of places
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    places: Vec<String>,
}

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
async fn create_user(user: web::Json<User>) -> impl Responder {
    // Check if "New York" is in the list of places
    if user.places.contains(&"New York".to_string()) {
        HttpResponse::Ok().body("User has New York in their places.")
    } else {
        HttpResponse::Ok().json(user.into_inner())
    }
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

        },
            _ => continue,
        }
    }

    generate_project(project.name.clone(), pallets);
    return HttpResponse::Ok().body("New project with Utility pallet is generated on location");
    // HttpResponse::Ok().json(project.into_inner())
}

// A function to get a user by name and return their places
async fn get_user_by_name(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let user = User {
        name: name.clone(),
        places: vec!["New York".to_string(), "San Francisco".to_string()], // Example places
    };

    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print a message to indicate that the server is starting
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet_user))
            .route("/users", web::post().to(create_user))
            .route("/generate-project", web::post().to(generate_a_project))
            .route("/users/{name}", web::get().to(get_user_by_name))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// fn main() {
//     // let server = start_server();
//     // let _ = server.map_err(|e| {
//     //     eprintln!("Server error: {}", e);
//     //     process::exit(1);
//     // });
//
//     // // Initialize an empty vector of enum type `Message`
//     // let mut pallets: Vec<ESupportedPallets> = Vec::new();
//     //
//     // // Push enum variants into the vector
//     // pallets.push(ESupportedPallets::PalletUtility);
//     //
//     // generate_project("test".to_string(), pallets);
//
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(greet_user))
//             .route("/users", web::post().to(create_user))
//             .route("/users/{name}", web::get().to(get_user_by_name))
//     })
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
//
// }
