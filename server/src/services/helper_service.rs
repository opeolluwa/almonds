use std::io;

use async_trait::async_trait;

use crate::errors::service_error::ServiceError;

pub struct PasswordUpdatedTemplate;

#[derive(Clone)]
pub struct ServiceHelpers {}

impl ServiceHelpers {
    pub fn init() -> Self {
        Self {}
    }
}

#[async_trait]
pub trait ServiceHelpersTrait {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError>;
    fn validate_password(&self, raw_password: &str, hash: &str) -> Result<bool, ServiceError>;
    fn delete_file_if_exists(path: &str) -> io::Result<()>;

    async fn send_account_confirmation_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> Result<(), ServiceError>;

    async fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> Result<(), ServiceError>;

    async fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> Result<(), ServiceError>;

    async fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> Result<(), ServiceError>;

    async fn send_wait_list_confirmation_email(
        &self,
        user_email: &str,
        first_name: &str,
    ) -> Result<(), ServiceError>;

    fn generate_otp(&self, user_email: &str) -> Result<String, ServiceError>;
}

#[async_trait]
impl ServiceHelpersTrait for ServiceHelpers {
    fn hash_password(&self, _raw_password: &str) -> Result<String, ServiceError> {
        unimplemented!()
    }

    fn validate_password(&self, _password: &str, _hash: &str) -> Result<bool, ServiceError> {
        unimplemented!()
    }

    fn delete_file_if_exists(_path: &str) -> io::Result<()> {
        unimplemented!()
    }

    async fn send_account_confirmation_email(
        &self,
        _user_email: &str,
        _otp: &str,
    ) -> Result<(), ServiceError> {
        unimplemented!()
    }

    async fn send_forgotten_password_email(
        &self,
        _user_email: &str,
        _otp: &str,
    ) -> Result<(), ServiceError> {
        unimplemented!()
    }

    async fn send_password_updated_email(
        &self,
        _user_email: &str,
        _template: PasswordUpdatedTemplate,
    ) -> Result<(), ServiceError> {
        unimplemented!()
    }

    async fn send_welcome_email(
        &self,
        _user_email: &str,
        _user_name: &str,
    ) -> Result<(), ServiceError> {
        unimplemented!()
    }

    async fn send_wait_list_confirmation_email(
        &self,
        _user_email: &str,
        _first_name: &str,
    ) -> Result<(), ServiceError> {
        unimplemented!()
    }

    fn generate_otp(&self, _user_email: &str) -> Result<String, ServiceError> {
        unimplemented!()
    }
}
