use actix_web::{get, post, web, HttpRequest, HttpResponse};
use sqlx::MySqlPool;

use crate::{
    error::ApiError,
    models::{CreateUpdateChargeSession, GetChargeSessionsForVehicle},
    services, RedisPool,
};

#[get("/chargeSession")]
async fn get_charge_sessions(
    request: HttpRequest,
    db_pool: web::Data<MySqlPool>,
    redis_pool: web::Data<RedisPool>,
    query: web::Query<GetChargeSessionsForVehicle>,
) -> Result<HttpResponse, ApiError> {
    services::auth::validate_auth(redis_pool.get_ref(), request.headers().get("Authorization"))
        .await?;

    let charge_sessions = services::charge_session::get_finished_charge_sessions_for_vehicle(
        db_pool.get_ref(),
        &query.vehicle_id,
        query.index,
        query.page_size,
    )
    .await?;
    let charge_sessions_count =
        services::charge_session::get_finished_charge_sessions_for_vehicle_count(
            db_pool.get_ref(),
            &query.vehicle_id,
        )
        .await?;

    Ok(HttpResponse::Ok()
        .append_header(("X-Count", charge_sessions_count))
        .json(charge_sessions))
}

#[post("/chargeSession")]
async fn create_or_update_charge_session(
    request: HttpRequest,
    db_pool: web::Data<MySqlPool>,
    redis_pool: web::Data<RedisPool>,
    body: web::Json<CreateUpdateChargeSession>,
) -> Result<HttpResponse, ApiError> {
    services::auth::validate_auth(redis_pool.get_ref(), request.headers().get("Authorization"))
        .await?;

    let charge_session = services::charge_session::create_or_update_charge_session(
        db_pool.get_ref(),
        body.soc,
        &body.vehicle_id,
        body.stop,
    )
    .await?;

    Ok(HttpResponse::Ok().json(charge_session))
}
