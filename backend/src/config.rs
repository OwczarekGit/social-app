use minio_rsc::Minio;
use minio_rsc::provider::StaticProvider;
use neo4rs::{ConfigBuilder, Graph};
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};

use crate::{
    SysRes,
    arguments::{Neo4jConfig, PostgresConfig, S3Config, ValkeyConfig},
};

pub async fn valkey_connection(config: ValkeyConfig) -> SysRes<ConnectionManager> {
    let client = redis::Client::open(config.valkey_url)?;
    let manager = ConnectionManager::new(client).await?;
    Ok(manager)
}

pub async fn postgres_connection(config: PostgresConfig) -> SysRes<DatabaseConnection> {
    let db = Database::connect(config.database_url).await?;
    Ok(db)
}

pub async fn neo4j_connection(config: Neo4jConfig) -> SysRes<Graph> {
    let graph = ConfigBuilder::new()
        .uri(config.neo4j_url)
        .user(config.neo4j_user)
        .password(config.neo4j_password)
        .db(config.neo4j_db)
        .build()
        .expect("To create config.");

    Ok(Graph::connect(graph).await?)
}

pub async fn minio_connection(config: S3Config) -> SysRes<Minio> {
    let provider = StaticProvider::new(config.s3_user, config.s3_password, None);
    let minio = Minio::builder()
        .endpoint(config.s3_url)
        .provider(provider)
        .secure(false)
        .build()
        .unwrap();

    Ok(minio)
}
