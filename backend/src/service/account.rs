use std::{collections::HashMap};

use axum::http::StatusCode;
use axum_macros::FromRef;
use redis::cmd;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use crate::entities::{*, prelude::*};

static ACCOUNT_PREFIX: &str = "account";

#[derive(Clone, FromRef)]
pub struct AccountService {
    redis: redis::aio::ConnectionManager,
    postgres: sea_orm::DatabaseConnection,
    expire_time_secs: u32,
}

impl AccountService {
    pub fn new(redis: redis::aio::ConnectionManager, postgres: sea_orm::DatabaseConnection) -> Self {
        Self { redis, postgres, expire_time_secs: 60*60*24 }
    }

    pub async fn activate_account(&mut self, email: &str, key: &str) -> Result<(), StatusCode> {
        let redis = &mut self.redis;

        let result = cmd("hgetall")
            .arg(build_prefix(email))
            .query_async::<_, Option<HashMap<String, String>>>(redis)
            .await
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
            ;

        if result.len() == 0 {
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
        
        Account::insert(model)
            .exec_without_returning(&self.postgres)
            .await
            .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        cmd("del")
            .arg(build_prefix(email))
            .query_async(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            ;

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
            .arg(build_prefix(email))
            .query_async::<redis::aio::ConnectionManager, i32>(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        if is_taken == 1 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let activation_key = generate_activation_key();

        cmd("hset")
            .arg(build_prefix(email))
            .arg("key")
            .arg(activation_key.clone())
            .arg("password")
            .arg(hash_password(&password))
            .query_async(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        cmd("expire")
            .arg(build_prefix(email))
            .arg(self.expire_time_secs)
            .query_async(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;
        
        Ok((email.to_string(), activation_key))
    }
}

pub fn generate_activation_key() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, 8).expect("To be hashsed").to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

pub fn build_prefix(email: &str) -> String {
    format!("{ACCOUNT_PREFIX}:{email}")
}