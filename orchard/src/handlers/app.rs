use crate::errors::app_error::AppError;
use crate::response::{ApiResponse, ApiResponseBuilder, EmptyResponseBody};

pub async fn health_check() -> Result<ApiResponse<EmptyResponseBody>, AppError> {
    Ok(ApiResponseBuilder::new()
        .message("service is healthy")
        .build())
}
