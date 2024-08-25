use crate::{Error, SysRes};
use axum_macros::FromRef;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use minio_rsc::client::KeyArgs;
use minio_rsc::Minio;
use neo4rs::{query, Graph, Node, Row};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;

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
    pub async fn change_username(&self, user_id: i64, username: &str) -> SysRes<()> {
        Ok(self
            .neo4j
            .run(
                query("match (p:Profile{id: $id}) set p.username=$username")
                    .param("id", user_id)
                    .param("username", username),
            )
            .await?)
    }
    pub async fn change_profile_picture(&self, user_id: i64, picture: DynamicImage) -> SysRes<()> {
        let object_name = format!("{}.png", user_id);
        let scaled = picture.resize_to_fill(
            PROFILE_PICTURE_SIZE,
            PROFILE_PICTURE_SIZE,
            FilterType::Triangle,
        );

        let mut bytes = Vec::new();
        scaled
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .expect("Should not happen at this point.");

        let key = KeyArgs::new(object_name.clone()).content_type(Some("image/png".to_string()));

        self.minio.put_object("profiles", key, bytes.into()).await?;

        let q = query("match (p:Profile{id: $id}) set p.picture_url = $url return p")
            .param("id", user_id)
            .param("url", format!("/profiles/{}", object_name));

        self.neo4j.run(q).await?;

        Ok(())
    }

    pub async fn get_profile(&self, user_id: i64) -> SysRes<Profile> {
        let profile = self
            .neo4j
            .execute(query("match (p:Profile{id: $id}) return p").param("id", user_id))
            .await?
            .next()
            .await?
            .ok_or(Error::ProfileFetchError(user_id))?
            .try_into()?;

        Ok(profile)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub user_id: i64,
    pub username: String,
    pub picture_url: String,
}

impl TryFrom<Row> for Profile {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let p: Node = value
            .get("p")
            .map_err(|_| Error::Neo4rsMissingParam("p".to_owned()))?;

        Ok(Self {
            user_id: p
                .get("id")
                .map_err(|_| Error::Neo4rsMissingParam("id".to_owned()))?,
            username: p
                .get("username")
                .map_err(|_| Error::Neo4rsMissingParam("username".to_owned()))?,
            picture_url: p.get("picture_url").unwrap_or("".to_string()),
        })
    }
}
