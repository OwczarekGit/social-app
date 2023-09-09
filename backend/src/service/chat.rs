use std::sync::Arc;
use neo4rs::{Graph, query};

use crate::{Result, Error};

#[derive(Clone)]
pub struct ChatService {
    graph: Arc<Graph>
}

impl ChatService {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph }
    }
}

impl ChatService {
    pub async fn send_message_to_friend(&self, user_id: i64, friend_id: i64, content: &str) -> Result<()> {
        let q = query(r#"
            match (u:Profile{id: $id})-[:FRIEND]-(o:Profile{id: $other})
            create (u)-[m:MESSAGE{content: $content, date: $date}]->(o)
            return m"#
        )
            .param("id", user_id)
            .param("other", friend_id)
            .param("content", content)
            .param("date", chrono::Utc::now().naive_local());

        let result = self.graph.execute(q)
            .await?
            .next()
            .await?
            .ok_or(Error::InvalidSendMessageRequest(user_id, friend_id));

        dbg!(result);

        Ok(())
    }
}