use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeUsernameRequest {
    pub username: String,
}

#[derive(TryFromMultipart, Debug)]
pub struct ChangeProfilePictureRequest {
    #[form_data(limit = "5MB")]
    pub image: FieldData<NamedTempFile>,
}
