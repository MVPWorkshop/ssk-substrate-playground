use std::io::ErrorKind;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use substrate_runtime_builder::code_generator::get_all_pallet_configs_from_dir;
use substrate_runtime_builder::route::{
    generate_a_project, get_templates, greet_user, list_supported_pallets,
};
use substrate_runtime_builder::utils::file_manager::download_project;
use substrate_runtime_builder::CONFIG_DIR;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print a message to indicate that the server is starting
    println!("Starting server at http://0.0.0.0:8080");

    //insert_pallet_data_to_db().await;
    let data = Data::from(Arc::new(
        get_all_pallet_configs_from_dir(CONFIG_DIR)
            .await
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
