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

    #[error("Invalid authentication token")]
    InvalidToken,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("GraphQL error: {0}")]
    GraphQLError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error(transparent)]
    KernelError(#[from] almond_kernel::error::KernelError),

    #[error(transparent)]
    FileSystemError(#[from] std::io::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::GraphQLError(_) => StatusCode::BAD_REQUEST,
            AppError::OperationFailed(_) => StatusCode::BAD_REQUEST,
            AppError::EnvError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::StartupError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::KernelError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::FileSystemError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
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