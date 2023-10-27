use std::sync::Arc;

use axum::{middleware, Router};
use axum::extract::DefaultBodyLimit;
use axum::response::Response;
use clap::Parser;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeFile;
use tracing::{debug, warn};
use tracing_subscriber::EnvFilter;
use crate::app_state::AppState;
pub use self::error::{Error, Result};

mod entities;
mod error;
mod endpoint;
mod service;
mod config;
mod arguments;
mod authorization_filter;
mod app_state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = arguments::Arguments::parse();

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let static_files = tower_http::services::ServeDir::new("./static")
        .fallback(ServeFile::new("./static/index.html"));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let redis_connection = config::redis_connection().await.expect("To connect pg.");
    let postgres_connection = config::postgres_connection().await.expect("To connect redis.");
    let neo4j_connection = Arc::new(config::neo4j_connection().await.expect("To connect n4j."));
    let minio_connection = config::minio_connection().await.expect("To connect minio.");

    let state = AppState::new(
        redis_connection,
        postgres_connection,
        neo4j_connection.clone(),
        minio_connection
    ).await;

    if let Some(action) = args.create_admin_args {
        match action {
            arguments::ExecuteActionOnStart::CreateAdminAccount(admin) => {
                if state.account_service.create_admin_account(&admin.email, &admin.password).await.is_ok() {
                    warn!("Admin account: {} has been created. Shutting down.", &admin.email);
                    return;
                }
            }
        }
    }

    let app = Router::<AppState>::new()
        .nest(
            "/api", Router::<AppState>::new()
                .nest("/admin/activation", endpoint::activation::admin_routes())
                .nest("/admin/domain", endpoint::domain::admin_routes())
                .nest("/admin/tag", endpoint::tag::admin_routes())
                // All routes above can only be accessed by admin.
                .layer(middleware::from_fn_with_state(state.clone(), authorization_filter::authorize_admin))
                .nest("/tag", endpoint::tag::public_routes())
                .nest("/post", endpoint::post::routes())
                .nest("/notification", endpoint::notification::routes())
                .nest("/friend", endpoint::friend::routes())
                .nest("/profile", endpoint::profile::routes())
                .nest("/image", endpoint::image::routes())
                .nest("/chat", endpoint::chat::routes())
                .nest("/wallpaper", endpoint::wallpaper::routes())
                .nest("/account", endpoint::account::logged_in_routes())
                .nest("/domain", endpoint::domain::routes())
                // All routes that require authentication go above this route_layer.
                .layer(middleware::from_fn_with_state(state.account_service.clone(), authorization_filter::authorize_by_cookie))
                .layer(middleware::from_fn_with_state(state.domain_service.clone(), service::domain::extract_image_domain))
                .nest("/account", endpoint::account::routes())
        )
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024*1024*1024))
        .with_state(state)
        .fallback_service(static_files);

    debug!("Starting server");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn main_response_mapper(res: Response) -> Response {
    if let Some(err) = res.extensions().get::<Error>() {
        //TODO: Some of these are probably worth saving.
        debug!("Error: {:?}", err);
    }
    res
}