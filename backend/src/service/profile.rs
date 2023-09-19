use std::io::Cursor;
use std::sync::Arc;
use axum::http::StatusCode;
use axum_macros::FromRef;
use image::{DynamicImage, ImageOutputFormat};
use image::imageops::FilterType;
use minio_rsc::Minio;
use minio_rsc::types::args::ObjectArgs;
use neo4rs::{Graph, Node, query, Row};
use serde::{Deserialize, Serialize};

static PROFILE_PICTURE_SIZE: u32 = 256;

#[derive(Clone, FromRef)]
pub struct ProfileService {
    neo4j: Arc<Graph>,
    minio: Minio,
}

impl ProfileService {
    pub fn new(neo4j: Arc<Graph>, minio: Minio) -> Self {
        Self { neo4j, minio }
    }
}

impl ProfileService {
    pub async fn change_username(&self, user_id: i64, username: &str) -> Result<(), StatusCode> {
        self.neo4j.run(
            query("match (p:Profile{id: $id}) set p.username=$username")
                .param("id", user_id)
                .param("username", username)
        )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
    pub async fn change_profile_picture(&self, user_id: i64, picture: DynamicImage) -> crate::Result<()> {
        let object_name = format!("{}.png", user_id);
        let scaled = picture.resize_to_fill(PROFILE_PICTURE_SIZE, PROFILE_PICTURE_SIZE, FilterType::Triangle);

        let mut bytes = Vec::new();
        scaled.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
            .expect("Should not happen at this point.");

        self.minio.put_object(
            ObjectArgs::new("profiles", object_name.clone())
                .content_type(Some("image/png".to_string())), bytes.into()
        ).await?;

        let q = query("match (p:Profile{id: $id}) set p.picture_url = $url return p")
            .param("id", user_id)
            .param("url", format!("/profiles/{}", object_name));

        self.neo4j.run(q).await?;

        Ok(())
    }

    pub async fn get_profile(&self, user_id: i64) -> Result<Profile, StatusCode> {
        let profile = self.neo4j.execute(
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
    user_id: i64,
    username: String,
    picture_url: String,
}

impl TryFrom<Row> for Profile {
    type Error = StatusCode;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let p: Node = value.get("p").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Self {
            user_id: p.get("id").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
            username: p.get("username").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
            picture_url: p.get("picture_url").unwrap_or("".to_string()),
        })
    }
}