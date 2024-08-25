use crate::entities::{prelude::*, *};
use crate::SysRes;
use axum_macros::FromRef;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use std::sync::Arc;
use tokio::sync::RwLock;

static SYSTEM_DOMAIN_VAR_NAME: &str = "system_domain";
static IMAGE_DOMAIN_VAR_NAME: &str = "image_domain";

#[derive(Clone, FromRef)]
pub struct DomainService {
    postgres: sea_orm::DatabaseConnection,
    image_domain: Arc<RwLock<Option<String>>>,
}

impl DomainService {
    pub async fn new(postgres: sea_orm::DatabaseConnection) -> Self {
        let image_domain = Self::get_variable(&postgres, IMAGE_DOMAIN_VAR_NAME)
            .await
            .expect("Database connection to be established.");
        Self {
            postgres,
            image_domain: Arc::new(RwLock::new(image_domain)),
        }
    }
}

impl DomainService {
    pub async fn set_system_domain(&self, value: &str) -> SysRes<()> {
        Self::set_variable(&self.postgres, SYSTEM_DOMAIN_VAR_NAME, value).await
    }

    pub async fn get_system_domain(&self) -> SysRes<Option<String>> {
        Self::get_variable(&self.postgres, SYSTEM_DOMAIN_VAR_NAME).await
    }

    pub async fn set_image_domain(&mut self, value: &str) -> SysRes<()> {
        if Self::set_variable(&self.postgres, IMAGE_DOMAIN_VAR_NAME, value)
            .await
            .is_ok()
        {
            *self.image_domain.write().await = Some(value.to_owned())
        }
        Ok(())
    }

    pub async fn get_image_domain(&self) -> SysRes<Option<String>> {
        Ok(self.image_domain.read().await.clone())
    }

    async fn get_variable(pg: &sea_orm::DatabaseConnection, key: &str) -> SysRes<Option<String>> {
        Ok(Variables::find()
            .filter(variables::Column::Key.eq(key))
            .one(pg)
            .await?
            .map(|m| m.value))
    }

    async fn set_variable(pg: &sea_orm::DatabaseConnection, key: &str, value: &str) -> SysRes<()> {
        let model = Variables::find()
            .filter(variables::Column::Key.eq(key))
            .one(pg)
            .await?;

        let mut model = match model {
            None => variables::ActiveModel {
                key: ActiveValue::Set(key.to_string()),
                ..Default::default()
            },
            Some(d) => d.into_active_model(),
        };

        model.value = ActiveValue::Set(value.to_string());
        model.save(pg).await?;

        Ok(())
    }
}
