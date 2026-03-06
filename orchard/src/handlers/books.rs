use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum_typed_multipart::TypedMultipart;
use uuid::Uuid;

use crate::adapters::audio_books::{
    AddBookToPlaylistRequest, MarkFavoriteResponse, UpdateBookRequest, UploadAssetRequest,
};
use crate::adapters::jwt::Claims;
use crate::adapters::pagination::{PaginatedResponse, PaginationParams};
use crate::adapters::request::AuthenticatedRequest;
use crate::entities::audio_books;
use crate::errors::service_error::ServiceError;
use crate::response::{ApiResponse, ApiResponseBuilder};
use crate::services::audio_book_service::{AudioBooksService, AudioBooksServiceExt};

pub async fn create_new_book(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    request: TypedMultipart<UploadAssetRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    audio_book_service.create_new(request, &claims).await?;

    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(())
        .message("You request is being processed, you'll get an email when it is done")
        .build())
}

pub async fn fetch_book(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Path(book_identifier): Path<Uuid>,
) -> Result<ApiResponse<audio_books::Model>, ServiceError> {
    let book = audio_book_service
        .fetch_one(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(book)
        .status_code(StatusCode::OK)
        .message("Book retrieved successfully")
        .build())
}

pub async fn add_to_playlist(
    State(audio_book_service): State<AudioBooksService>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<AddBookToPlaylistRequest>,
) -> Result<ApiResponse<()>, ServiceError> {
    audio_book_service
        .add_to_playlist(&data, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .status_code(StatusCode::OK)
        .message("Book successfully added to playlist")
        .build())
}

pub async fn remove_from_playlist(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Path(book_identifier): Path<Uuid>,
) -> Result<ApiResponse<()>, ServiceError> {
    audio_book_service
        .remove_from_playlist(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .data(())
        .status_code(StatusCode::OK)
        .message("Book successfully removed to playlist")
        .build())
}

pub async fn update_book(
    State(audio_book_service): State<AudioBooksService>,
    Path(book_identifier): Path<Uuid>,
    AuthenticatedRequest { data, claims }: AuthenticatedRequest<UpdateBookRequest>,
) -> Result<ApiResponse<audio_books::Model>, ServiceError> {
    let book = audio_book_service
        .update_book(data, &book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("book updated successfully")
        .data(book)
        .build())
}

pub async fn delete_book(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Path(book_identifier): Path<Uuid>,
) -> Result<ApiResponse<()>, ServiceError> {
    audio_book_service
        .delete_book(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("Book deleted successfully")
        .build())
}

pub async fn mark_favorite(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Path(book_identifier): Path<Uuid>,
) -> Result<ApiResponse<MarkFavoriteResponse>, ServiceError> {
    audio_book_service
        .mark_favorite(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("Book Starred successfully")
        .build())
}

pub async fn unmark_favorite(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Path(book_identifier): Path<Uuid>,
) -> Result<ApiResponse<MarkFavoriteResponse>, ServiceError> {
    audio_book_service
        .unmark_favorite(&book_identifier, &claims.user_identifier)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("Book remove from starred successfully")
        .build())
}

pub async fn fetch_favorites(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Query(pagination_params): Query<PaginationParams>,
) -> Result<ApiResponse<PaginatedResponse<Vec<audio_books::Model>>>, ServiceError> {
    let data = audio_book_service
        .fetch_favorites(&claims.user_identifier, &pagination_params)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("Book remove from starred successfully")
        .data(data)
        .build())
}

pub async fn find_many_books(
    State(audio_book_service): State<AudioBooksService>,
    claims: Claims,
    Query(pagination_params): Query<PaginationParams>,
) -> Result<ApiResponse<PaginatedResponse<Vec<audio_books::Model>>>, ServiceError> {
    let data = audio_book_service
        .fetch_many(&claims.user_identifier, &pagination_params)
        .await?;

    Ok(ApiResponseBuilder::new()
        .message("Book retrieved from starred successfully")
        .data(data)
        .build())
}
