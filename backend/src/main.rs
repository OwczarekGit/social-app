use std::env;

use axum::Router;
use axum_macros::FromRef;
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use service::{account::AccountService, email::EmailService};

mod entities;
mod endpoint;
mod service;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let redis_connection = redis_connection().await.expect("To connect.");
    let postgres_connection = postgres_connection().await.expect("To connect.");

    let state = AppState::new(redis_connection, postgres_connection);


    let app = Router::<AppState>::new()
        .nest(
            "/api", Router::<AppState>::new()
                .nest("/account", endpoint::account::routes())
        )
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
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