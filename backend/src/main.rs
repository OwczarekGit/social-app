use std::sync::Arc;

pub use self::error::{Error, SysRes};
use crate::app_state::AppState;
use app::App;
use arguments::Arguments;
use clap::Parser;
use config::get_arg;
use tracing::warn;
use tracing_subscriber::EnvFilter;

mod active_user;
mod app;
mod app_state;
mod arguments;
mod config;
mod endpoint;
mod error;
mod image_domain;
mod service;

#[tokio::main]
async fn main() -> SysRes<()> {
    let args = init().await;

    let redis_connection = config::redis_connection().await?;
    let postgres_connection = config::postgres_connection().await?;
    let neo4j_connection = Arc::new(config::neo4j_connection().await?);
    let minio_connection = config::minio_connection().await?;

    let state = AppState::new(
        redis_connection,
        postgres_connection,
        neo4j_connection.clone(),
        minio_connection,
    )
    .await;

    if let Some(action) = args.create_admin_args {
        match action {
            arguments::ExecuteActionOnStart::CreateAdminAccount(admin) => {
                if state
                    .account_service
                    .create_admin_account(&admin.username, &admin.email, &admin.password)
                    .await
                    .is_ok()
                {
                    warn!(
                        "Admin account: {} has been created. Shutting down.",
                        &admin.email
                    );
                    return Ok(());
                }
            }
        }
    }

    App::new(get_arg::<u16>("PORT")?.parse()?)
        .await?
        .run(endpoint::routes(state))
        .await
}

async fn init() -> Arguments {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    arguments::Arguments::parse()
}
