use axum::extract::State;
use axum::http::StatusCode;

use crate::adapters::authentication::{
    ChangePasswordRequest, ForgottenPasswordResponse, RefreshTokenResponse,
};
use crate::adapters::authentication::{OnboardingRequest, VerifyAccountRequest};
use crate::adapters::jwt::Claims;
use crate::adapters::request::AuthenticatedRequest;
use crate::errors::service_error::ServiceError;
use crate::middlewares::validator::ValidatedRequest;
use crate::response::ApiResponseBuilder;
use crate::{
    adapters::authentication::{
        CreateUserRequest, CreateUserResponse, ForgottenPasswordRequest, LoginRequest,
        LoginResponse, SetNewPasswordRequest, VerifyAccountResponse,
    },
    response::ApiResponse,
    services::authentication_service::{AuthenticationService, AuthenticationServiceTrait},
};

pub async fn create_account(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<CreateUserRequest>,
) -> Result<ApiResponse<CreateUserResponse>, ServiceError> {
    let resp = auth_service.create_account(&request).await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("Account created successfully")
        .data(resp)
        .build())
}
pub async fn login(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, ServiceError> {
    let login_response = auth_service.login(&request).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(login_response)
        .message("logged in successfully")
        .build())
}
pub async fn verify_account(
    State(auth_service): State<AuthenticationService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<VerifyAccountRequest>,
) -> Result<ApiResponse<VerifyAccountResponse>, ServiceError> {
    let verify_account_response = auth_service.verify_account(&claims, &data).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .message("Account verified successfully")
        .build())
}
pub async fn forgotten_password(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<ApiResponse<ForgottenPasswordResponse>, ServiceError> {
    let forgotten_password_response = auth_service.forgotten_password(&request).await?;

    Ok(ApiResponseBuilder::new()
        .data(forgotten_password_response)
        .message("account retrival instructions has been sent to the registered email address")
        .build())
}

pub async fn set_new_password(
    State(auth_service): State<AuthenticationService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<SetNewPasswordRequest>, // claims: Claims,
) -> Result<ApiResponse<()>, ServiceError> {
    let _ = auth_service.set_new_password(&data, &claims).await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .message("password updated successfully")
        .build())
}

pub async fn request_refresh_token(
    State(auth_service): State<AuthenticationService>,
    claims: Claims,
) -> Result<ApiResponse<RefreshTokenResponse>, ServiceError> {
    let refresh_token_response = auth_service.request_refresh_token(&claims).await?;

    Ok(ApiResponseBuilder::new()
        .data(refresh_token_response)
        .message("token updated successfully")
        .build())
}

pub async fn logout() -> Result<ApiResponse<()>, ServiceError> {
    todo!()
}

pub async fn onboard_user(
    State(auth_service): State<AuthenticationService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<OnboardingRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    auth_service.onboard_user(&claims, &data).await?;

    Ok(ApiResponseBuilder::new()
        .message("profile updated successfully")
        .build())
}

pub async fn verify_reset_otp(
    State(auth_service): State<AuthenticationService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<VerifyAccountRequest>,
) -> Result<ApiResponse<VerifyAccountResponse>, ServiceError> {
    let verify_account_response = auth_service.verify_reset_otp(&claims, &data).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .message("OTP verified successfully")
        .build())
}

pub async fn change_password(
    State(user_service): State<AuthenticationService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<ChangePasswordRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    user_service.change_password(&data, &claims).await?;

    Ok(ApiResponseBuilder::new()
        .message("User's password changed successfully")
        .build())
}
