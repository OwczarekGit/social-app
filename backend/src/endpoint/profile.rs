use crate::active_user::ActiveUser;
use crate::image_domain::ImageDomain;
use crate::service::profile::{Profile, ProfileService};
use crate::AppState;
use crate::SysRes;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, put};
use axum::{Json, Router};
use axum_typed_multipart::TypedMultipart;
use dto::profile::*;
use image::ImageReader;
use std::io::{Cursor, Read};

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
) -> SysRes<impl IntoResponse> {
    Ok(profile_service
        .change_username(user.id, &request.username)
        .await)
}

pub async fn get_my_profile(
    image_domain: ImageDomain,
    user: ActiveUser,
    State(profile_service): State<ProfileService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(profile_service.get_profile(user.id).await.map(
        |p| Profile {
            picture_url: format!("{}{}", image_domain.0, p.picture_url),
            ..p
        },
    )?))
}

pub async fn get_profile(
    image_domain: ImageDomain,
    State(profile_service): State<ProfileService>,
    Path(id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(profile_service.get_profile(id).await.map(|p| {
        Profile {
            picture_url: format!("{}{}", image_domain.0, p.picture_url),
            ..p
        }
    })?))
}

pub async fn set_profile_picture(
    user: ActiveUser,
    State(profile_service): State<ProfileService>,
    TypedMultipart(request): TypedMultipart<ChangeProfilePictureRequest>,
) -> SysRes<impl IntoResponse> {
    let mut image_bytes = vec![];
    request
        .image
        .contents
        .as_file()
        .read_to_end(&mut image_bytes)?;

    let image = ImageReader::new(Cursor::new(&mut image_bytes))
        .with_guessed_format()?
        .decode()?;

    Ok(profile_service.change_profile_picture(user.id, image).await)
}
