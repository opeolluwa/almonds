use crate::adapters::response::api_response::ApiResponseBuilder;
use crate::errors::{
    app_error::AppError, common_service_error::ServiceError, user_service_error::UserServiceError,
};
use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationServiceError {
    #[error("Invalid email or password")]
    WrongCredentials,
    #[error("Missing authorization headers")]
    MissingCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
    #[error(transparent)]
    UserServiceError(#[from] UserServiceError),
    #[error(transparent)]
    AppError(#[from] AppError),
    #[error("error processing authorization token")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

impl AuthenticationServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthenticationServiceError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthenticationServiceError::MissingCredentials => StatusCode::BAD_REQUEST,

            AuthenticationServiceError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthenticationServiceError::ServiceError(err) => err.status_code(),
            AuthenticationServiceError::UserServiceError(err) => err.status_code(),
            AuthenticationServiceError::AppError(err) => err.status_code(),
            AuthenticationServiceError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for AuthenticationServiceError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
