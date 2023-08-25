use axum::http::StatusCode;
use axum_macros::FromRef;
use redis::cmd;

static ACCOUNT_PREFIX: &str = "account";

#[derive(Clone, FromRef)]
pub struct AccountService {
    redis: redis::aio::ConnectionManager,
    expire_time_secs: u32
}

impl AccountService {
    pub fn new(redis: redis::aio::ConnectionManager) -> Self {
        Self { redis, expire_time_secs: 60*60*24 }
    }

    pub async fn activate_account(&mut self, email: &str, key: &str) -> Result<(), StatusCode> {
        println!("Activating '{email}' using the code '{key}'");
        Ok(())   
    }

    pub async fn register_account(&mut self, email: &str, password: &str) -> Result<(String, String), StatusCode> {
        let redis = &mut self.redis;

        //TODO: Check if email already exists as an registered user(outside of redis cache).

        let is_taken: i32 = cmd("exists")
            .arg(format!("{ACCOUNT_PREFIX}:{email}"))
            .query_async::<redis::aio::ConnectionManager, i32>(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        if is_taken == 1 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let activation_key = generate_activation_key();

        cmd("hset")
            .arg(format!("{ACCOUNT_PREFIX}:{email}"))
            .arg("key")
            .arg(activation_key.clone())
            .arg("password")
            .arg(hash_password(&password))
            .query_async(redis)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        cmd("expire")
            .arg(format!("{ACCOUNT_PREFIX}:{email}"))
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