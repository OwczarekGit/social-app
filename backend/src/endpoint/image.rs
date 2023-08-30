use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use tempfile::NamedTempFile;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(test))
}

pub async fn test(
    TypedMultipart(request): TypedMultipart<ImageUploadRequest>,
) -> impl IntoResponse {
    dbg!(request);
}

#[derive(TryFromMultipart, Debug)]
pub struct ImageUploadRequest {
    title: String,
    tags: Vec<String>,
    #[form_data(limit = "5MB")]
    image: FieldData<NamedTempFile>,
}