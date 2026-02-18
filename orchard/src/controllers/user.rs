use axum::extract::State;

use crate::{
    adapters::{
        dto::{jwt::Claims, user::UserDto},
        response::api_response::{ApiResponse, ApiResponseBuilder},
    },
    errors::user_service_error::UserServiceError,
    services::user_service::UserService,
};

use crate::services::user_service::UserServiceTrait;
pub async fn retrieve_information(
    State(user_service): State<UserService>,
    claim: Claims,
) -> Result<ApiResponse<UserDto>, UserServiceError> {
    let user_data = user_service.retrieve_information(claim.identifier).await?;

    Ok(ApiResponseBuilder::new()
        .data(user_data)
        .message("User's profile fetched successfully")
        .build())
}
