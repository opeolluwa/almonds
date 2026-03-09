use almond_kernel::error::KernelError;
use axum::{http::StatusCode, response::IntoResponse};

use crate::errors::app_error::AppError;
use crate::response::ApiResponseBuilder;

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("Invalid email or password")]
    WrongCredentials,
    #[error("Missing or invalid authorization headers")]
    MissingCredentials,
    #[error("Invalid authorization token")]
    InvalidToken,
    #[error("Invalid or expired OTP")]
    InvalidOtp,
    #[error(transparent)]
    AppError(#[from] AppError),
    #[error("error processing authorization token")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    KernelError(#[from] KernelError),
}

impl AuthenticationError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthenticationError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthenticationError::MissingCredentials => StatusCode::BAD_REQUEST,
            AuthenticationError::ValidationError(_) => StatusCode::UNAUTHORIZED,
            AuthenticationError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthenticationError::InvalidOtp => StatusCode::UNAUTHORIZED,
            AuthenticationError::AppError(err) => err.status_code(),
            AuthenticationError::JwtError(_) | _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for AuthenticationError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
