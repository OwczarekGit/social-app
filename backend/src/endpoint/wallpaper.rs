use crate::service::wallpaper::{Image, WallpaperService};
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};

use crate::app_state::ActiveUser;
use crate::service::domain::ImageDomain;
use crate::SysRes;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_wallpapers))
        .route("/:id", post(set_wallpaper))
        .route("/current", get(get_current_wallpaper))
        .route("/", delete(unset_wallpaper))
}

pub async fn get_all_wallpapers(
    image_domain: ImageDomain,
    State(image_service): State<WallpaperService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        image_service
            .get_all_wallpapers()
            .await?
            .into_iter()
            .map(|i| Image {
                url: format!("{}{}", image_domain.0, i.url),
                ..i
            })
            .collect::<Vec<_>>(),
    ))
}

pub async fn set_wallpaper(
    user: ActiveUser,
    State(wallpaper_service): State<WallpaperService>,
    Path(id): Path<i64>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(wallpaper_service.set_wallpaper(user.id, id).await?))
}

pub async fn unset_wallpaper(
    user: ActiveUser,
    State(wallpaper_service): State<WallpaperService>,
) -> SysRes<impl IntoResponse> {
    wallpaper_service.unset_wallpaper(user.id).await
}

pub async fn get_current_wallpaper(
    image_domain: ImageDomain,
    user: ActiveUser,
    State(wallpaper_service): State<WallpaperService>,
) -> SysRes<impl IntoResponse> {
    Ok(Json(
        wallpaper_service
            .get_current_wallpaper(user.id)
            .await?
            .map(|i| Image {
                id: i.id,
                title: i.title,
                url: format!("{}{}", image_domain.0, i.url),
            }),
    ))
}
