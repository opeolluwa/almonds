use std::fmt::Debug;

use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Request},
};
use serde::{Serialize, de::DeserializeOwned};
use validator::Validate;

use crate::{
    adapters::jwt::Claims,
    errors::{auth_error::AuthenticationError, service_error::ServiceError},
};

pub struct AuthenticatedRequest<T>
where
    T: Debug + Serialize + DeserializeOwned + Validate,
{
    pub data: T,
    pub claims: Claims,
}

impl<S, T> FromRequest<S> for AuthenticatedRequest<T>
where
    T: Send + Serialize + DeserializeOwned + Debug + Validate,
    Claims: FromRequestParts<S> + Send,
    S: Send + Sync,
{
    type Rejection = ServiceError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        let claims: crate::adapters::jwt::Claims =
            Claims::from_request_parts(&mut parts.clone(), state)
                .await
                .map_err(|_| AuthenticationError::MissingCredentials)?;

        let Json(data) = Json::<T>::from_request(Request::from_parts(parts, body), state)
            .await
            .map_err(ServiceError::from)?;

        data.validate().map_err(ServiceError::from)?;

        Ok(Self { data, claims })
    }
}
