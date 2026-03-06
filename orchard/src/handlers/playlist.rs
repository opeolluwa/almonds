use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    adapters::{
        jwt::Claims,
        pagination::PaginationParams,
        playlist::{CreatePlaylistRequest, UpdatePlaylistRequest},
        request::AuthenticatedRequest,
    },
    entities::playlists,
    errors::service_error::ServiceError,
    response::{ApiResponse, ApiResponseBuilder},
    services::playlist_service::{PlaylistService, PlaylistServiceExt},
};

pub async fn create_playlist(
    State(playlist_service): State<PlaylistService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<CreatePlaylistRequest>,
) -> Result<ApiResponse<playlists::Model>, ServiceError> {
    let playlist_identifier = playlist_service
        .create_new_playlist(&data, &claims.user_identifier)
        .await?;

    let playlist = playlist_service
        .fetch_one(&playlist_identifier, &claims.user_identifier)
        .await?
        .ok_or(ServiceError::OperationFailed)?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("playlist created successfully")
        .data(playlist)
        .build())
}

pub async fn retrieve_playlist(
    State(playlist_service): State<PlaylistService>,
    claims: Claims,
    Path(playlist_identifier): Path<Uuid>,
) -> Result<ApiResponse<playlists::Model>, ServiceError> {
    let playlist = playlist_service
        .fetch_one(&playlist_identifier, &claims.user_identifier)
        .await?
        .ok_or(ServiceError::OperationFailed)?;

    Ok(ApiResponseBuilder::new()
        .message("playlist fetched successfully")
        .data(playlist)
        .build())
}

pub async fn delete_playlist(
    State(playlist_service): State<PlaylistService>,
    claims: Claims,
    Path(playlist_identifier): Path<Uuid>,
) -> Result<ApiResponse<()>, ServiceError> {
    playlist_service
        .delete_playlist(&playlist_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("playlist fetched successfully")
        .build())
}

pub async fn update_playlist(
    State(playlist_service): State<PlaylistService>,
    claims: Claims,
    Path(playlist_identifier): Path<Uuid>,
    Json(request): Json<UpdatePlaylistRequest>,
) -> Result<ApiResponse<playlists::Model>, ServiceError> {
    let playlist = playlist_service
        .update_playlist(&request, &playlist_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(playlist)
        .message("playlist updated successfully")
        .build())
}

pub async fn list_playlists(
    State(playlist_service): State<PlaylistService>,
    claims: Claims,
    Query(pagination): Query<PaginationParams>,
) -> Result<ApiResponse<Vec<playlists::Model>>, ServiceError> {
    let playlists = playlist_service
        .fetch_many(&pagination, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("playlists fetched successfully")
        .data(playlists)
        .build())
}
