use crate::errors::common_service_error::ServiceError;
use axum::{Form, Json};
use axum::extract::rejection::FormRejection;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServiceError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(ServiceError::AxumJsonRejection)?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}
