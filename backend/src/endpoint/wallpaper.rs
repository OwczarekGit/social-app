use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::routing::{delete, get, post};
use crate::{AppState};
use crate::service::wallpaper::WallpaperService;

use crate::{Result};
use crate::app_state::ActiveUser;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_wallpapers))
        .route("/:id", post(set_wallpaper))
        .route("/current", get(get_current_wallpaper))
        .route("/", delete(unset_wallpaper))
}

pub async fn get_all_wallpapers(
    State(image_service): State<WallpaperService>,
) -> crate::Result<impl IntoResponse> {
    Ok(Json(image_service.get_all_wallpapers().await?))
}

pub async fn set_wallpaper(
    Extension(user): Extension<ActiveUser>,
    State(wallpaper_service): State<WallpaperService>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    Ok(Json(wallpaper_service.set_wallpaper(user.id, id).await?))
}

pub async fn unset_wallpaper(
    Extension(user): Extension<ActiveUser>,
    State(wallpaper_service): State<WallpaperService>,
) -> Result<impl IntoResponse> {
    Ok(wallpaper_service.unset_wallpaper(user.id).await?)
}

pub async fn get_current_wallpaper(
    Extension(user): Extension<ActiveUser>,
    State(wallpaper_service): State<WallpaperService>,
) -> Result<impl IntoResponse> {
    Ok(Json(wallpaper_service.get_current_wallpaper(user.id).await?))
}
