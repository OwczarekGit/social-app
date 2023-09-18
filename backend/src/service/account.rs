use std::collections::HashMap;
use std::sync::Arc;

use axum::http::StatusCode;
use axum_macros::FromRef;
use neo4rs::query;
use redis::cmd;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveValue, IntoActiveModel, ActiveModelTrait};
use tracing::log::debug;
use crate::{ActiveUser, ActiveUserRole};
use crate::{Result, Error};

use crate::entities::{*, prelude::*};
use crate::entities::sea_orm_active_enums::AccountType;

static ACCOUNT_PREFIX: &str = "account";
static SESSION_PREFIX: &str = "session";

#[derive(Clone, FromRef)]
pub struct AccountService {
    redis: redis::aio::ConnectionManager,
    postgres: sea_orm::DatabaseConnection,
    neo4j: Arc<neo4rs::Graph>,
    expire_time_secs: u32,
}

impl AccountService {
    pub fn new(
        redis: redis::aio::ConnectionManager,
        postgres: sea_orm::DatabaseConnection,
        neo4j: Arc<neo4rs::Graph>,
    ) -> Self {
        Self { redis, postgres, neo4j, expire_time_secs: 60*60*24 }
    }

    pub async fn verify_session(&mut self, session_key: &str) -> Result<ActiveUser> {
        let redis = &mut self.redis;
        
        let id = cmd("hget")
            .arg(build_prefix(SESSION_PREFIX, session_key))
            .arg("user_id")
            .query_async::<_, String>(redis)
            .await?;

        let id: i64 = id.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let user = Account::find_by_id(id)
            .one(&self.postgres)
            .await
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(ActiveUser::from(user))
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<(String, ActiveUserRole)> {
        let redis = &mut self.redis;

        let account = Account::find()
            .filter(account::Column::Email.eq(email))
            .one(&self.postgres)
            .await?
            .ok_or(Error::LoginError)?;

        if !verify_password(password, &account.password) {
            return Err(Error::LoginError);
        }

        let session_key = generate_session_key();

        // TODO: Set expires on cookies so that they have to be renewed periodically.
        cmd("hset")
            .arg(build_prefix(SESSION_PREFIX, &session_key))
            .arg("user_id")
            .arg(account.id)
            .query_async(redis)
            .await?;

        let role = match account.r#type {
            AccountType::Admin => ActiveUserRole::Admin,
            AccountType::User => ActiveUserRole::User
        };

        Ok((session_key, role))
    }

    pub async fn logout(&mut self, key: &str) {
        let redis = &mut self.redis;
        let _ = cmd("del")
            .arg(build_prefix(SESSION_PREFIX, key))
            .query_async::<_, i32>(redis)
            .await
            ;
    }

    pub async fn change_password(&self, user_id: i64, old_password: &str, new_password: &str) -> Result<()> {
        let account = Account::find_by_id(user_id)
            .one(&self.postgres)
            .await?
            .ok_or(Error::AccountForUpdatePasswordNotFound(user_id))?;

        if !verify_password(&old_password, &account.password) {
            return Err(Error::AccountForUpdatePasswordWrongPasswordProvided(user_id));
        }

        let mut account = account.into_active_model();

        // FIXME: Logout all sessions after the password is changed.
        account.password = ActiveValue::Set(hash_password(new_password));
        account.save(&self.postgres).await?;
        Ok(())
    }

    pub async fn activate_account(&mut self, email: &str, key: &str) -> Result<()> {
        let redis = &mut self.redis;

        let result = cmd("hgetall")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .query_async::<_, Option<HashMap<String, String>>>(redis)
            .await?
            .ok_or(Error::NonExistentAccountActivationAttempt)?
            ;

        if result.is_empty() {
            return Err(Error::NonExistentAccountActivationAttempt);
        }

        let actual_key = result.get("key").ok_or(Error::BadRequest)?;
        let password = result.get("password").ok_or(Error::BadRequest)?;

        if !actual_key.eq(key) {
            return Err(Error::AccountActivationWrongPassword);
        }

        let model = account::ActiveModel {
            email:    ActiveValue::Set(email.to_string()),
            password: ActiveValue::Set(password.to_string()),
            joined:   ActiveValue::Set(chrono::Utc::now().naive_utc()),
            r#type:   ActiveValue::Set(AccountType::User),
            ..Default::default()
        };
        
        let account = Account::insert(model)
            .exec(&self.postgres)
            .await?;

        cmd("del")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .query_async(redis)
            .await?;

        self.neo4j.run(
            query("merge (p:Profile{ id: $id, username: $username })")
                .param("id", account.last_insert_id)
                .param("username", "New User")
        ).await?;

        debug!("Activating '{email}' using the code '{key}'");
        Ok(())   
    }

    pub async fn register_account(&mut self, email: &str, password: &str) -> Result<(String, String)> {
        let redis = &mut self.redis;

        let is_taken = Account::find()
            .filter(account::Column::Email.eq(email))
            .one(&self.postgres)
            .await?;

        if is_taken.is_some() {
            return Err(Error::EmailTaken);
        }

        let is_taken: i32 = cmd("exists")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .query_async::<redis::aio::ConnectionManager, i32>(redis)
            .await?;

        if is_taken == 1 {
            return Err(Error::EmailTakenPendingActivation);
        }

        let activation_key = generate_activation_key();

        cmd("hset")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .arg("key")
            .arg(activation_key.clone())
            .arg("password")
            .arg(hash_password(password))
            .query_async(redis)
            .await?;

        cmd("expire")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .arg(self.expire_time_secs)
            .query_async(redis)
            .await?;

        #[cfg(debug_assertions)]
        {
            let _ = self.activate_account(email, &activation_key).await;
            tracing::warn!("In debug builds accounts are activated automatically.")
        }

        Ok((email.to_string(), activation_key))
    }
}

pub fn generate_activation_key() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, 8).expect("To be hashed").to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

pub fn build_prefix(prefix: &str, key: &str) -> String {
    format!("{prefix}:{key}")
}

pub fn generate_session_key() -> String {
    uuid::Uuid::new_v4().to_string()
}