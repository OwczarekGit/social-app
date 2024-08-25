use std::sync::Arc;

use axum::{extract::DefaultBodyLimit, middleware, response::Response, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use tracing::debug;

use crate::{app_state::AppState, Error};

pub mod account;
pub mod activation;
pub mod chat;
pub mod domain;
pub mod friend;
pub mod image;
pub mod notification;
pub mod post;
pub mod profile;
pub mod tag;
pub mod wallpaper;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/account", account::routes())
                .nest("/account", account::logged_in_routes())
                .nest("/admin/activation", activation::admin_routes())
                .nest("/chat", chat::routes())
                .nest("/admin/domain", domain::admin_routes())
                .nest("/friend", friend::routes())
                .nest("/image", image::routes())
                .nest("/notification", notification::routes())
                .nest("/post", post::routes())
                .nest("/profile", profile::routes())
                .nest("/tag", tag::public_routes())
                .nest("/admin/tag", tag::admin_routes())
                .nest("/wallpaper", wallpaper::routes()),
        )
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(cors())
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .fallback_service(static_files())
        .with_state(state)
}

fn static_files() -> ServeDir<ServeFile> {
    tower_http::services::ServeDir::new("./static").fallback(ServeFile::new("./static/index.html"))
}

fn cors() -> CorsLayer {
    CorsLayer::new().allow_origin(Any).allow_methods(Any)
}

pub async fn main_response_mapper(res: Response) -> Response {
    if let Some(err) = res.extensions().get::<Arc<Error>>() {
        //TODO: Some of these are probably worth saving.
        debug!("Error: {:?}", err);
    }
    res
}
