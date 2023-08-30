use std::io::{Cursor, Read};
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use image::io::Reader;
use tempfile::NamedTempFile;

use crate::{ActiveUser, AppState};
use crate::service::image::ImageService;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(share_image).get(get_all_tags))
}

pub async fn share_image(
    Extension(user): Extension<ActiveUser>,
    State(image_service): State<ImageService>,
    TypedMultipart(request): TypedMultipart<ImageUploadRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut image_bytes = vec![];
    request.image.contents.as_file().read_to_end(&mut image_bytes)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let image = Reader::new(Cursor::new(&mut image_bytes))
        .with_guessed_format()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .decode()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    image_service.upload_image(user.id, &request.title, request.tags, image).await?;

    Ok(())
}

pub async fn get_all_tags(
    State(image_service): State<ImageService>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(image_service.get_all_tags().await?))
}

#[derive(TryFromMultipart, Debug)]
pub struct ImageUploadRequest {
    title: String,
    tags: Vec<String>,
    #[form_data(limit = "5MB")]
    image: FieldData<NamedTempFile>,
}