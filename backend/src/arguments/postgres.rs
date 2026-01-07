use clap::Args;
#[derive(Debug, Clone, Args)]
pub struct PostgresConfig {
    #[arg(env("DATABASE_URL"))]
    pub database_url: String,
}
