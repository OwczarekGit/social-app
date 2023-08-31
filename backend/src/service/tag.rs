use std::sync::Arc;
use axum::http::StatusCode;
use axum_macros::FromRef;
use neo4rs::{Graph, Node, query, Row};
use serde::{Deserialize, Serialize};

#[derive(Clone, FromRef)]
pub struct TagService {
    neo4j: Arc<Graph>,
}

impl TagService {
    pub fn new(neo4j: Arc<Graph>) -> Self {
        Self { neo4j }
    }
}

impl TagService {
    pub async fn create_tag(&self, name: &str) -> Result<DetailedTag, StatusCode> {
        if self.tag_node_exists(name.trim()).await? {
            return Err(StatusCode::CONFLICT);
        }

        Ok(self.neo4j.execute(
            query("create (t:Tag{name: $name}) return t")
                .param("name", name)
        )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .next()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .try_into()?)
    }

    pub async fn delete_tag(&self, id: i64) -> Result<(), StatusCode> {
        self.neo4j.run(
            query("match (t:Tag) where id(t) = $id detach delete t")
                .param("id", id)
        )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub async fn get_all_tags_with_usage(&self) -> Result<Vec<DetailedTag>, StatusCode> {
        let mut result = self.neo4j.execute(query(r#"
            MATCH (t:Tag)
            OPTIONAL MATCH (t)<-[rel:TAGGED_AS]-()
            WITH t, COUNT(rel) AS r
            RETURN t, r
            order by r desc
        "#))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut res = vec![];
        while let Ok(Some(row)) = result.next().await {
            if let Ok(tag) = DetailedTag::try_from(row) {
                res.push(tag);
            }
        }

        Ok(res)
    }

    pub async fn update_tag(&self, id: i64, name: &str) -> Result<(), StatusCode> {
        self.neo4j.run(
            query("match (t:Tag) where id(t) = $id set t.name = $name return t")
                .param("id", id)
                .param("name", name)
        ).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn tag_node_exists(&self, name: &str) -> Result<bool, StatusCode> {
        let q = query("match (t:Tag) where toLower(t.name) = toLower($name) return true AS x limit 1")
            .param("name", name.trim());

        let res = self.neo4j.execute(q)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .next()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if res.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedTag {
    pub id: i64,
    pub name: String,
    pub usage: i64,
}

impl TryFrom<Row> for DetailedTag {
    type Error = StatusCode;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let n: Node = value.get("t").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let c: i64 = value.get("r").unwrap_or(0);

        Ok(Self {
            id: n.id(),
            name: n.get("name").unwrap_or("<CORRUPTED_TAG>".to_owned()),
            usage: c
        })

    }
}