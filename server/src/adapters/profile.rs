use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;

#[derive(TryFromMultipart)]
#[try_from_multipart(rename_all = "camelCase")]
pub struct UploadProfilePictureRequest {
    #[form_data(limit = "1MiB")]
    pub image: FieldData<NamedTempFile>,
}
