use std::sync::Arc;
use axum::http::StatusCode;
use axum_macros::FromRef;
use chrono::NaiveDateTime;
use neo4rs::{Graph, Node, query, Relation, Row};
use serde::{Deserialize, Serialize};

use crate::{Result, Error};

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

    pub async fn create_post(&self, author_id: i64, content: &str) -> Result<()> {
        let query = query("match (u:Profile{id:$id}) create (u)-[w:Posted{date: $time}]->(p:Post{content: $content}) return p,w,u")
            .param("id", author_id)
            .param("content", content)
            .param("time", chrono::Utc::now().naive_local());

        self.neo4j.run(query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(())
    }

    pub async fn get_posts_for_user(&self, user_id: i64) -> Result<Vec<Post>> {
        let q = query(r#"
            match (p:Post)<-[r:Posted]-(a:Profile{id: $id})
            return p,r,a
            order by r.date desc
        "#)
            .param("id", user_id);

        let mut res = self.neo4j.execute(q)
            .await?;

        let mut results = vec![];
        while let Ok(Some(row)) = res.next().await {
            if let Ok(post) = Post::try_from(row) {
                results.push(post);
            }
        }

        Ok(results)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i64,
    pub author_id: i64,
    pub author_username: String,
    pub author_picture_url: String,
    pub content: String,
    pub date: NaiveDateTime,
}

impl TryFrom<Row> for Post {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self> {
        let author: Node = value.get("a").ok_or(Error::Neo4jNodeNotFound)?;
        let post: Node = value.get("p").ok_or(Error::Neo4jNodeNotFound)?;
        let relation: Relation = value.get("r").ok_or(Error::Neo4jNodeNotFound)?;

        Ok(
            Self {
                id: post.id(),
                author_id: author.get("id").ok_or(Error::Neo4jInvalidNode(author.id()))?,
                author_username: author.get("username").unwrap_or("".to_string()),
                author_picture_url: author.get("picture_url").unwrap_or("".to_string()),

                content: post.get("content").ok_or(Error::Neo4jInvalidNode(post.id()))?,
                date: relation.get("date").ok_or(Error::Neo4jInvalidNode(relation.id()))?
            }
        )
    }
}