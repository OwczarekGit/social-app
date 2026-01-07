use clap::Parser;

mod neo4j;
mod postgres;
mod s3;
mod start_subcommand;
mod valkey;

pub use {
    neo4j::Neo4jConfig, postgres::PostgresConfig, s3::S3Config, start_subcommand::*,
    valkey::ValkeyConfig,
};

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
    pub create_admin_args: Option<StartSubcommand>,
}

impl Arguments {
    pub fn get() -> Self {
        Self::parse()
    }
}
