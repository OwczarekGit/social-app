use std::sync::Arc;
use chrono::NaiveDateTime;
use neo4rs::{Graph, Node, query, Relation, Row};
use serde::{Deserialize, Serialize};

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
    pub async fn send_message_to_friend(&self, user_id: i64, friend_id: i64, content: &str) -> Result<FriendMessage> {
        let q = query(r#"
            match (p:Profile{id: $id})-[:FRIEND]-(f:Profile{id: $other})
            create (p)-[m:MESSAGE{message: $content, date: $date}]->(f)
            return p,f,m"#
        )
            .param("id", user_id)
            .param("other", friend_id)
            .param("content", content)
            .param("date", chrono::Utc::now().naive_local());

        let result = self.graph.execute(q)
            .await?
            .next()
            .await?
            .ok_or(Error::InvalidSendMessageToFriendRequest(user_id, friend_id))?;

       FriendMessage::try_from(result)
    }

    pub async fn get_friend_conversation_messages(&self, user_id: i64, friend_id: i64) -> Result<Vec<FriendMessage>>{
        let q = query(r#"
            match (p:Profile{id: $id})-[m:MESSAGE]-(f:Profile{id: $friend_id})
            return p,f,m order by m.date desc"#
        )
            .param("id", user_id)
            .param("friend_id", friend_id);

        let mut result = self.graph.execute(q)
            .await?;

        let mut messages = vec![];
        while let Ok(Some(value)) = result.next().await {
            messages.push(FriendMessage::try_from(value)?);
        }

        Ok(messages)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendMessage {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub date: NaiveDateTime,
    pub message: String,
}

impl TryFrom<Row> for FriendMessage {
    type Error = Error;

    fn try_from(value: Row) -> std::result::Result<Self, Self::Error> {
        let user1: Node = value.get("p").ok_or(Error::FriendMessageMissingUserNode)?;
        let user2: Node = value.get("f").ok_or(Error::FriendMessageMissingUserNode)?;
        let message: Relation = value.get("m").ok_or(Error::FriendMessageMissingMessageBody)?;

        let author = if user1.id() == message.start_node_id() {
            &user1
        } else {
            &user2
        };

        Ok(Self {
            id: message.id(),
            user_id: author.get("id").ok_or(Error::Neo4jInvalidNode(author.id()))?,
            username: author.get("username").ok_or(Error::Neo4jInvalidNode(author.id()))?,
            date: message.get("date").ok_or(Error::Neo4jInvalidNode(message.id()))?,
            message: message.get("message").ok_or(Error::Neo4jInvalidNode(message.id()))?
        })
    }
}