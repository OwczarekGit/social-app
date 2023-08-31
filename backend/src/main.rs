use std::sync::Arc;

use axum::{Router, response::IntoResponse, middleware::{self, Next}, http::{Request}, extract::State, Extension};
use axum::extract::DefaultBodyLimit;
use axum::response::Response;
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
use tower_http::services::ServeFile;
use tracing::debug;
use tracing_subscriber::EnvFilter;
use crate::entities::sea_orm_active_enums::AccountType;
use crate::service::friend::FriendService;
use crate::service::image::ImageService;
use crate::service::notification::NotificationService;
use crate::service::post::PostService;
use crate::service::profile::ProfileService;
use crate::service::tag::TagService;

pub use self::error::{Error, Result};

mod entities;
mod error;
mod endpoint;
mod service;
mod config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

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

    let state = AppState::new(redis_connection, postgres_connection, neo4j_connection.clone(), minio_connection);

    let app = Router::<AppState>::new()
        .nest(
            "/api", Router::<AppState>::new()
                .nest("/admin/tag", endpoint::tag::admin_routes())
                // All routes above can only be accessed by admin / moderators.
                .layer(middleware::from_fn_with_state(state.clone(), authorize_moderator_or_admin))
                .nest("/tag", endpoint::tag::public_routes())
                .nest("/post", endpoint::post::routes())
                .nest("/notification", endpoint::notification::routes())
                .nest("/friend", endpoint::friend::routes())
                .nest("/profile", endpoint::profile::routes())
                .nest("/image", endpoint::image::routes())
                // All routes that require authentication go above this route_layer.
                .layer(middleware::from_fn_with_state(state.account_service.clone(), authorize_by_cookie))
                .nest("/account", account::routes())
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

async fn authorize_moderator_or_admin<B>(
    Extension(user): Extension<ActiveUser>,
    request: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse> {
    if ActiveUserRole::from(user.role.clone()) != ActiveUserRole::Admin {
        return Err(Error::UnauthorizedForAdminOperations(user.id));
    }

    debug!("User id: ({}), role: ({}) accessed the admin endpoints.", user.id, user.role.to_string());

    Ok(next.run(request).await)
}

async fn authorize_by_cookie<B>(
    State(mut acs): State<AccountService>,
    cookies: Cookies,
    request: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse> {
    let cookie = cookies.get(account::SESSION_COOKIE_NAME).ok_or(Error::UnauthorizedForUserOperations)?;

    let user = acs.verify_session(cookie.value()).await?;

    let mut response = request;
    response.extensions_mut().insert(user);

    Ok(next.run(response).await)
}

pub async fn main_response_mapper(res: Response) -> Response {
    if let Some(err) = res.extensions().get::<Error>() {
        //TODO: Some of these are probably worth saving.
        debug!("Error: {:?}", err);
    }
    res
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub account_service: AccountService,
    pub email_service: EmailService,
    pub post_service: PostService,
    pub notification_service: NotificationService,
    pub friend_service: FriendService,
    pub profile_service: ProfileService,
    pub image_service: ImageService,
    pub tag_service: TagService,
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
            image_service: ImageService::new(neo4j_connection.clone(), minio_connection.clone()),
            tag_service: TagService::new(neo4j_connection.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveUser {
    pub id: i64,
    pub role: ActiveUserRole
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ActiveUserRole {
    User,
    Admin
}

impl From<entities::account::Model> for ActiveUser {
    fn from(value: entities::account::Model) -> Self {
        Self {
            id: value.id,
            role: match value.r#type {
                AccountType::Admin => ActiveUserRole::Admin,
                AccountType::User => ActiveUserRole::User
            }
        }
    }
}

impl ToString for ActiveUserRole {
    fn to_string(&self) -> String {
        match self {
            ActiveUserRole::User => "User".to_string(),
            ActiveUserRole::Admin => "Admin".to_string()
        }
    }
}