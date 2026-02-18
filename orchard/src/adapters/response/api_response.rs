use std::fmt::Debug;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct ApiResponse<T: Serialize> {
    message: String,
    data: Option<T>,
    status_code: StatusCode,
}

pub type EmptyResponseBody = ();

#[derive(Debug)]
pub struct ApiResponseBuilder<T: Serialize> {
    status_code: StatusCode,
    message: Option<String>,
    data: Option<T>,
}

impl<T> Default for ApiResponseBuilder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            status_code: StatusCode::OK,
            message: None,
            data: None,
        }
    }
}

impl<T> ApiResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            message: self.message.unwrap_or_default(),
            data: self.data,
            status_code: self.status_code,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Json(json!({
          "message":self.message,
          "data":self.data
        }));
        (self.status_code, body).into_response()
    }
}
