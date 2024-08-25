use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;

#[derive(TryFromMultipart, Debug)]
pub struct ImageUploadRequest {
    pub title: String,
    pub tags: Vec<String>,
    #[form_data(limit = "8MB")]
    pub image: FieldData<NamedTempFile>,
}
