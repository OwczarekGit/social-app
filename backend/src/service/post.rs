use std::sync::Arc;
use axum::http::StatusCode;
use axum_macros::FromRef;
use neo4rs::{Graph, query};

#[derive(Clone, FromRef)]
pub struct PostService {
    neo4j: Arc<Graph>
}

impl PostService {
    pub fn new(neo4j: Arc<Graph>) -> Self {
        Self {
            neo4j
        }
    }

    pub async fn create_post(&self, author_id: i64, content: &str) -> Result<(), StatusCode> {
        let query = query("match (u:Profile{id:$id}) create (u)-[w:Posted{at: $time}]->(p:Post{content: $content}) return p,w,u")
            .param("id", author_id)
            .param("content", content)
            .param("time", chrono::Utc::now().naive_local());

        self.neo4j.run(query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(())
    }
}