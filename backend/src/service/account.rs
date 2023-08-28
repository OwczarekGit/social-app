use std::collections::HashMap;
use std::sync::Arc;

use axum::http::StatusCode;
use axum_macros::FromRef;
use neo4rs::query;
use redis::cmd;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use crate::entities::{*, prelude::*};

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

    pub async fn verify_session(&mut self, session_key: &str) -> Result<i64, StatusCode> {
        let redis = &mut self.redis;
        
        let id = cmd("hget")
            .arg(build_prefix(SESSION_PREFIX, session_key))
            .arg("user_id")
            .query_async::<_, String>(redis)
            .await
            .map_err(|_|StatusCode::UNAUTHORIZED)?
        ;

        id.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<String, StatusCode> {
        let redis = &mut self.redis;

        let account = Account::find()
            .filter(account::Column::Email.eq(email))
            .one(&self.postgres)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
        ;

        if !verify_password(password, &account.password) {
            return Err(StatusCode::FORBIDDEN);
        }

        let session_key = generate_session_key();

        // TODO: Set expires on cookies so that they have to be renewed periodically.
        cmd("hset")
            .arg(build_prefix(SESSION_PREFIX, &session_key))
            .arg("user_id")
            .arg(account.id)
            .query_async(redis)
            .await
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        Ok(session_key)
    }

    pub async fn logout(&mut self, key: &str) {
        let redis = &mut self.redis;
        let _ = cmd("del")
            .arg(build_prefix(SESSION_PREFIX, key))
            .query_async::<_, i32>(redis)
            .await
            ;
    }

    pub async fn activate_account(&mut self, email: &str, key: &str) -> Result<(), StatusCode> {
        let redis = &mut self.redis;

        let result = cmd("hgetall")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .query_async::<_, Option<HashMap<String, String>>>(redis)
            .await
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
            ;

        if result.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let actual_key = result.get("key").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let password = result.get("password").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        if !actual_key.eq(key) {
            return Err(StatusCode::BAD_REQUEST);
        }

        let model = account::ActiveModel {
            email: sea_orm::ActiveValue::Set(email.to_string()),
            password: sea_orm::ActiveValue::Set(password.to_string()),
            joined: sea_orm::ActiveValue::set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        
        let account = Account::insert(model)
            .exec(&self.postgres)
            .await
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        cmd("del")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .query_async(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        self.neo4j.run(
            query("merge (p:Profile{ id: $id, username: $username })")
                .param("id", account.last_insert_id)
                .param("username", "New User")
        ).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        println!("Activating '{email}' using the code '{key}'");
        Ok(())   
    }

    pub async fn register_account(&mut self, email: &str, password: &str) -> Result<(String, String), StatusCode> {
        let redis = &mut self.redis;

        let is_taken = Account::find()
            .filter(account::Column::Email.eq(email))
            .one(&self.postgres)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        if is_taken.is_some() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let is_taken: i32 = cmd("exists")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .query_async::<redis::aio::ConnectionManager, i32>(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        if is_taken == 1 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let activation_key = generate_activation_key();

        cmd("hset")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .arg("key")
            .arg(activation_key.clone())
            .arg("password")
            .arg(hash_password(password))
            .query_async(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        cmd("expire")
            .arg(build_prefix(ACCOUNT_PREFIX, email))
            .arg(self.expire_time_secs)
            .query_async(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        #[cfg(debug_assertions)]
        let _ = self.activate_account(&email, &activation_key).await;

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