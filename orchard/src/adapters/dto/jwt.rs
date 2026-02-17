use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::shared::extract_env::extract_env;

pub const _FIVE_MINUTES: Duration = Duration::from_secs(5 * 60 * 60);
pub const TWENTY_FIVE_MINUTES: Duration = Duration::from_secs(26 * 60 * 60);
pub const TEN_MINUTES: Duration = Duration::from_secs(10 * 60 * 60);

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtCredentials {
    pub email: String,
    pub identifier: Uuid,
}

pub type Claims = JwtCredentials;

pub struct Keys {
    encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Claim {
    pub email: String,
    pub identifier: String,
    pub iat: i64,
    pub exp: i64,
}

impl JwtCredentials {
    pub fn new(email: &str, identifier: &Uuid) -> Self {
        Self {
            email: email.to_string(),
            identifier: identifier.to_owned(),
        }
    }

    pub fn generate_token(&self, validity: Duration) -> Result<String, AuthenticationServiceError> {
        let now = chrono::Utc::now().timestamp();
        let claim = Claim {
            email: self.email.to_string(),
            identifier: self.identifier.to_string(),
            iat: now,
            exp: now + validity.as_secs() as i64,
        };

        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let encoding_key = Keys::new(secret.as_bytes()).encoding;
        let token = encode(&Header::default(), &claim, &encoding_key)
            .map_err(AuthenticationServiceError::from)?;

        Ok(token)
    }
}
