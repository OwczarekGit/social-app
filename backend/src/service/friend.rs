use std::sync::Arc;
use axum::http::StatusCode;
use axum_macros::FromRef;
use neo4rs::{query, Graph, Node};
use serde::{Deserialize, Serialize};

#[derive(Clone, FromRef)]
pub struct FriendService {
    neo4j: Arc<Graph>,
}

impl FriendService {
    pub async fn get_pending_friend_requests(&self, user_id: i64) -> Result<Vec<FriendRequest>, StatusCode> {
        let query = query("match (p:Profile)-[:REQUESTED_FRIENDSHIP]->(:Profile{id: $id}) return p")
            .param("id", user_id);

        let mut data = self.neo4j.execute(query).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut requests = vec![];
        while let Ok(Some(row)) = data.next().await {
            let n: Node = row.get("p").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            let id: i64 = n.get("id").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
            let username: String = n.get("username").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            requests.push(FriendRequest{
                user_id: id,
                username
            });
        }

        Ok(requests)
    }

    pub async fn accept_friend_request(&self, user_id: i64, requester_id: i64) -> Result<(), StatusCode> {
        let query = query("MATCH (p1:Profile {id: $requester_id})-[r:REQUESTED_FRIENDSHIP]->(p2:Profile {id: $user_id}) delete r merge (p1)-[:FRIEND]->(p2)")
            .param("requester_id", requester_id)
            .param("user_id", user_id);

        self.neo4j.run(query).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
}

impl FriendService {
    pub fn new(neo4j: Arc<Graph>) -> Self {
        Self {neo4j}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FriendRequest {
    user_id: i64,
    username: String,
}