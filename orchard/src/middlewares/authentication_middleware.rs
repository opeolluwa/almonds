use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{adapters::jwt::Claims, errors::auth_error::AuthenticationError};

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthenticationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|err| {
                log::error!("failed to extract authorization header due to {err}");
                AuthenticationError::MissingCredentials
            })?;
        // Decode the user data
        let token = bearer.token();

        Claims::from_token(token)
    }
}
