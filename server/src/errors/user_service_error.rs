use axum::{http::StatusCode, response::IntoResponse};

use crate::response::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("{0}")]
    OperationFailed(String),
    #[error("duplicate record: {0}")]
    ConflictError(String),
}

impl UserServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConflictError(_) => StatusCode::CONFLICT,
        }
    }
}

impl IntoResponse for UserServiceError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
