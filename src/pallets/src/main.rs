use actix_web::{web, App, HttpServer};
use pallets_configs::code_generator::generate_project;
use pallets_configs::types::ESupportedPallets;
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

fn main() {
    // let server = start_server();
    // let _ = server.map_err(|e| {
    //     eprintln!("Server error: {}", e);
    //     process::exit(1);
    // });

    // Initialize an empty vector of enum type `Message`
    let mut pallets: Vec<ESupportedPallets> = Vec::new();

    // Push enum variants into the vector
    pallets.push(ESupportedPallets::PalletUtility);

    generate_project("test".to_string(), pallets);
}
