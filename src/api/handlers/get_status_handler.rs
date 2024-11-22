use std::sync::Arc;

use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, Enum, Object,
};
use scc::HashMap as ConcurrentHashMap;
use url::Url;
use uuid::Uuid;

use crate::services::code_generator::CodeGeneratorServiceError;

#[derive(Enum, Clone)]
pub enum Status {
    Pending,
    Finished,
}
#[derive(Object, Clone)]
pub struct StatusResponse {
    pub status: Status,
    pub url: Option<Url>,
}

#[derive(ApiResponse)]
pub enum GetStatusResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok(Json<StatusResponse>),
    #[oai(status = 200)]
    TaskNotFound(PlainText<String>),
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

pub async fn get_status_handler(
    task_status_map: Arc<
        ConcurrentHashMap<Uuid, Option<Result<String, CodeGeneratorServiceError>>>,
    >,
    task_id: Path<Uuid>,
) -> GetStatusResponse {
    match task_status_map
        .read_async(&task_id.0, |_, v| v.clone())
        .await
    {
        Some(None) => GetStatusResponse::Ok(Json(StatusResponse {
            status: Status::Pending,
            url: None,
        })),
        Some(Some(Ok(url))) => GetStatusResponse::Ok(Json(StatusResponse {
            status: Status::Finished,
            url: Url::parse(url.as_str()).ok(),
        })),
        Some(Some(Err(e))) => GetStatusResponse::InternalServerError(PlainText(format!(
            "Internal Server Error: {}",
            e
        ))),
        None => GetStatusResponse::TaskNotFound(PlainText("Task Not Found".to_string())),
    }
}
