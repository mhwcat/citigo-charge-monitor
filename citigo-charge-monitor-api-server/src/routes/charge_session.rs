use actix_web::{get, post, web, HttpResponse};
use sqlx::MySqlPool;

use crate::{
    error::ApiError,
    models::{CreateUpdateChargeSession, GetChargeSessionsForVehicle},
    services,
};

#[get("/chargeSession")]
async fn get_charge_sessions(
    db_pool: web::Data<MySqlPool>,
    query: web::Query<GetChargeSessionsForVehicle>,
) -> Result<HttpResponse, ApiError> {
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
    db_pool: web::Data<MySqlPool>,
    body: web::Json<CreateUpdateChargeSession>,
) -> Result<HttpResponse, ApiError> {
    let charge_session = services::charge_session::create_or_update_charge_session(
        db_pool.get_ref(),
        body.soc,
        &body.vehicle_id,
        body.stop,
    )
    .await?;

    Ok(HttpResponse::Ok().json(charge_session))
}
