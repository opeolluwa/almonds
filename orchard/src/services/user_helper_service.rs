use crate::errors::user_service_error::UserServiceError;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Clone)]
pub struct UserHelperService {}

impl UserHelperService {
    pub fn init() -> Self {
        Self {}
    }
}

pub trait UserHelperServiceTrait {
    fn hash_password(&self, raw_password: &str) -> Result<String, UserServiceError>;
    fn validate_password(&self, raw_password: &str, hash: &str) -> Result<bool, UserServiceError>;
}

impl UserHelperServiceTrait for UserHelperService {
    fn hash_password(&self, raw_password: &str) -> Result<String, UserServiceError> {
        hash(raw_password.trim(), DEFAULT_COST)
            .map_err(|err| UserServiceError::OperationFailed(err.to_string()))
    }
    fn validate_password(&self, password: &str, hash: &str) -> Result<bool, UserServiceError> {
        verify(password, hash).map_err(|err| UserServiceError::OperationFailed(err.to_string()))
    }
}
