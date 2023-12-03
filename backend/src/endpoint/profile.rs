use std::io::{Cursor, Read};
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::{get, put};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use image::io::Reader;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use crate::AppState;
use crate::app_state::ActiveUser;
use crate::service::domain::ImageDomain;
use crate::service::profile::{Profile, ProfileService};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_profile))
        .route("/:id", get(get_profile))
        .route("/username", put(change_username))
        .route("/picture", put(set_profile_picture))
}

pub async fn change_username(
    user: ActiveUser,
    State(profile_service): State<ProfileService>,
    Json(request): Json<ChangeUsernameRequest>,
) -> crate::Result<impl IntoResponse> {
    Ok(profile_service.change_username(user.id, &request.username).await)
}

pub async fn get_my_profile(
    image_domain: ImageDomain,
    user: ActiveUser,
    State(profile_service): State<ProfileService>,
) -> crate::Result<impl IntoResponse> {
    Ok(
        Json(
            profile_service.get_profile(user.id)
                .await
                .map(|p| Profile {
                    picture_url: format!("{}{}", image_domain.0, p.picture_url),
                    ..p
                })?
        )
    )
}

pub async fn get_profile(
    image_domain: ImageDomain,
    State(profile_service): State<ProfileService>,
    Path(id): Path<i64>
) -> crate::Result<impl IntoResponse> {
    Ok(
        Json(
            profile_service.get_profile(id)
                .await
                .map(|p| Profile {
                    picture_url: format!("{}{}", image_domain.0, p.picture_url),
                    ..p
                })?
        )
    )
}

pub async fn set_profile_picture(
    user: ActiveUser,
    State(profile_service): State<ProfileService>,
    TypedMultipart(request): TypedMultipart<ChangeProfilePictureRequest>
) -> crate::Result<impl IntoResponse> {
    let mut image_bytes = vec![];
    request.image.contents
        .as_file()
        .read_to_end(&mut image_bytes)?;

    let image = Reader::new(Cursor::new(&mut image_bytes))
        .with_guessed_format()?
        .decode()?;

    Ok(profile_service.change_profile_picture(user.id, image).await)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeUsernameRequest {
    pub username: String,
}

#[derive(TryFromMultipart, Debug)]
pub struct ChangeProfilePictureRequest {
    #[form_data(limit = "5MB")]
    image: FieldData<NamedTempFile>,
}
