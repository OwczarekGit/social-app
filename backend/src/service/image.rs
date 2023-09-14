use std::io::{Cursor};
use std::sync::Arc;
use axum::http::StatusCode;
use axum_macros::FromRef;
use image::{DynamicImage, ImageOutputFormat};
use minio_rsc::Minio;
use minio_rsc::types::args::ObjectArgs;
use neo4rs::{Graph, Node, query, Row};
use serde::{Deserialize, Serialize};


#[derive(Clone, FromRef)]
pub struct ImageService {
    neo4j: Arc<Graph>,
    minio: Minio
}

impl ImageService {
    pub fn new(neo4j: Arc<Graph>, minio: Minio) -> Self {
        Self { neo4j, minio }
    }

    pub async fn get_all_tags(&self) -> crate::Result<Vec<Tag>> {
        let mut tags = self.neo4j.execute(query("match (t:Tag) return t order by t.name"))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut res = vec![];
        while let Ok(Some(tag)) = tags.next().await {
            if let Ok(t) = Tag::try_from(tag) {
                res.push(t)
            }
        }

        Ok(res)
    }

    //FIXME: Thumbnail sized images.
    pub async fn upload_image(&self, user_id: i64, title: &str, mut tags: Vec<String>, image: DynamicImage) -> crate::Result<()> {
        let object_name = uuid::Uuid::new_v4().to_string() + ".png";

        let mut bytes = Vec::new();
        image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
            .expect("Should not happen at this point.");

        self.minio.put_object(
            ObjectArgs::new("images", object_name.clone())
                .content_type(Some("image/png".to_string())), bytes.into()
        )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let create_image_node_query = query("create (i:Image{title: $title, url: $url}) return i")
            .param("title", title)
            .param("url", format!("/images/{object_name}"));

        let image_node_id = self.neo4j.execute(create_image_node_query)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .next()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .get::<Node>("i")
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .id()
            ;

        let add_author_query = query(r#"
            match (i:Image), (p:Profile{id: $user_id})
            where id(i) = $id
            create (p)-[r:SHARED{date: $date}]->(i)
            return p,i"#)
            .param("id", image_node_id)
            .param("user_id", user_id)
            .param("date", chrono::Utc::now().naive_local());

        self.neo4j.run(add_author_query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        tags.dedup_by(|a,b| {
            a.to_lowercase().trim() == b.to_lowercase().trim()
        });
        self.connect_image_to_tags(image_node_id, tags).await?;

        Ok(())
    }

    async fn connect_image_to_tags(&self, image_id: i64, tags: Vec<String>) -> crate::Result<()> {
        for tag in tags {
            self.tag_image(image_id, &tag).await?;
        }

        Ok(())
    }

    async fn tag_image(&self, image_id: i64, name: &str) -> crate::Result<()> {
        let tag_image_query = query(r#"
            match (t:Tag), (i:Image)
            where toLower(t.name) = toLower($name)
            and id(i) = $id
            create (i)-[:TAGGED_AS]->(t)
            return t,i
        "#)
            .param("name", name.trim())
            .param("id", image_id);


        if self.tag_node_exists(name).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
            self.neo4j.run(tag_image_query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else {
            self.neo4j.run(query("create (t:Tag{name: $name})").param("name", name.trim()))
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            self.neo4j.run(tag_image_query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }

        Ok(())
    }

    async fn tag_node_exists(&self, name: &str) -> crate::Result<bool> {
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
pub struct Tag {
    name: String,
}

impl TryFrom<Row> for Tag {
    type Error = StatusCode;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let t = value.get::<Node>("t")
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Self {
            name: t.get("name").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        })
    }
}