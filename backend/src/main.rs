use std::sync::Arc;

use axum::{Router, response::IntoResponse, middleware::{self, Next}, http::{Request, StatusCode}, extract::State};
use axum::extract::DefaultBodyLimit;
use axum_macros::FromRef;
use minio_rsc::Minio;
use neo4rs::{Graph};
use endpoint::account;
use redis::aio::ConnectionManager;
use sea_orm::{DatabaseConnection};
use service::{account::AccountService, email::EmailService};
use tower_cookies::{CookieManagerLayer, Cookies};
use serde::{Serialize, Deserialize};
use tower_http::cors::{Any, CorsLayer};
use crate::entities::account::Model;
use crate::entities::sea_orm_active_enums::AccountType;
use crate::service::friend::FriendService;
use crate::service::image::ImageService;
use crate::service::notification::NotificationService;
use crate::service::post::PostService;
use crate::service::profile::ProfileService;

mod entities;
mod endpoint;
mod service;
mod config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let redis_connection = config::redis_connection().await.expect("To connect pg.");
    let postgres_connection = config::postgres_connection().await.expect("To connect redis.");
    let neo4j_connection = Arc::new(config::neo4j_connection().await.expect("To connect n4j."));
    let minio_connection = config::minio_connection().await.expect("To connect minio.");

    let state = AppState::new(redis_connection, postgres_connection, neo4j_connection.clone(), minio_connection);

    let app = Router::<AppState>::new()
        .nest(
            "/api", Router::<AppState>::new()
                .nest("/post", endpoint::post::routes())
                .nest("/notification", endpoint::notification::routes())
                .nest("/friend", endpoint::friend::routes())
                .nest("/profile", endpoint::profile::routes())
                .nest("/image", endpoint::image::routes())
                // All routes that require authentication go above this route_layer.
                .layer(middleware::from_fn_with_state(state.account_service.clone(), authorize_by_cookie))
                .nest("/account", account::routes())
        )
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024*1024*1024))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn authorize_by_cookie<B>(
    State(mut acs): State<AccountService>,
    cookies: Cookies,
    request: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse, StatusCode> {
    let cookie = cookies.get(account::SESSION_COOKIE_NAME)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = acs.verify_session(cookie.value()).await?;

    let mut response = request;
    response.extensions_mut().insert(user);

    Ok(next.run(response).await)
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub account_service: AccountService,
    pub email_service: EmailService,
    pub post_service: PostService,
    pub notification_service: NotificationService,
    pub friend_service: FriendService,
    pub profile_service: ProfileService,
    pub image_service: ImageService
}

impl AppState {
    pub fn new(
        redis_connection:ConnectionManager,
        postgres_connection: DatabaseConnection,
        neo4j_connection: Arc<Graph>,
        minio_connection: Minio,
    ) -> Self {
        Self {
            account_service: AccountService::new(redis_connection, postgres_connection.clone(), neo4j_connection.clone()),
            email_service: EmailService::new(),
            post_service: PostService::new(neo4j_connection.clone()),
            notification_service: NotificationService::new(neo4j_connection.clone(), postgres_connection.clone()),
            friend_service: FriendService::new(neo4j_connection.clone()),
            profile_service: ProfileService::new(neo4j_connection.clone()),
            image_service: ImageService::new(neo4j_connection.clone(), minio_connection.clone())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveUser {
    pub id: i64,
    pub role: ActiveUserRole
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActiveUserRole {
    User,
    Admin
}

impl From<crate::entities::account::Model> for ActiveUser {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            role: match value.r#type {
                AccountType::Admin => ActiveUserRole::Admin,
                AccountType::User => ActiveUserRole::User
            }
        }
    }
}