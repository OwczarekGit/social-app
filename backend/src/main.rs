use std::env;

use axum::{Router, response::IntoResponse, middleware::{self, Next}, http::{Request, StatusCode}, extract::State};
use axum_macros::FromRef;
use endpoint::account;
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use service::{account::AccountService, email::EmailService};
use tower_cookies::{CookieManagerLayer, Cookies};
use serde::{Serialize, Deserialize};
use tower_http::cors::{Any, CorsLayer};

mod entities;
mod endpoint;
mod service;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let redis_connection = redis_connection().await.expect("To connect.");
    let postgres_connection = postgres_connection().await.expect("To connect.");

    let state = AppState::new(redis_connection, postgres_connection);

    let app = Router::<AppState>::new()
        .nest(
            "/api", Router::<AppState>::new()
                .nest("/post", endpoint::post::routes())
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
    let cookie = cookies.get(endpoint::account::SESSION_COOKIE_NAME)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id = acs.verify_session(cookie.value()).await?;

    let mut response = request;
    response.extensions_mut().insert(ActiveUserId(user_id));

    Ok(next.run(response).await)
}

async fn redis_connection() -> Result<ConnectionManager, ()> {
    let redis_connection_string = env::var("REDIS_URL").expect("REDIS_URL to be set.");
    let client = redis::Client::open(redis_connection_string).map_err(|_| ())?;
    let manager = redis::aio::ConnectionManager:: new(client).await.map_err(|_| ())?;
    Ok(manager)
}

async fn postgres_connection() -> Result<DatabaseConnection, ()> {
    let postgres_connection_string = env::var("DATABASE_URL").expect("DATABASE_URL to be set.");
    let db = Database::connect(postgres_connection_string).await.map_err(|_|())?;
    Ok(db)
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub account_service: AccountService,
    pub email_service: EmailService,
}

impl AppState {
    pub fn new(redis_connection: redis::aio::ConnectionManager, postgres_connection: DatabaseConnection) -> Self {
        Self {
            account_service: AccountService::new(redis_connection, postgres_connection.clone()),
            email_service: EmailService::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveUserId(pub i64);