use std::sync::Arc;
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

    // TODO: Mark the post as "edited".
    // NOTE: Right now the content is replaced, maybe it would be better to store previous versions of the post.
    pub async fn edit_post(&self, author_id: i64, post_id: i64, content: &str) -> Result<()> {
        let q = query(r#"
            match (u:Profile{id: $user_id})-[r:POSTED]->(p:Post)
            where id(p)=$post_id
            set p.content=$content
            return u,r,p
        "#)
            .param("user_id", author_id)
            .param("post_id", post_id)
            .param("content", content);

        Ok(self.neo4j.run(q).await?)
    }

    pub async fn delete_post(&self, author_id: i64, post_id: i64) -> Result<()> {
        let q = query(r#"
            match (u:Profile{id: $user_id})-[r:POSTED]->(p:Post)
            where id(p)=$post_id
            detach delete p
        "#)
            .param("user_id", author_id)
            .param("post_id", post_id);

        Ok(self.neo4j.run(q).await?)
    }

    pub async fn create_post(&self, author_id: i64, content: &str) -> Result<Post> {
        let query = query("match (a:Profile{id:$id}) create (a)-[r:POSTED{date: $time}]->(p:Post{content: $content}) return a,r,p")
            .param("id", author_id)
            .param("content", content)
            .param("time", chrono::Utc::now().naive_local());

        self.neo4j.execute(query)
            .await?
            .next()
            .await?
            .ok_or(Error::Neo4jQueryError)?
            .try_into()
    }

    pub async fn get_posts_for_user(&self, user_id: i64) -> Result<Vec<Post>> {
        let q = query(r#"
            match (p:Post)<-[r:POSTED]-(a:Profile{id: $id})
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