use axum::Router;
use axum_macros::FromRef;
use redis::aio::ConnectionManager;
use service::{account::AccountService, email::EmailService};

mod endpoint;
mod service;

#[tokio::main]
async fn main() {

    let redis_connection = redis_connection().await.expect("To connect.");

    let state = AppState::new(redis_connection);


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
    let client = redis::Client::open("redis://default:1234@127.0.0.1/").map_err(|_| ())?;
    let manager = redis::aio::ConnectionManager:: new(client).await.map_err(|_| ())?;
    Ok(manager)
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub account_service: AccountService,
    pub email_service: EmailService,
}

impl AppState {
    pub fn new(redis_connection: redis::aio::ConnectionManager) -> Self {
        Self {
            account_service: AccountService::new(redis_connection),
            email_service: EmailService::new(),
        }
    }
}