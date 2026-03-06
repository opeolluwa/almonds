use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;
use ts_rs::TS;

#[derive(TryFromMultipart, TS)]
#[try_from_multipart(rename_all = "camelCase")]
#[ts(export)]
pub struct UploadProfilePictureRequest {
    #[form_data(limit = "1MiB")]
    #[ts(type = "File")]
    pub image: FieldData<NamedTempFile>,
}
