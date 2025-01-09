use std::io::ErrorKind;
use std::sync::Arc;

use poem::endpoint::PrometheusExporter;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use prometheus::Registry;

use substrate_runtime_builder::{
    api::Api,
    services::{
        async_zip::AsyncZipArchiverService, code_generator::service::CodeGeneratorService,
        git::GitService, s3::S3ObjectStoreService,
    },
};

const PORT: &str = "3000";
const METRICS_PORT: &str = "9090";

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
    let prometheus_registry = Registry::new();

    let api_service = OpenApiService::new(
        Api::new(
            Arc::new(object_store_service),
            Arc::new(code_generator_service),
            Arc::new(GitService),
            &prometheus_registry,
        ),
        "Substrate Runtime Builder",
        "1.0",
    )
    .server(hosted_url);
    let ui = api_service.swagger_ui();

    tokio::spawn(async move {
        Server::new(TcpListener::bind(format!("0.0.0.0:{PORT}")))
            .run(Route::new().nest("/", api_service).nest("/docs", ui))
            .await
    });
    Server::new(TcpListener::bind(format!("0.0.0.0:{METRICS_PORT}")))
        .run(Route::new().nest("/metrics", PrometheusExporter::new(prometheus_registry)))
        .await
}
