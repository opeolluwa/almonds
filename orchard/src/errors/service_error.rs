use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse};
use redis::RedisError;

use crate::errors::app_error::AppError;
use crate::errors::auth_error::AuthenticationError;
use crate::errors::database_error::DatabaseError;
use crate::errors::user_service_error::UserServiceError;
use crate::response::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("an internal error occurred")]
    OperationFailed,

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),

    #[error(transparent)]
    UserServiceError(#[from] UserServiceError),

    #[error("badly formatted request")]
    BadRequest,

    #[error("an internal error occurred")]
    AppError(#[from] AppError),

    #[error("an internal error occurred due to redis client")]
    RedisError(#[from] RedisError),

    #[error("an internal error occurred while parsing message")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    TextExtractError(#[from] aers_text_extract::TextExtractError),

    #[error(transparent)]
    TtsError(#[from] aers_piper_tts::SpeechSynthesisError),

    #[error("{0}")]
    InternalError(String),
}

impl From<String> for ServiceError {
    fn from(value: String) -> Self {
        Self::InternalError(value)
    }
}

impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ValidationError(_)
            | ServiceError::AxumFormRejection(_)
            | ServiceError::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,

            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::AppError(err) => err.status_code(),
            ServiceError::RedisError(_)
            | ServiceError::InternalError(_)
            | ServiceError::SerdeJsonError(_)
            | ServiceError::TextExtractError(_)
            | ServiceError::TtsError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            ServiceError::OperationFailed => StatusCode::UNPROCESSABLE_ENTITY,
           

            ServiceError::DatabaseError(err) => err.status_code(),
            ServiceError::AuthenticationError(err) => err.status_code(),
            ServiceError::UserServiceError(err) => err.status_code(),
        }
    }
}
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
