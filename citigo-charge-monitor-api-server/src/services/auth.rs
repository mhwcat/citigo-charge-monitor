use crate::{
    error::{ApiError, ApiResult},
    services, RedisPool,
};
use actix_web::http::header::HeaderValue;
use actix_web::http::header::ToStrError;

// Currently actix-web-httpauth does not work correctly with actix-cors (see https://github.com/actix/actix-extras/issues/127),
// so we implement barebones auth check and execute it manually in all protected endpoints.
// TODO: Use actix-web-httpauth middleware, once this is fixed (e.g. https://github.com/actix/actix-extras/pull/194 is merged)

pub async fn validate_auth(redis_pool: &RedisPool, token: Option<&HeaderValue>) -> ApiResult<()> {
    if let Some(token) = token {
        let token = extract_token_value_from_header(token)?;
        let session_present = services::user::is_user_session_present(redis_pool, &token).await?;

        match session_present {
            true => Ok(()),
            false => Err(ApiError::unauthorized()),
        }
    } else {
        Err(ApiError::unauthorized())
    }
}

// Wrapper for registration token to make it easier for web::Data
#[derive(Clone)]
pub struct RegistrationToken {
    pub registration_token: String,
}

pub fn extract_token_value_from_header(header: &HeaderValue) -> Result<String, ToStrError> {
    let token = header.to_str()?.replacen("Bearer ", "", 1);

    Ok(token)
}
