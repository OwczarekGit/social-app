use clap::Args;
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
