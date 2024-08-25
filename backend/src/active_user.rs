use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::{
    app_state::AppState,
    endpoint::account::SESSION_COOKIE_NAME,
    entities::{self, sea_orm_active_enums::AccountType},
    Error,
};

// FIXME: Admin User!!!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveUser {
    pub id: i64,
    pub role: ActiveUserRole,
}

#[async_trait]
impl<S> FromRequestParts<S> for ActiveUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = crate::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<Cookies>()
            .await
            .map_err(|_| Error::CookiesMissing)?;

        let auth = cookies
            .get(SESSION_COOKIE_NAME)
            .ok_or(Error::Unauthorized)?;

        let mut state = parts
            .extract_with_state::<AppState, _>(state)
            .await
            .map_err(|_| Error::AppStateMissing)?;

        Ok(state.account_service.verify_session(auth.value()).await?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ActiveUserRole {
    User,
    Moderator,
    Admin,
}

impl From<entities::account::Model> for ActiveUser {
    fn from(value: entities::account::Model) -> Self {
        Self {
            id: value.id,
            role: match value.r#type {
                AccountType::Admin => ActiveUserRole::Admin,
                AccountType::Moderator => ActiveUserRole::Moderator,
                AccountType::User => ActiveUserRole::User,
            },
        }
    }
}

impl ToString for ActiveUserRole {
    fn to_string(&self) -> String {
        match self {
            ActiveUserRole::User => "User".to_string(),
            ActiveUserRole::Moderator => "Moderator".to_string(),
            ActiveUserRole::Admin => "Admin".to_string(),
        }
    }
}