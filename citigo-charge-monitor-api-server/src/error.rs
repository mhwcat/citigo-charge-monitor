use std::fmt::Display;

use actix_web::http::header::ToStrError;
use actix_web::{http, HttpResponse, ResponseError};
use deadpool_redis::{
    redis::{self, RedisError},
    PoolError,
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum ApiErrorType {
    ServerError,
    Unauthorized,
}
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    error_type: ApiErrorType,
    details: Option<String>,
}

#[allow(dead_code)]
impl ApiError {
    pub fn unauthorized() -> Self {
        Self {
            error_type: ApiErrorType::Unauthorized,
            details: Some("Unauthorized".to_string()),
        }
    }

    pub fn server_error(details: String) -> Self {
        Self {
            error_type: ApiErrorType::ServerError,
            details: Some(details),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}/{}",
            self.error_type,
            self.details.as_ref().unwrap_or(&"n/a".to_string())
        )
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        Self {
            error_type: ApiErrorType::ServerError,
            details: Some(err.to_string()),
        }
    }
}

impl From<RedisError> for ApiError {
    fn from(err: redis::RedisError) -> Self {
        Self {
            error_type: ApiErrorType::ServerError,
            details: Some(err.to_string()),
        }
    }
}

impl From<PoolError> for ApiError {
    fn from(err: PoolError) -> Self {
        Self {
            error_type: ApiErrorType::ServerError,
            details: Some(err.to_string()),
        }
    }
}

impl From<ToStrError> for ApiError {
    fn from(err: ToStrError) -> Self {
        Self {
            error_type: ApiErrorType::ServerError,
            details: Some(err.to_string()),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> http::StatusCode {
        match self.error_type {
            ApiErrorType::Unauthorized => http::StatusCode::UNAUTHORIZED,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();

        HttpResponse::build(status_code).json(self)
    }
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
