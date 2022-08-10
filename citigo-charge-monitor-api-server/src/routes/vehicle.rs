use actix_web::{get, post, put, web, HttpResponse};
use sqlx::MySqlPool;

use crate::{
    error::ApiError,
    models::{CreateVehicle, GetVehicle, GetVehicleStatus, UpdateVehicle},
    services,
};

#[get("/vehicle")]
async fn get_vehicle(
    query: web::Query<GetVehicle>,
    db_pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, ApiError> {
    let vehicle = services::vehicle::get_vehicle(db_pool.get_ref(), &query.id, &query.vin).await?;

    match vehicle {
        Some(vehicle) => Ok(HttpResponse::Ok().json(vehicle)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[get("/vehicle/all")]
async fn get_all_vehicles(db_pool: web::Data<MySqlPool>) -> Result<HttpResponse, ApiError> {
    let vehicles = services::vehicle::get_vehicles(db_pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(vehicles))
}

#[get("/vehicle/status")]
async fn get_vehicle_status(
    query: web::Query<GetVehicleStatus>,
    db_pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, ApiError> {
    let vehicle_status =
        services::vehicle::get_vehicle_status(db_pool.get_ref(), &query.id).await?;

    match vehicle_status {
        Some(vehicle_status) => Ok(HttpResponse::Ok().json(vehicle_status)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("/vehicle")]
async fn create_vehicle(
    db_pool: web::Data<MySqlPool>,
    body: web::Json<CreateVehicle>,
) -> Result<HttpResponse, ApiError> {
    let vehicle = services::vehicle::create_vehicle(
        db_pool.get_ref(),
        &body.vin,
        body.target_soc
            .try_into()
            .expect("Failed converting target SOC"),
    )
    .await?;

    match vehicle {
        Some(vehicle) => Ok(HttpResponse::Ok().json(vehicle)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[put("/vehicle")]
async fn update_vehicle(
    db_pool: web::Data<MySqlPool>,
    body: web::Json<UpdateVehicle>,
) -> Result<HttpResponse, ApiError> {
    let vehicle =
        services::vehicle::update_vehicle(db_pool.get_ref(), &body.id, body.soc, body.target_soc)
            .await?;

    match vehicle {
        Some(vehicle) => Ok(HttpResponse::Ok().json(vehicle)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
