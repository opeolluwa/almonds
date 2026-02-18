use crate::adapters::response::api_response::{ApiResponse, ApiResponseBuilder};
use crate::services::root_service::RootServiceTrait;
use crate::{errors::app_error::AppError, services::root_service::RootService};
use axum::extract::State;

pub async fn health_check(
    State(root_service): State<RootService>,
) -> Result<ApiResponse<()>, AppError> {
    root_service.health_check()?;
    Ok(ApiResponseBuilder::new()
        .message("service is healthy")
        .build())
}
