use axum::{Json, extract::State};
use axum_typed_multipart::TypedMultipart;

use crate::{
    adapters::{
        authentication::SetNewPasswordRequest,
        jwt::Claims,
        profile::UploadProfilePictureRequest,
        request::AuthenticatedRequest,
        users::{BackupEmail, PartialUserProfile},
    },
    entities::users,
    errors::service_error::ServiceError,
    response::{ApiResponse, ApiResponseBuilder},
    services::user_service::UserService,
};

use crate::services::user_service::UserServiceTrait;

pub async fn retrieve_information(
    State(user_service): State<UserService>,
    claims: Claims,
) -> Result<ApiResponse<users::Model>, ServiceError> {
    let user_data = user_service
        .retrieve_information(claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(user_data)
        .message("User's profile fetched successfully")
        .build())
}

pub async fn update_password(
    State(user_service): State<UserService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<SetNewPasswordRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    user_service
        .update_password(&data, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("User's profile fetched successfully")
        .build())
}

pub async fn update_profile_picture(
    State(user_service): State<UserService>,
    claims: Claims,
    request: TypedMultipart<UploadProfilePictureRequest>,
) -> Result<ApiResponse<users::Model>, ServiceError> {
    user_service
        .update_profile_picture(request, &claims.user_identifier)
        .await?;

    let updated_profile = user_service
        .retrieve_information(claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(updated_profile)
        .message("profile updated successfully")
        .build())
}

pub async fn update_profile(
    State(user_service): State<UserService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<PartialUserProfile>,
) -> Result<ApiResponse<users::Model>, ServiceError> {
    let updated_profile = user_service
        .update_profile(&data, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(updated_profile)
        .message("profile updated successfully")
        .build())
}

pub async fn toggle_2fa(
    State(user_service): State<UserService>,
    claims: Claims,
) -> Result<ApiResponse<()>, ServiceError> {
    let response = user_service.toggle_2fa(&claims.user_identifier).await?;

    let message = if response.enable_biometrics {
        "Two-factor authentication enabled successfully"
    } else {
        "Two-factor authentication disabled successfully"
    };

    Ok(ApiResponseBuilder::new().data(()).message(message).build())
}

pub async fn toggle_biometrics(
    State(user_service): State<UserService>,
    claims: Claims,
) -> Result<ApiResponse<()>, ServiceError> {
    let response = user_service
        .toggle_biometrics(&claims.user_identifier)
        .await?;

    let message = if response.enable_biometrics {
        "biometrics enabled successfully"
    } else {
        "biometrics disabled successfully"
    };

    Ok(ApiResponseBuilder::new().data(()).message(message).build())
}

pub async fn add_backup_email(
    State(user_service): State<UserService>,
    claims: Claims,
    Json(payload): Json<BackupEmail>,
) -> Result<ApiResponse<()>, ServiceError> {
    user_service
        .add_backup_email(&claims.user_identifier, &payload.email)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .message("backup email added")
        .build())
}
