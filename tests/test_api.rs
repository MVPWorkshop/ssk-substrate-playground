use std::{arch::x86_64, collections::HashMap, sync::Arc, time::Duration};

use poem_openapi::{param::Path, payload::Json, ApiResponse};
use substrate_runtime_builder::{
    api::{
        handlers::{
            generate_project_handler::{
                GenerateProjectResponse, NewProject, ParameterConfiguration,
            },
            get_status_handler::{GetStatusResponse, Status, StatusResponse},
            list_supported_pallets_handler::ListSupportedPalletsResponse,
        },
        Api,
    },
    services::{
        archiver::async_zip::AsyncZipArchiverService,
        code_generator::service::CodeGeneratorService, object_store::s3::S3ObjectStoreService,
    },
};

fn env_setup() {
    dotenv::from_filename(".env.local").ok();
}

async fn boot_api() -> Result<Api, String> {
    let object_store_service = S3ObjectStoreService::new()
        .await
        .map_err(|err| format!("Error creating object store: {:?}", err))?;
    let code_generator_service = CodeGeneratorService::try_new(Arc::new(AsyncZipArchiverService))
        .await
        .map_err(|err| format!("Error creating code generator service: {:?}", err))?;
    Ok(Api::new(
        Arc::new(object_store_service),
        Arc::new(code_generator_service),
    ))
}

#[tokio::test]
async fn test_api() {
    env_setup();
    let api = boot_api().await;
    assert!(api.is_ok());
    let api = api.unwrap();
    let sp = api.list_supported_pallets().await;
    let sp = match sp {
        ListSupportedPalletsResponse::Ok(sp) => sp,
    };
    assert!(sp.len() > 0);
    let request_sp =
        sp.0.iter()
            .map(|(name, _)| (name.clone(), None))
            .collect::<HashMap<String, Option<HashMap<String, ParameterConfiguration>>>>();
    let req_body = "{\"name\": \"test_project\", \"pallets\": ".to_string()
        + &serde_json::to_string(&request_sp).unwrap()
        + "}";
    let project = serde_json::from_str::<NewProject>(&req_body);
    assert!(project.is_ok());
    let project = project.unwrap();
    let gp = api.generate_a_project(Json(project)).await;
    let response = match gp {
        GenerateProjectResponse::Ok(response) => response.0,
        _ => panic!("Expected Ok response"),
    };
    let status = loop {
        let status = api.get_status(Path(response)).await;
        match status {
            GetStatusResponse::Ok(sr) => match sr.0.status {
                Status::Finished => break sr.0,
                _ => tokio::time::sleep(Duration::from_millis(100)).await,
            },
            _ => panic!("Expected Ok response"),
        }
    };
    assert!(status.url.is_some());
    let response = reqwest::get(status.url.unwrap()).await;
    assert!(response.is_ok());
    let response = response.unwrap();
    assert!(response.status().is_success());
    assert!(response.content_length().unwrap() > 44000);
}
