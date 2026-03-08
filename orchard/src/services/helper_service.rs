use std::{fs, io, path::Path};

use aers_email_client::{
    ConfirmEmailTemplate, Email, EmailClient, ForgottenPasswordTemplate, PasswordUpdatedTemplate,
    WaitListConfirmationTemplate,
};
use bcrypt::{DEFAULT_COST, hash, verify};

use crate::errors::service_error::ServiceError;

#[derive(Clone)]
pub struct ServiceHelpers {}

impl ServiceHelpers {
    pub fn init() -> Self {
        Self {}
    }
}

pub trait ServiceHelpersTrait {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError>;
    fn validate_password(&self, raw_password: &str, hash: &str) -> Result<bool, ServiceError>;
    fn delete_file_if_exists(path: &str) -> io::Result<()>;
    fn send_account_confirmation_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn send_wait_list_confirmation_email(
        &self,
        user_email: &str,
        first_name: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn generate_otp(&self, user_email: &str) -> Result<String, ServiceError>;
}

impl ServiceHelpersTrait for ServiceHelpers {
    fn hash_password(&self, raw_password: &str) -> Result<String, ServiceError> {
        hash(raw_password.trim(), DEFAULT_COST).map_err(|err| {
            log::error!("operation failed due to {err}");
            ServiceError::OperationFailed
        })
    }
    fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        verify(password, hash).map_err(|err| {
            log::error!("operation failed due to {err}");
            ServiceError::OperationFailed
        })
    }

    fn delete_file_if_exists(path: &str) -> io::Result<()> {
        let file_path = Path::new(path);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    async fn send_account_confirmation_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> Result<(), ServiceError> {
        let template = ConfirmEmailTemplate::new(user_email, otp);
        let email = Email::builder()
            .subject("Confirm your account")
            .to(user_email)
            .template(template)
            .from("admin@eckko.oapp")
            .build();

        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send confirmation email due to: {err}");
            ServiceError::OperationFailed
        })?;

        log::info!("email sent to {user_email}");
        Ok(())
    }

    async fn send_forgotten_password_email(
        &self,
        user_email: &str,
        otp: &str,
    ) -> Result<(), ServiceError> {
        let template = ForgottenPasswordTemplate::new(otp, user_email);

        let email = Email::builder()
            .subject("Forgotten Password")
            .to(user_email)
            .template(template)
            .from("admin@eckko.app")
            .build();
        let email_client = EmailClient::new();

        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send forgotten password email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
    }
    async fn send_password_updated_email(
        &self,
        user_email: &str,
        template: PasswordUpdatedTemplate,
    ) -> Result<(), ServiceError> {
        let email = Email::builder()
            .subject("Password Updated")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {err}");
            ServiceError::OperationFailed
        })?;

        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send password updated email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    async fn send_welcome_email(
        &self,
        user_email: &str,
        user_name: &str,
    ) -> Result<(), ServiceError> {
        let template = aers_email_client::WelcomeTemplate::new(user_name);
        let email = Email::builder()
            .subject("Welcome to Eckko")
            .to(user_email)
            .template(template)
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send welcome email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    async fn send_wait_list_confirmation_email(
        &self,
        user_email: &str,
        first_name: &str,
    ) -> Result<(), ServiceError> {
        let template = WaitListConfirmationTemplate::new(first_name);
        let email = Email::builder()
            .subject("You're on the Wait List!")
            .to(user_email)
            .template(template)
            .from("admin@eckko.app")
            .build();
        let email_client = EmailClient::new();
        email_client.send_email(&email).map_err(|err| {
            log::error!("Failed to send wait list confirmation email due to: {err}");
            ServiceError::OperationFailed
        })?;

        Ok(())
    }

    fn generate_otp(&self, user_email: &str) -> Result<String, ServiceError> {
        let otp = aers_utils::generate_otp();
        log::info!("Generated OTP for {user_email}: {otp}");
        Ok(otp)
    }
}
