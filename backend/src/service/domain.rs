use axum_macros::FromRef;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use crate::entities::{*, prelude::*};
use crate::{Result};

static SYSTEM_DOMAIN_VAR_NAME: &'static str  = "system_domain";
static IMAGE_DOMAIN_VAR_NAME:  &'static str  = "image_domain";

#[derive(Clone, FromRef)]
pub struct DomainService {
    postgres: sea_orm::DatabaseConnection,
}

impl DomainService {
    pub fn new(postgres: sea_orm::DatabaseConnection) -> Self {
        Self { postgres }
    }
}

impl DomainService {
    pub async fn set_system_domain(&self, value: &str) -> Result<()> {
        Self::set_variable(&self.postgres, SYSTEM_DOMAIN_VAR_NAME, value).await
    }

    pub async fn get_system_domain(&self) -> Result<Option<String>> {
        Self::get_variable(&self.postgres, SYSTEM_DOMAIN_VAR_NAME).await
    }

    pub async fn set_image_domain(&self, value: &str) -> Result<()> {
        Self::set_variable(&self.postgres, IMAGE_DOMAIN_VAR_NAME, value).await
    }

    pub async fn get_image_domain(&self) -> Result<Option<String>> {
        Self::get_variable(&self.postgres, IMAGE_DOMAIN_VAR_NAME).await
    }

    async fn get_variable(pg: &sea_orm::DatabaseConnection, key: &str) -> Result<Option<String>> {
        Ok(
            Variables::find()
                .filter(variables::Column::Key.eq(key))
                .one(pg)
                .await?
                .map(|m| m.value)
        )
    }

    async fn set_variable(pg: &sea_orm::DatabaseConnection, key: &str, value: &str) -> Result<()> {
        let model = Variables::find()
            .filter(variables::Column::Key.eq(key))
            .one(pg)
            .await?;

        let mut model = match model {
            None => {
                variables::ActiveModel {
                    key: ActiveValue::Set(key.to_string()),
                    ..Default::default()
                }
            }
            Some(d) => d.into_active_model()
        };

        model.value = ActiveValue::Set(value.to_string());
        model
            .save(pg)
            .await?;

        Ok(())
    }
}