use std::fmt::Debug;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::errors::service_error::ServiceError;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Debug + Serialize + DeserializeOwned,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    data: Option<T>,

    #[serde(skip)]
    status_code: StatusCode,
}

impl From<ServiceError> for ApiResponse<()> {
    fn from(value: ServiceError) -> Self {
        ApiResponse {
            message: None,
            data: Some(()),
            status_code: value.into_response().status(),
        }
    }
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

    pub fn build(self) -> ApiResponse<T>
    where
        T: Debug + Serialize + DeserializeOwned,
    {
        ApiResponse {
            message: self.message,
            data: self.data,
            status_code: self.status_code,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Debug + Serialize + DeserializeOwned,
{
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

impl<T> ApiResponse<T>
where
    T: Debug + Serialize + DeserializeOwned,
{
    pub fn builder() -> ApiResponseBuilder<T> {
        ApiResponseBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        count: u32,
    }

    #[test]
    fn serialize_with_data_only() {
        let response = ApiResponse::builder()
            .data(TestData {
                name: "example".into(),
                count: 3,
            })
            .build();

        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "example",
                "count": 3
            })
        );
    }

    #[test]
    fn serialize_with_message_only() {
        let response = ApiResponse::<()>::builder()
            .message("something happened")
            .build();

        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(
            json,
            json!({
                "message": "something happened"
            })
        );
    }

    #[test]
    fn serialize_with_message_and_data() {
        let response = ApiResponse::builder()
            .message("ok")
            .data(TestData {
                name: "test".into(),
                count: 1,
            })
            .build();

        let json = serde_json::to_value(&response).unwrap();

        assert_eq!(
            json,
            json!({
                "message": "ok",
                "name": "test",
                "count": 1
            })
        );
    }

    #[test]
    fn status_code_is_not_serialized() {
        let response = ApiResponse::<()>::builder()
            .status_code(StatusCode::CREATED)
            .message("created")
            .build();

        let json = serde_json::to_value(&response).unwrap();

        assert!(json.get("status_code").is_none());
    }

    #[test]
    fn into_response_sets_correct_status() {
        let response = ApiResponse::<()>::builder()
            .status_code(StatusCode::NOT_FOUND)
            .message("not found")
            .build()
            .into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
