use std::sync::Arc;
use axum_macros::FromRef;
use neo4rs::{Graph, Node, query, Row};
use serde::{Deserialize, Serialize};

use crate::{Result, Error};

#[derive(Clone, FromRef)]
pub struct WallpaperService {
    neo4j: Arc<Graph>,
}

impl WallpaperService {
    pub fn new(neo4j: Arc<Graph>) -> Self {
        Self { neo4j }
    }
}

impl WallpaperService {
    pub async fn get_all_wallpapers(&self) -> Result<Vec<Image>> {
        let q = query("match (i:Image)-[:TAGGED_AS]->(:Tag{name: 'Wallpaper'}) return i");

        let mut results = self.neo4j.execute(q)
            .await?;

        let mut res = vec![];
        while let Ok(Some(row)) = results.next().await {
            if let Ok(img) = Image::try_from(row) {
                res.push(img);
            }
        }

        Ok(res)
    }

    pub async fn get_current_wallpaper(&self, user_id: i64) -> Result<Option<Image>> {
        let q = query(r#"
            optional match (p:Profile{id: $id})-[:WALLPAPER]-(i:Image)
            return i"#
        )
            .param("id", user_id);

        Ok(
            self.neo4j.execute(q)
                .await?
                .next()
                .await?
                .ok_or(Error::Neo4jIOError)?
                .try_into()
                .ok()
        )
    }

    pub async fn unset_wallpaper(&self, user_id: i64) -> Result<()> {
        self.neo4j.run(
            query("optional match (p:Profile{id: $id})-[w:WALLPAPER]->(i:Image) delete w")
                .param("id", user_id)
        ).await?;

        Ok(())
    }

    pub async fn set_wallpaper(&self, user_id: i64, image_id: i64) -> Result<Image> {
        let q = query(r#"
            match (p:Profile{id: $id})
            optional match (p)-[w:WALLPAPER]->(i:Image)
            delete w
            with p
            match (i:Image) where id(i) = $image_id
            merge (p)-[w:WALLPAPER]->(i)
            return p, w, i"#
        )
            .param("id", user_id)
            .param("image_id", image_id);

        self.neo4j.execute(q)
            .await?
            .next()
            .await?
            .ok_or(Error::Neo4jIOError)?
            .try_into()
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: i64,
    pub title: String,
    pub url: String,
}

impl TryFrom<Row> for Image {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self> {
        let t = value.get::<Node>("i")
            .map_err(|_| Error::Neo4jNodeNotFound)?;

        Ok(Self {
            id: t.id(),
            title: t.get("title").unwrap_or("".to_string()),
            url: t.get("url").map_err(|_| Error::Neo4jInvalidNode(t.id()))?
        })
    }
}
