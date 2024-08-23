use crate::SysRes;
use axum_macros::FromRef;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

#[derive(Clone, FromRef)]
pub struct ActivationService {
    postgres: sea_orm::DatabaseConnection,
}

impl ActivationService {
    pub fn new(postgres: sea_orm::DatabaseConnection) -> Self {
        Self { postgres }
    }
}

impl ActivationService {
    pub async fn get_current_activation_mail_template(&self) -> SysRes<Option<ActivationEmail>> {
        Ok(Variables::find()
            .filter(variables::Column::Key.eq("activation_email"))
            .one(&self.postgres)
            .await?
            .map(|v| ActivationEmail { content: v.value }))
    }

    pub async fn set_current_activation_mail_template(&self, content: &str) -> SysRes<()> {
        let res = Variables::find()
            .filter(variables::Column::Key.eq("activation_email"))
            .one(&self.postgres)
            .await?;

        let model = if let Some(actual) = res {
            let mut model = actual.into_active_model();
            model.value = ActiveValue::Set(content.to_string());
            model
        } else {
            variables::ActiveModel {
                value: ActiveValue::Set(content.to_string()),
                key: ActiveValue::Set("activation_email".to_string()),
                ..Default::default()
            }
        };

        model.save(&self.postgres).await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivationEmail {
    pub content: String,
}
