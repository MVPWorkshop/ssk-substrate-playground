use std::io::ErrorKind;
use std::sync::Arc;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

use substrate_runtime_builder::{
    api::Api, services::archiver::async_zip::AsyncZipArchiverService,
    services::code_generator::service::CodeGeneratorService,
    services::object_store::s3::S3ObjectStoreService,
};

const PORT: &str = "3000";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    let hosted_url = std::env::var("HOSTED_URL")
        .map_err(|e| format!("{:?}", e))
        .unwrap_or(format!("http://127.0.0.1:{PORT}"));
    tracing_subscriber::fmt::init();
    println!("Initializing Substrate Runtime Builder API server...");
    let archiver_service = Arc::new(AsyncZipArchiverService);
    let object_store_service = S3ObjectStoreService::new().await.map_err(|err| {
        std::io::Error::new(
            ErrorKind::Other,
            format!("Error creating object store: {:?}", err),
        )
    })?;
    let code_generator_service = CodeGeneratorService::try_new(archiver_service.clone())
        .await
        .map_err(|err| {
            std::io::Error::new(
                ErrorKind::Other,
                format!("Error creating object store: {:?}", err),
            )
        })?;

    let api_service = OpenApiService::new(
        Api::new(
            Arc::new(object_store_service),
            Arc::new(code_generator_service),
        ),
        "Substrate Runtime Builder",
        "1.0",
    )
    .server(hosted_url);
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind(format!("0.0.0.0:{PORT}")))
        .run(Route::new().nest("/", api_service).nest("/docs", ui))
        .await
}
