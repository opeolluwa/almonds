use axum_typed_multipart::{FieldData, TryFromMultipart};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use uuid::Uuid;
use validator::Validate;

#[derive(TryFromMultipart)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct UploadAssetRequest {
    #[form_data(limit = "25MiB")]
    pub document: FieldData<NamedTempFile>,
    pub playlist_identifier: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAudioBook {
    pub name: String,
    pub src: String,
    pub user_identifier: Uuid,
    pub playlist_identifier: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioBook {
    pub identifier: Uuid,
    pub user_identifier: Uuid,
    pub audio_source: String,
    pub file_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_played: Option<DateTime<Utc>>,
    pub playlist_identifier: Option<Uuid>,
    pub starred: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateAudioBookRequest {
    pub file_name: String,
    pub src: String,
    pub playlist_identifier: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddBookToPlaylistRequest {
    pub book_identifier: Uuid,
    pub playlist_identifier: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MarkFavoriteRequest {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkFavoriteResponse {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindAudioBook {
    pub user_identifier: Uuid,
    pub identifier: Uuid,
}
