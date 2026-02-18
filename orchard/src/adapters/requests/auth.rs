use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::adapters::dto::jwt::Claims;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "password is required"))]
    pub password: String,
    #[validate(length(min = 1, message = "first name cannot be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "last name cannot be empty "))]
    pub last_name: String,
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
