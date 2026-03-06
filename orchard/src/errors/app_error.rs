use axum::{http::StatusCode, response::IntoResponse};

use crate::response::ApiResponseBuilder;
use crate::response::EmptyResponseBody;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("App failed to start up due to {0}")]
    StartupError(String),
    #[error("Error parsing env due to {0}")]
    EnvError(String),
    #[error("{0}")]
    OperationFailed(String),
    #[error(transparent)]
    EnvExtractError(#[from] aers_utils::errors::Error),
    #[error(transparent)]
    FileSystemError(#[from] std::io::Error),
    #[error(transparent)]
    DbError(#[from] migration::DbErr),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<EmptyResponseBody>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
