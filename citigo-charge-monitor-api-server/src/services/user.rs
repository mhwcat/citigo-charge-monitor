use chrono::Local;
use deadpool_redis::redis;
use log::info;
use sha2::{Digest, Sha256};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::ApiResult;
use crate::models::SimpleUser;
use crate::RedisPool;
use crate::{models::User, models::UserSession};

const USER_SESSION_EXPIRY_SECONDS: &str = "86400"; // 24h

pub async fn get_user_by_username(
    connection: &MySqlPool,
    username: &str,
) -> ApiResult<Option<User>> {
    let result = sqlx::query_as!(User, r#"SELECT * FROM users WHERE username = ?"#, username)
        .fetch_optional(connection)
        .await?;

    Ok(result)
}

pub async fn create_user(
    db_pool: &MySqlPool,
    username: &str,
    password: &str,
) -> ApiResult<Option<SimpleUser>> {
    let uuid = Uuid::new_v4().to_string();
    let username = username.trim();
    let hashed = hash_str(password);

    sqlx::query!(r#"INSERT INTO users(`id`, `username`, `password_hash`, `creation_time`) VALUES (?, ?, ?, NOW())"#, uuid, username, hashed)
        .execute(db_pool)
        .await?;

    let created_user = get_user_by_username(db_pool, username)
        .await?
        .map(SimpleUser::from);

    Ok(created_user)
}

pub async fn is_user_session_present(redis_pool: &RedisPool, id: &str) -> ApiResult<bool> {
    let mut conn = redis_pool.get().await?;
    let redis_key = format!("citigo/{}", id);

    // Check for user session and refresh its expiry time
    let user_id: Option<String> = redis::cmd("GETEX")
        .arg(&[&redis_key, "EX", USER_SESSION_EXPIRY_SECONDS])
        .query_async(&mut conn)
        .await?;

    Ok(user_id.is_some())
}

pub async fn authorize_user(
    redis_pool: &RedisPool,
    user_id: &str,
    password: &str,
    password_hash: &str,
) -> ApiResult<Option<UserSession>> {
    let hashed = hash_str(password);

    // If user is authorized, create new user session and return it
    if hashed.eq_ignore_ascii_case(password_hash) {
        let uuid = Uuid::new_v4().to_string();
        let redis_key = format!("citigo/{}", uuid);

        let mut redis_conn = redis_pool.get().await?;
        redis::cmd("SET")
            .arg(&[&redis_key, user_id, "EX", USER_SESSION_EXPIRY_SECONDS])
            .query_async::<_, ()>(&mut redis_conn)
            .await?;

        info!("User authorized [id={}]", user_id);

        Ok(Some(UserSession {
            id: uuid,
            creation_time: Local::now().naive_utc(),
            user_id: user_id.to_string(),
        }))
    // If user is not authorized, return None
    } else {
        info!("User not authorized [id={}]", user_id);

        Ok(None)
    }
}

pub async fn logout_user(redis_pool: &RedisPool, session_id: &str) -> ApiResult<bool> {
    let redis_key = format!("citigo/{}", session_id);

    let mut redis_conn = redis_pool.get().await?;
    redis::cmd("DEL")
        .arg(&[&redis_key])
        .query_async::<_, ()>(&mut redis_conn)
        .await?;

    info!("Session logged out [id={}]", session_id);

    Ok(true)
}

fn hash_str(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value);
    let hashed = format!("{:X}", hasher.finalize());

    hashed
}
