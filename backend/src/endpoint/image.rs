use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use image::ImageReader;
use std::io::{Cursor, Read};
use tempfile::NamedTempFile;

use crate::app_state::ActiveUser;
use crate::service::image::ImageService;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(share_image).get(get_all_tags))
}

pub async fn share_image(
    user: ActiveUser,
    State(image_service): State<ImageService>,
    TypedMultipart(request): TypedMultipart<ImageUploadRequest>,
) -> crate::Result<impl IntoResponse> {
    let mut image_bytes = vec![];
    request
        .image
        .contents
        .as_file()
        .read_to_end(&mut image_bytes)?;

    let image = ImageReader::new(Cursor::new(&mut image_bytes))
        .with_guessed_format()?
        .decode()?;

    image_service
        .upload_image(user.id, &request.title, request.tags, image)
        .await?;

    Ok(())
}

pub async fn get_all_tags(
    State(image_service): State<ImageService>,
) -> crate::Result<impl IntoResponse> {
    Ok(Json(image_service.get_all_tags().await?))
}

#[derive(TryFromMultipart, Debug)]
pub struct ImageUploadRequest {
    title: String,
    tags: Vec<String>,
    #[form_data(limit = "5MB")]
    image: FieldData<NamedTempFile>,
}
