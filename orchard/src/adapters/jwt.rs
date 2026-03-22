use std::fmt::Display;
use std::time::Duration;

use almond_kernel::utils::extract_env;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::auth_error::AuthenticationError;

// Fixed duration constants (corrected from your original)
pub const FIVE_MINUTES: Duration = Duration::from_secs(5 * 60);
pub const TWENTY_FIVE_MINUTES: Duration = Duration::from_secs(25 * 60);
pub const TEN_MINUTES: Duration = Duration::from_secs(10 * 60);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) iat: i64,
    pub(crate) exp: i64,
    pub(crate) email: String,
    pub(crate) user_identifier: Uuid,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user_identifier: {}\nemail {}\nexp: {}",
            self.user_identifier, self.email, self.exp
        )
    }
}

pub struct ClaimsBuilder {
    claims: Claims,
    validity: Option<Duration>,
}

impl ClaimsBuilder {
    pub fn new() -> Self {
        Self {
            claims: Claims::default(),
            validity: None,
        }
    }

    pub fn email(mut self, email: &str) -> Self {
        self.claims.email = email.into();
        self
    }

    pub fn user_identifier(mut self, user_id: &Uuid) -> Self {
        self.claims.user_identifier = *user_id;
        self
    }

    pub fn subject<T: Into<String>>(mut self, subject: T) -> Self {
        self.claims.sub = subject.into();
        self
    }

    pub fn validity(mut self, duration: Duration) -> Self {
        self.validity = Some(duration);
        self
    }

    pub fn issued_at(mut self, timestamp: i64) -> Self {
        self.claims.iat = timestamp;
        self
    }

    pub fn expires_at(mut self, timestamp: i64) -> Self {
        self.claims.exp = timestamp;
        self
    }

    pub fn build(mut self) -> Result<Claims, AuthenticationError> {
        if self.claims.email.is_empty() {
            return Err(AuthenticationError::ValidationError(
                "Email is required".to_string(),
            ));
        }

        if self.claims.user_identifier == Uuid::nil() {
            return Err(AuthenticationError::ValidationError(
                "User identifier is required".to_string(),
            ));
        }

        let now = chrono::Utc::now().timestamp();

        // Set issued at time if not already set
        if self.claims.iat == 0 {
            self.claims.iat = now;
        }

        // Set expiration time based on validity or default
        if self.claims.exp == 0 {
            let validity = self.validity.unwrap_or(TWENTY_FIVE_MINUTES);
            self.claims.exp = now + validity.as_secs() as i64;
        }

        // Set subject to user_identifier if not set
        if self.claims.sub.is_empty() {
            self.claims.sub = self.claims.user_identifier.to_string();
        }

        Ok(self.claims)
    }

    /// Convenience method to build claims and generate token in one step
    pub fn build_and_sign(self) -> Result<String, AuthenticationError> {
        let claims = self.build()?;
        claims.generate_token()
    }
}

impl Default for ClaimsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

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

impl Claims {
    /// Create new Claims with email and user_identifier
    pub fn new(email: &str, user_identifier: &Uuid) -> Self {
        Self {
            email: email.to_string(),
            user_identifier: user_identifier.to_owned(),
            ..Default::default()
        }
    }

    /// Create a ClaimsBuilder for more flexible construction
    pub fn builder() -> ClaimsBuilder {
        ClaimsBuilder::new()
    }

    /// Generate a JWT token from the claims
    pub fn generate_token(&self) -> Result<String, AuthenticationError> {
        let now = chrono::Utc::now().timestamp();

        let claim = Claims {
            email: self.email.clone(),
            user_identifier: self.user_identifier,
            sub: if self.sub.is_empty() {
                self.user_identifier.to_string()
            } else {
                self.sub.clone()
            },
            iat: if self.iat == 0 { now } else { self.iat },
            exp: if self.exp == 0 {
                now + TWENTY_FIVE_MINUTES.as_secs() as i64
            } else {
                self.exp
            },
        };

        let secret = extract_env::<String>("JWT_SIGNING_KEY")?;
        tracing::debug!("Generating token with claims: {}", claim);
        tracing::debug!("Using secret key of length: {}", secret);
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());

        Ok(encode(&Header::default(), &claim, &encoding_key)?)
    }

    /// Generate token with custom validity (legacy method for backward compatibility)
    pub fn generate_token_with_validity(
        &self,
        validity: Duration,
    ) -> Result<String, AuthenticationError> {
        let now = chrono::Utc::now().timestamp();
        let claim = Claims {
            email: self.email.clone(),
            user_identifier: self.user_identifier,
            iat: now,
            exp: now + validity.as_secs() as i64,
            sub: self.sub.to_owned(),
        };

        let secret = extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationError::from)?;
        let encoding_key = Keys::new(secret.as_bytes()).encoding;

        let token =
            encode(&Header::default(), &claim, &encoding_key).map_err(AuthenticationError::from)?;

        Ok(token)
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        self.exp < now
    }

    /// Get time remaining until expiration
    pub fn time_until_expiry(&self) -> Duration {
        let now = chrono::Utc::now().timestamp();
        let remaining = (self.exp - now).max(0) as u64;
        Duration::from_secs(remaining)
    }

    pub fn from_token(token: &str) -> Result<Self, AuthenticationError> {
        let secret = extract_env::<String>("JWT_SIGNING_KEY")?;

        tracing::debug!("Decoding token: {}", token);
        tracing::debug!("Using secret key of length: {}", secret);

        let decoding_key = Keys::new(secret.as_bytes()).decoding;

        let jwt_validation = Validation::default();

        let token_data =
            decode::<Claims>(token, &decoding_key, &jwt_validation).map_err(|err| {
                log::error!("failed to decode JWT token due to {err}");
                AuthenticationError::InvalidToken
            })?;

        Ok(token_data.claims)
    }
}
