use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;
use neo4rs::{ConfigBuilder, Graph};
use redis::aio::ConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use std::{any::type_name, env};

pub async fn redis_connection() -> Result<ConnectionManager, ()> {
    let redis_connection_string = get_arg::<String>("REDIS_URL");
    let client = redis::Client::open(redis_connection_string).map_err(|_| ())?;
    let manager = ConnectionManager::new(client).await.map_err(|_| ())?;
    Ok(manager)
}

pub async fn postgres_connection() -> Result<DatabaseConnection, ()> {
    let postgres_connection_string = get_arg::<String>("DATABASE_URL");
    let db = Database::connect(postgres_connection_string)
        .await
        .map_err(|_| ())?;
    Ok(db)
}

pub async fn neo4j_connection() -> Result<Graph, ()> {
    let neo4j_connection_uri = get_arg::<String>("NEO4J_URI");
    let neo4j_connection_user = get_arg::<String>("NEO4J_USER");
    let neo4j_connection_password = get_arg::<String>("NEO4J_PASS");
    let neo4j_connection_db = get_arg::<String>("NEO4J_DB");
    let graph = ConfigBuilder::new()
        .uri(neo4j_connection_uri)
        .user(neo4j_connection_user)
        .password(neo4j_connection_password)
        .db(neo4j_connection_db)
        .build()
        .expect("To create config.");

    Graph::connect(graph).await.map_err(|_| ())
}

pub async fn minio_connection() -> Result<Minio, ()> {
    let minio_user = get_arg::<String>("MINIO_ROOT_USER");
    let minio_password = get_arg::<String>("MINIO_ROOT_PASSWORD");
    let minio_endpoint = get_arg::<String>("MINIO_ENDPOINT");
    let provider = StaticProvider::new(minio_user, minio_password, None);
    let minio = Minio::builder()
        .endpoint(minio_endpoint)
        .provider(provider)
        .secure(false)
        .build()
        .unwrap();

    Ok(minio)
}

pub fn get_arg<T>(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        panic!(
            "Variable \"{name}\" <{}> missing from environment.",
            type_name::<T>()
        )
    })
}
