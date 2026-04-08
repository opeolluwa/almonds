use axum::extract::State;
use axum::http::StatusCode;

use crate::adapters::wait_list::{JoinWaitListRequest, JoinWaitListResponse};
use crate::errors::service_error::ServiceError;
use crate::middlewares::validator::ValidatedRequest;
use crate::response::{ApiResponse, ApiResponseBuilder};
use crate::services::wait_list_service::{WaitListService, WaitListServiceTrait};

pub async fn join_wait_list(
    State(wait_list_service): State<WaitListService>,
    ValidatedRequest(request): ValidatedRequest<JoinWaitListRequest>,
) -> Result<ApiResponse<JoinWaitListResponse>, ServiceError> {
    let resp = wait_list_service.join_wait_list(&request).await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("You've been added to the wait list")
        .data(resp)
        .build())
}
