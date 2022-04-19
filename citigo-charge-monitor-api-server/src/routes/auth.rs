use actix_web::{post, web, HttpRequest, HttpResponse};
//use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::MySqlPool;

use crate::{
    error::ApiError,
    models::{CreateUser, LoginUser},
    services::{self, auth::RegistrationToken},
    RedisPool,
};

#[post("/login")]
pub async fn login(
    _request: HttpRequest,
    db_pool: web::Data<MySqlPool>,
    redis_pool: web::Data<RedisPool>,
    body: web::Json<LoginUser>,
) -> Result<HttpResponse, ApiError> {
    let user = services::user::get_user_by_username(db_pool.get_ref(), &body.username).await?;

    match user {
        Some(user) => {
            let user_session = services::user::authorize_user(
                redis_pool.get_ref(),
                &user.id,
                &body.password,
                &user.password_hash,
            )
            .await?;

            match user_session {
                Some(user_session) => Ok(HttpResponse::Ok().json(user_session)),
                None => Ok(HttpResponse::Unauthorized().finish()),
            }
        }
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[post("/logout")]
pub async fn logout(
    request: HttpRequest,
    redis_pool: web::Data<RedisPool>,
) -> Result<HttpResponse, ApiError> {
    let token = request.headers().get("Authorization");

    if let Some(token) = token {
        let token = services::auth::extract_token_value_from_header(token)?;
        let _ = services::user::logout_user(redis_pool.get_ref(), &token).await?;

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::server_error("Missing session token".to_string()))
    }
}

#[post("/register")]
pub async fn register(
    request: HttpRequest,
    db_pool: web::Data<MySqlPool>,
    registration_token_wrapper: web::Data<RegistrationToken>,
    body: web::Json<CreateUser>,
) -> Result<HttpResponse, ApiError> {
    let token = request.headers().get("Authorization");

    if let Some(token) = token {
        let token = services::auth::extract_token_value_from_header(token)?;

        if token.eq_ignore_ascii_case(&registration_token_wrapper.get_ref().registration_token) {
            let user =
                services::user::create_user(db_pool.get_ref(), &body.username, &body.password)
                    .await?
                    .expect("Cannot fetch created user");

            Ok(HttpResponse::Ok().json(user))
        } else {
            Ok(HttpResponse::Unauthorized().finish())
        }
    } else {
        Err(ApiError::server_error("Missing session token".to_string()))
    }
}
