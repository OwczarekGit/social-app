use std::sync::Arc;
use axum::http::StatusCode;
use axum_macros::FromRef;
use neo4rs::{Graph, Node, query, Row};
use serde::{Deserialize, Serialize};

#[derive(Clone, FromRef)]
pub struct ProfileService {
    neo4j: Arc<Graph>
}


impl ProfileService {
    pub fn new(neo4j: Arc<Graph>) -> Self {
        Self {
            neo4j
        }
    }

    pub async fn change_username(&self, user_id: i64, username: &str) -> Result<(), StatusCode> {
        self.neo4j.run(
            query("match (p:Profile{id: $id}) set p.username=$username")
                .param("id", user_id)
                .param("username", username)
        )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub async fn get_profile(&self, user_id: i64) -> Result<Profile, StatusCode> {
        let profile =self.neo4j.execute(
            query("match (p:Profile{id: $id}) return p")
                .param("id", user_id)
        )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .next()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            .try_into()?;

        Ok(profile)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    id: i64,
    username: String,
}

impl TryFrom<Row> for Profile {
    type Error = StatusCode;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let p: Node = value.get("p").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Self {
            id: p.get("id").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
            username: p.get("username").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
        })
    }
}