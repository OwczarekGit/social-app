use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Subcommand)]
pub enum ExecuteActionOnStart {
    CreateAdminAccount(CreateAdminAccount),
}

#[derive(Serialize, Deserialize, Debug, Clone, Args)]
pub struct CreateAdminAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    #[command(flatten)]
    pub neo4j_config: Neo4jConfig,

    #[command(flatten)]
    pub s3_config: S3Config,

    #[command(flatten)]
    pub valkey_config: ValkeyConfig,

    #[command(flatten)]
    pub postgres_config: PostgresConfig,

    #[command(subcommand)]
    pub create_admin_args: Option<ExecuteActionOnStart>,
}

impl Arguments {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Clone, Args)]
pub struct Neo4jConfig {
    #[arg(env("NEO4J_URL"))]
    pub neo4j_url: String,
    #[arg(env("NEO4J_USER"))]
    pub neo4j_user: String,
    #[arg(env("NEO4J_PASSWORD"))]
    pub neo4j_password: String,
    #[arg(env("NEO4J_DB"))]
    pub neo4j_db: String,
}

#[derive(Debug, Clone, Args)]
pub struct S3Config {
    #[arg(env("S3_URL"))]
    pub s3_url: String,
    #[arg(env("S3_USER"))]
    pub s3_user: String,
    #[arg(env("S3_PASSWORD"))]
    pub s3_password: String,
}

#[derive(Debug, Clone, Args)]
pub struct PostgresConfig {
    #[arg(env("DATABASE_URL"))]
    pub database_url: String,
}

#[derive(Debug, Clone, Args)]
pub struct ValkeyConfig {
    #[arg(env("VALKEY_URL"))]
    pub valkey_url: String,
}
