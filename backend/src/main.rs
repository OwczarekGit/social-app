use std::env;
use std::sync::Arc;

use axum::{Router, response::IntoResponse, middleware::{self, Next}, http::{Request, StatusCode}, extract::State};
use axum_macros::FromRef;
use neo4rs::{ConfigBuilder, Graph};
use endpoint::account;
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use service::{account::AccountService, email::EmailService};
use tower_cookies::{CookieManagerLayer, Cookies};
use serde::{Serialize, Deserialize};
use tower_http::cors::{Any, CorsLayer};
use crate::service::friend::FriendService;
use crate::service::notification::NotificationService;
use crate::service::post::PostService;

mod entities;
mod endpoint;
mod service;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let redis_connection = redis_connection().await.expect("To connect pg.");
    let postgres_connection = postgres_connection().await.expect("To connect redis.");
    let neo4j_connection = Arc::new(neo4j_connection().await.expect("To connect n4j."));

    let state = AppState::new(redis_connection, postgres_connection, neo4j_connection.clone());

    let app = Router::<AppState>::new()
        .nest(
            "/api", Router::<AppState>::new()
                .nest("/post", endpoint::post::routes())
                .nest("/notification", endpoint::notification::routes())
                .nest("/friend", endpoint::friend::routes())
                // All routes that require authentication go above this route_layer.
                .layer(middleware::from_fn_with_state(state.account_service.clone(), authorize_by_cookie))
                .nest("/account", account::routes())
        )
        .layer(CookieManagerLayer::new())
        .layer(cors)
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

    let user_id = acs.verify_session(cookie.value()).await?;

    let mut response = request;
    response.extensions_mut().insert(ActiveUserId(user_id));

    Ok(next.run(response).await)
}

async fn redis_connection() -> Result<ConnectionManager, ()> {
    let redis_connection_string = env::var("REDIS_URL").expect("REDIS_URL to be set.");
    let client = redis::Client::open(redis_connection_string).map_err(|_| ())?;
    let manager = ConnectionManager:: new(client).await.map_err(|_| ())?;
    Ok(manager)
}

async fn postgres_connection() -> Result<DatabaseConnection, ()> {
    let postgres_connection_string = env::var("DATABASE_URL").expect("DATABASE_URL to be set.");
    let db = Database::connect(postgres_connection_string).await.map_err(|_|())?;
    Ok(db)
}

async fn neo4j_connection() -> Result<Graph, ()> {
    let neo4j_connection_uri = env::var("NEO4J_URI").expect("NEO4J_URI to be set.");
    let neo4j_connection_user = env::var("NEO4J_USER").expect("NEO4J_USER to be set.");
    let neo4j_connection_password = env::var("NEO4J_PASS").expect("NEO4J_PASS to be set.");
    let neo4j_connection_db = env::var("NEO4J_DB").expect("NEO4J_DB to be set.");
    let graph = ConfigBuilder::new()
        .uri(neo4j_connection_uri)
        .user(neo4j_connection_user)
        .password(neo4j_connection_password)
        .db(neo4j_connection_db)
        .build()
        .expect("To create config.");

    Graph::connect(graph)
        .await
        .map_err(|_| ())
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub account_service: AccountService,
    pub email_service: EmailService,
    pub post_service: PostService,
    pub notification_service: NotificationService,
    pub friend_service: FriendService,
}

impl AppState {
    pub fn new(
        redis_connection:ConnectionManager,
        postgres_connection: DatabaseConnection,
        neo4j_connection: Arc<Graph>
    ) -> Self {
        Self {
            account_service: AccountService::new(redis_connection, postgres_connection.clone(), neo4j_connection.clone()),
            email_service: EmailService::new(),
            post_service: PostService::new(neo4j_connection.clone()),
            notification_service: NotificationService::new(neo4j_connection.clone()),
            friend_service: FriendService::new(neo4j_connection.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveUserId(pub i64);