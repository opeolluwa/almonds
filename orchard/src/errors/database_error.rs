use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;

use crate::response::ApiResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Record not found: The requested record does not exist or has been deleted.")]
    RecordNotFound,
    #[error("Duplicate Record: The information you provided already exists.")]
    DuplicateRecord,
    #[error("Operation failed due to {0}")]
    OperationFailed(String),
    #[error(transparent)]
    SeaOrmError(#[from] DbErr),
}

impl DatabaseError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            DatabaseError::RecordNotFound => StatusCode::NOT_FOUND,
            DatabaseError::DuplicateRecord => StatusCode::CONFLICT,
            DatabaseError::OperationFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
            DatabaseError::SeaOrmError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
