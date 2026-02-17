use crate::adapters::dto::jwt::Claims;
use crate::adapters::requests::auth::VerifyAccountRequest;
use crate::adapters::response::api_response::ApiResponseBuilder;
use crate::adapters::response::auth::{ForgottenPasswordResponse, RefreshTokenResponse};
use crate::middlewares::validator::ValidatedRequest;
use crate::{
    adapters::{
        requests::auth::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, SetNewPasswordRequest,
        },
        response::{
            api_response::ApiResponse,
            auth::{CreateUserResponse, LoginResponse, VerifyAccountResponse},
        },
    },
    errors::auth_service_error::AuthenticationServiceError,
    services::auth_service::{AuthenticationService, AuthenticationServiceTrait},
};
use axum::extract::State;
use axum::http::StatusCode;

pub async fn create_account(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<CreateUserRequest>,
) -> Result<ApiResponse<CreateUserResponse>, AuthenticationServiceError> {
    auth_service.create_account(&request).await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("Account created successfully")
        .build())
}
pub async fn login(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, AuthenticationServiceError> {
    let login_response = auth_service.login(&request).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(login_response)
        .message("logged in successfully")
        .build())
}
pub async fn verify_account(
    State(auth_service): State<AuthenticationService>,
    claims: Claims,
    ValidatedRequest(request): ValidatedRequest<VerifyAccountRequest>,
) -> Result<ApiResponse<VerifyAccountResponse>, AuthenticationServiceError> {
    let verify_account_response = auth_service.verify_account(&claims, &request).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .build())
}
pub async fn forgotten_password(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<ApiResponse<ForgottenPasswordResponse>, AuthenticationServiceError> {
    let forgotten_password_response = auth_service.forgotten_password(&request).await?;

    Ok(ApiResponseBuilder::new()
        .data(forgotten_password_response)
        .message("account retrival instructions has been sent to the registered email address")
        .build())
}

pub async fn set_new_password(
    State(auth_service): State<AuthenticationService>,
    claims: Claims,
    ValidatedRequest(request): ValidatedRequest<SetNewPasswordRequest>,
) -> Result<ApiResponse<()>, AuthenticationServiceError> {
    let _ = auth_service.set_new_password(&request, &claims).await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .message("password updated successfully")
        .build())
}

pub async fn request_refresh_token(
    State(auth_service): State<AuthenticationService>,
    claims: Claims,
) -> Result<ApiResponse<RefreshTokenResponse>, AuthenticationServiceError> {
    let refresh_token_response = auth_service.request_refresh_token(&claims).await?;

    Ok(ApiResponseBuilder::new()
        .data(refresh_token_response)
        .message("token updated successfully")
        .build())
}
