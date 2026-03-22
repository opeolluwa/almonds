use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::adapters::jwt::Claims;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ForgottenPasswordRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNewPasswordRequest {
    #[validate(length(min = 1, message = "password cannot be empty"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "password does  not match"))]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerifyAccountRequest {
    pub otp: String,
}

pub type RefreshTokenRequest = Claims;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub iat: i64,
    pub exp: i64,
    pub refresh_token_exp: i64,
    pub refresh_token_iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgottenPasswordResponse {
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNewPasswordResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyAccountResponse {
    pub token: String,
}

pub type RefreshTokenResponse = LoginResponse;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingRequest {
    #[validate(length(min = 1, message = "first name cannot be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "last name cannot be empty "))]
    pub last_name: String,
    pub username: Option<String>,
    #[validate(length(min = 1, message = "country is required"))]
    pub country_identifier: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}
