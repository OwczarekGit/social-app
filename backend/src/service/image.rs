use axum::http::StatusCode;
use axum_macros::FromRef;
use image::{DynamicImage, ImageFormat};
use minio_rsc::client::KeyArgs;
use minio_rsc::Minio;
use neo4rs::{query, Graph, Node, Row};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

use crate::Error;
use crate::SysRes;

#[derive(Clone, FromRef)]
pub struct ImageService {
    neo4j: Arc<Graph>,
    minio: Minio,
}

impl ImageService {
    pub fn new(neo4j: Arc<Graph>, minio: Minio) -> Self {
        Self { neo4j, minio }
    }

    pub async fn get_all_tags(&self) -> SysRes<Vec<Tag>> {
        let mut tags = self
            .neo4j
            .execute(query("match (t:Tag) return t order by t.name"))
            .await?;

        let mut res = vec![];
        while let Ok(Some(tag)) = tags.next().await {
            if let Ok(t) = Tag::try_from(tag) {
                res.push(t)
            }
        }

        Ok(res)
    }

    //FIXME: Thumbnail sized images.
    pub async fn upload_image(
        &self,
        user_id: i64,
        title: &str,
        mut tags: Vec<String>,
        image: DynamicImage,
    ) -> SysRes<()> {
        let object_name = uuid::Uuid::new_v4().to_string() + ".png";

        let mut bytes = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .expect("Should not happen at this point.");

        let key = KeyArgs::new(object_name.clone()).content_type(Some("image/png".to_string()));

        self.minio.put_object("images", key, bytes.into()).await?;

        let create_image_node_query = query("create (i:Image{title: $title, url: $url}) return i")
            .param("title", title)
            .param("url", format!("/images/{object_name}"));

        let image_node_id = self
            .neo4j
            .execute(create_image_node_query)
            .await?
            .next()
            .await?
            .ok_or(Error::ImageCreationFailed)?
            .get::<Node>("i")?
            .id();

        let add_author_query = query(
            r#"
            match (i:Image), (p:Profile{id: $user_id})
            where id(i) = $id
            create (p)-[r:SHARED{date: $date}]->(i)
            return p,i"#,
        )
        .param("id", image_node_id)
        .param("user_id", user_id)
        .param("date", chrono::Utc::now().naive_local());

        self.neo4j.run(add_author_query).await?;

        tags.dedup_by(|a, b| a.to_lowercase().trim() == b.to_lowercase().trim());
        self.connect_image_to_tags(image_node_id, tags).await?;

        Ok(())
    }

    async fn connect_image_to_tags(&self, image_id: i64, tags: Vec<String>) -> SysRes<()> {
        for tag in tags {
            self.tag_image(image_id, &tag).await?;
        }

        Ok(())
    }

    async fn tag_image(&self, image_id: i64, name: &str) -> SysRes<()> {
        let tag_image_query = query(
            r#"
            match (t:Tag), (i:Image)
            where toLower(t.name) = toLower($name)
            and id(i) = $id
            create (i)-[:TAGGED_AS]->(t)
            return t,i
        "#,
        )
        .param("name", name.trim())
        .param("id", image_id);

        if self.tag_node_exists(name).await? {
            self.neo4j.run(tag_image_query).await?
        } else {
            self.neo4j
                .run(query("create (t:Tag{name: $name})").param("name", name.trim()))
                .await?;
            self.neo4j.run(tag_image_query).await?;
        }

        Ok(())
    }

    async fn tag_node_exists(&self, name: &str) -> SysRes<bool> {
        let q =
            query("match (t:Tag) where toLower(t.name) = toLower($name) return true AS x limit 1")
                .param("name", name.trim());

        let res = self.neo4j.execute(q).await?.next().await?;

        if res.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    name: String,
}

impl TryFrom<Row> for Tag {
    type Error = StatusCode;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let t = value
            .get::<Node>("t")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Self {
            name: t
                .get("name")
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        })
    }
}
