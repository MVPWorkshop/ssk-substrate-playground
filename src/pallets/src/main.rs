use actix_web::{web, App, HttpServer};
use std::process;

async fn start_server() {
    HttpServer::new(|| {
        App::new()
        // ... setup routes
    })
        .bind("127.0.0.1:8080").unwrap()
        .run()
        .await.expect("REASON")
}

fn main() {
    // let server = start_server();
    // let _ = server.map_err(|e| {
    //     eprintln!("Server error: {}", e);
    //     process::exit(1);
    // });
}