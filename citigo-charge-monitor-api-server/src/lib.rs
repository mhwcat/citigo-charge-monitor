use std::{net::SocketAddr, str::FromStr};

use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http, middleware,
    web::{self},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::info;
use routes::{
    charge_session::{create_or_update_charge_session, get_charge_sessions},
    health::health_check,
    vehicle::{create_vehicle, get_all_vehicles, get_vehicle, get_vehicle_status, update_vehicle},
};
use services::auth::{validate_auth, RegistrationToken};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::routes::auth::{login, logout, register};

pub mod config;
mod error;
mod models;
mod routes;
mod services;

pub type RedisPool = deadpool_redis::Pool;

pub fn run(
    api_base_addr: &SocketAddr,
    db_connection_pool: MySqlPool,
    redis_connection_pool: RedisPool,
) -> std::io::Result<Server> {
    let db_connection_pool = web::Data::new(db_connection_pool);
    let redis_connection_pool = web::Data::new(redis_connection_pool);

    let registration_token_wrapper = web::Data::new(RegistrationToken {
        registration_token: Uuid::new_v4().to_string(),
    });
    info!(
        "Generated registration token: {}",
        registration_token_wrapper.registration_token
    );

    let server = HttpServer::new(move || {
        let cors = Cors::permissive()
            .expose_headers(vec![http::header::HeaderName::from_str("X-Count").unwrap()]);

        App::new()
            .app_data(db_connection_pool.clone())
            .app_data(redis_connection_pool.clone())
            .app_data(registration_token_wrapper.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(web::scope("/health").service(health_check))
                    .service(
                        web::scope("/auth")
                            .service(register)
                            .service(login)
                            .service(logout),
                    )
                    .service(
                        web::scope("")
                            .wrap(HttpAuthentication::bearer(validate_auth))
                            .service(get_vehicle)
                            .service(get_all_vehicles)
                            .service(get_vehicle_status)
                            .service(create_vehicle)
                            .service(update_vehicle)
                            .service(get_charge_sessions)
                            .service(create_or_update_charge_session),
                    ),
            )
    })
    .bind(api_base_addr)?
    .run();

    Ok(server)
}
