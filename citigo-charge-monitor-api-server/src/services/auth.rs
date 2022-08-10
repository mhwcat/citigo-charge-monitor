use crate::RedisPool;
use actix_web::http::header::ToStrError;
use actix_web::{
    dev::ServiceRequest,
    error::{Error, ErrorInternalServerError},
    http::header::HeaderValue,
    web,
};
use actix_web_httpauth::{
    extractors::{bearer::BearerAuth, AuthenticationError},
    headers::www_authenticate::bearer::Bearer,
};

use super::user;

pub async fn validate_auth(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let redis_pool = req
        .app_data::<web::Data<RedisPool>>()
        .expect("Redis pool missing");
    let session_present = user::is_user_session_present(redis_pool, credentials.token()).await;

    match session_present {
        Ok(present) => {
            if present {
                Ok(req)
            } else {
                Err((AuthenticationError::new(Bearer::default()).into(), req))
            }
        }
        Err(e) => Err((ErrorInternalServerError(e), req)),
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
