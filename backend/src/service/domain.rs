use std::sync::Arc;
use axum::async_trait;
use axum::extract::{FromRequestParts, State, Request};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum_macros::FromRef;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use tokio::sync::RwLock;
use crate::entities::{*, prelude::*};
use crate::Result;

static SYSTEM_DOMAIN_VAR_NAME: &str  = "system_domain";
static IMAGE_DOMAIN_VAR_NAME:  &str  = "image_domain";

#[derive(Clone, FromRef)]
pub struct DomainService {
    postgres: sea_orm::DatabaseConnection,
    image_domain: Arc<RwLock<Option<String>>>
}

impl DomainService {
    pub async fn new(postgres: sea_orm::DatabaseConnection) -> Self {
        let image_domain = Self::get_variable(&postgres, IMAGE_DOMAIN_VAR_NAME)
            .await
            .expect("Database connection to be established.");
        Self {
            postgres,
            image_domain: Arc::new(RwLock::new(image_domain))
        }
    }
}

impl DomainService {
    pub async fn set_system_domain(&self, value: &str) -> Result<()> {
        Self::set_variable(&self.postgres, SYSTEM_DOMAIN_VAR_NAME, value).await
    }

    pub async fn get_system_domain(&self) -> Result<Option<String>> {
        Self::get_variable(&self.postgres, SYSTEM_DOMAIN_VAR_NAME).await
    }

    pub async fn set_image_domain(&mut self, value: &str) -> Result<()> {
        if Self::set_variable(&self.postgres, IMAGE_DOMAIN_VAR_NAME, value).await.is_ok() {
            *self.image_domain.write().await = Some(value.to_owned())
        }
        Ok(())
    }

    pub async fn get_image_domain(&self) -> Result<Option<String>> {
        Ok(self.image_domain.read().await.clone())
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

pub async fn extract_image_domain(
    State(domain_service): State<DomainService>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    let image_domain = domain_service
        .get_image_domain()
        .await
        .unwrap_or(Some("".to_string()))
        .unwrap_or("".to_string());

    let mut response = request;
    response.extensions_mut().insert(ImageDomain(image_domain));

    Ok(next.run(response).await)
}

#[derive(Debug, Clone)]
pub struct ImageDomain(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for ImageDomain {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Self::Rejection> {
        if let Some(domain) = parts.extensions.get::<ImageDomain>() {
            Ok(domain.clone())
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}