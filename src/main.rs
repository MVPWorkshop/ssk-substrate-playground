use std::io::ErrorKind;
use std::sync::Arc;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

use substrate_runtime_builder::{
    api::Api, code_generator::get_all_pallet_configs_from_dir, CONFIG_DIR,
};

const PORT: &str = "3000";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    println!("Initializing Substrate Runtime Builder API server...");

    let data = Arc::new(
        get_all_pallet_configs_from_dir(CONFIG_DIR)
            .await
            .map_err(|err| std::io::Error::new(ErrorKind::Other, err.to_string()))?,
    );

    let api_service = OpenApiService::new(Api::new(data), "Substrate Runtime Builder", "1.0")
        .server(format!("http://127.0.0.1:{PORT}"));
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind(format!("0.0.0.0:{PORT}")))
        .run(Route::new().nest("/", api_service).nest("/docs", ui))
        .await
}
