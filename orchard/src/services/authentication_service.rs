use std::sync::Arc;
use std::time::Duration;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::adapters::authentication::ChangePasswordRequest;
use crate::adapters::authentication::CreateUserResponse;
use crate::adapters::authentication::OnboardingRequest;
use crate::adapters::authentication::VerifyAccountRequest;
use crate::adapters::jwt::Claims;
use crate::adapters::jwt::TWENTY_FIVE_MINUTES;
use crate::adapters::repository::DatabaseInsertResult;
use crate::errors::database_error::DatabaseError;
use crate::errors::service_error::ServiceError;
use crate::events::redis::RedisClient;
use crate::events::redis::RedisClientExt;
use crate::repositories::base::Repository;
use crate::services::otp_service::OtpService;
use crate::services::otp_service::OtpServiceExt;
use crate::{
    adapters::authentication::{
        CreateUserRequest, ForgottenPasswordRequest, ForgottenPasswordResponse, LoginRequest,
        LoginResponse, RefreshTokenRequest, RefreshTokenResponse, SetNewPasswordRequest,
        SetNewPasswordResponse, VerifyAccountResponse,
    },
    errors::auth_error::AuthenticationError,
    repositories::user::{UserRepository, UserRepositoryTrait},
    services::helper_service::{ServiceHelpers, ServiceHelpersTrait},
};

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
    user_helper_service: ServiceHelpers,
    otp_service: OtpService,
}

impl AuthenticationService {
    pub fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            user_repository: UserRepository::init(db_conn),
            user_helper_service: ServiceHelpers::init(),
            otp_service: OtpService::init(db_conn),
        }
    }
}
pub trait AuthenticationServiceTrait {
    fn create_account(
        &self,
        request: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<CreateUserResponse, ServiceError>> + Send;

    fn login(
        &self,
        request: &LoginRequest,
    ) -> impl std::future::Future<Output = Result<LoginResponse, ServiceError>> + Send;

    fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
    ) -> impl std::future::Future<Output = Result<ForgottenPasswordResponse, ServiceError>> + Send;

    fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<SetNewPasswordResponse, ServiceError>> + Send;

    fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, ServiceError>> + Send;

    fn validate_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<String, ServiceError>> + Send;

    fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResponse, ServiceError>> + Send;

    fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn onboard_user(
        &self,
        claims: &Claims,
        request: &OnboardingRequest,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;

    fn verify_reset_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, ServiceError>> + Send;

    fn change_password(
        &self,
        request: &ChangePasswordRequest,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>> + Send;
}

impl AuthenticationServiceTrait for AuthenticationService {
    async fn create_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<CreateUserResponse, ServiceError> {
        if self
            .user_repository
            .find_by_email(&request.email)
            .await
            .is_some()
        {
            return Err(DatabaseError::DuplicateRecord.into());
        }

        let password_hash = self.user_helper_service.hash_password(&request.password)?;
        let user = CreateUserRequest {
            password: password_hash,
            email: request.email.clone(),
        };

        let DatabaseInsertResult {
            identifier: user_identifier,
        } = self
            .user_repository
            .create_user(&user)
            .await
            .map_err(ServiceError::from)?;

        let token = Claims::builder()
            .email(&user.email)
            .user_identifier(&user_identifier)
            .validity(TWENTY_FIVE_MINUTES)
            .build()?
            .generate_token()?;

        let otp = self
            .otp_service
            .new_otp_for_user(&user_identifier.to_string())
            .await?;
        tokio::task::spawn(async move {
            let service_helpers = ServiceHelpers::init();
            service_helpers
                .send_account_confirmation_email(&user.email, &otp)
                .await
                .unwrap_or_else(|err| {
                    log::error!("Failed to send confirmation email: {err}");
                });
        });

        Ok(CreateUserResponse { token })
    }

    async fn login(&self, request: &LoginRequest) -> Result<LoginResponse, ServiceError> {
        let Some(user) = self.user_repository.find_by_email(&request.email).await else {
            return Err(AuthenticationError::WrongCredentials.into());
        };

        let valid_password = self
            .user_helper_service
            .validate_password(&request.password, &user.password)?;
        if !valid_password {
            return Err(AuthenticationError::WrongCredentials.into());
        }

        let access_token = Claims::builder()
            .subject("access_token")
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(Duration::from_secs(150 * 60 /*15 minutes */))
            .build()?;

        let refresh_token = Claims::builder()
            .subject("refresh_token")
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(Duration::from_secs(7 * 60 * 60 /*7 hours */))
            .build()?;

        let refresh_token_out = refresh_token.generate_token()?;
        let mut redis_client = RedisClient::new().await?;
        redis_client.save_refresh_token(&refresh_token_out).await?;

        Ok(LoginResponse {
            access_token: access_token.generate_token()?,
            refresh_token: refresh_token_out,
            refresh_token_exp: refresh_token.exp,
            iat: access_token.iat,
            exp: access_token.exp,
            refresh_token_iat: refresh_token.iat,
        })
    }

    async fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, ServiceError> {
        let Some(user) = self.user_repository.find_by_email(&request.email).await else {
            return Err(ServiceError::DatabaseError(DatabaseError::RecordNotFound));
        };

        let token = Claims::builder()
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(TWENTY_FIVE_MINUTES)
            .build()?
            .generate_token()?;

        let otp = self
            .otp_service
            .new_otp_for_user(&user.identifier.to_string())
            .await?;

        tokio::task::spawn(async move {
            let service_helpers = ServiceHelpers::init();
            service_helpers
                .send_forgotten_password_email(&user.email, &otp)
                .await
                .unwrap_or_else(|err| {
                    log::error!("Failed to send confirmation email: {err}");
                });
        });

        Ok(ForgottenPasswordResponse { token })
    }

    async fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> Result<SetNewPasswordResponse, ServiceError> {
        let new_password = self.user_helper_service.hash_password(&request.password)?;

        if self
            .user_repository
            .find_by_identifier(&claims.user_identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationError::InvalidToken.into());
        };

        self.user_repository
            .update_password(&claims.user_identifier, &new_password)
            .await?;

        Ok(SetNewPasswordResponse {})
    }

    async fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, ServiceError> {
        let token = self.verify_reset_otp(claims, request).await?;
        self.user_repository
            .activate_account(&claims.user_identifier)
            .await?;
        Ok(token)
    }

    async fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, ServiceError> {
        let access_token = Claims::builder()
            .subject("access_token")
            .email(&request.email)
            .user_identifier(&request.user_identifier)
            .validity(Duration::from_secs(15 * 60 /*15 minutes */))
            .build()?;

        let refresh_token = Claims::builder()
            .subject("refresh_token")
            .email(&request.email)
            .user_identifier(&request.user_identifier)
            .validity(Duration::from_secs(7 * 60 * 60 /*7 hours */))
            .build()?;

        let refresh_token_out = refresh_token.generate_token()?;
        let mut redis_client = RedisClient::new().await?;
        redis_client.save_refresh_token(&refresh_token_out).await?;

        Ok(LoginResponse {
            access_token: access_token.generate_token()?,
            refresh_token: refresh_token_out,
            refresh_token_exp: refresh_token.exp,
            refresh_token_iat: refresh_token.iat,
            iat: access_token.iat,
            exp: access_token.exp,
        })
    }

    async fn set_avatar_url(
        &self,
        user_identifier: &Uuid,
        avatar_url: &str,
    ) -> Result<(), ServiceError> {
        self.user_repository
            .set_avatar_url(user_identifier, avatar_url)
            .await
            .map_err(ServiceError::from)
    }

    async fn onboard_user(
        &self,
        claims: &Claims,
        request: &OnboardingRequest,
    ) -> Result<(), ServiceError> {
        let Some(_) = self
            .user_repository
            .find_by_identifier(&claims.user_identifier)
            .await
        else {
            return Err(DatabaseError::RecordNotFound.into());
        };

        self.user_repository
            .onboard_user(&claims.user_identifier, request)
            .await?;
        Ok(())
    }

    async fn validate_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> Result<String, ServiceError> {
        let Some(user) = self
            .user_repository
            .find_by_identifier(&claims.user_identifier)
            .await
        else {
            return Err(ServiceError::AuthenticationError(
                AuthenticationError::InvalidToken,
            ));
        };

        let valid_otp = self
            .otp_service
            .validate_otp_for_user(&claims.user_identifier, request.otp.trim())
            .await?;

        if !valid_otp {
            return Err(ServiceError::AuthenticationError(
                AuthenticationError::InvalidOtp,
            ));
        }

        let token = Claims::builder()
            .email(&user.email)
            .user_identifier(&user.identifier)
            .validity(TWENTY_FIVE_MINUTES)
            .build()?
            .generate_token()?;

        Ok(token)
    }

    async fn verify_reset_otp(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, ServiceError> {
        let token = self.validate_otp(claims, request).await?;

        Ok(VerifyAccountResponse { token })
    }

    async fn change_password(
        &self,
        request: &ChangePasswordRequest,
        claims: &Claims,
    ) -> Result<(), ServiceError> {
        let Some(user) = self
            .user_repository
            .find_by_identifier(&claims.user_identifier)
            .await
        else {
            return Err(ServiceError::AuthenticationError(
                AuthenticationError::InvalidToken,
            ));
        };

        let valid_password = self
            .user_helper_service
            .validate_password(&request.current_password, &user.password)?;
        if !valid_password {
            return Err(AuthenticationError::WrongCredentials.into());
        }

        let new_password_hash = self
            .user_helper_service
            .hash_password(&request.new_password)?;

        self.user_repository
            .update_password(&claims.user_identifier, &new_password_hash)
            .await?;

        Ok(())
    }
}
