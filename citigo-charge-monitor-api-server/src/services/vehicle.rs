use log::{error, info};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::services;
use crate::{
    error::{ApiError, ApiResult},
    models::{Vehicle, VehicleStatus},
};

pub async fn get_vehicle(
    db_pool: &MySqlPool,
    id: &Option<String>,
    vin: &Option<String>,
) -> ApiResult<Option<Vehicle>> {
    if id.is_some() && vin.is_none() {
        get_vehicle_by_id(db_pool, id.as_ref().unwrap()).await
    } else if id.is_none() && vin.is_some() {
        get_vehicle_by_vin(db_pool, vin.as_ref().unwrap()).await
    } else if id.is_some() && vin.is_some() {
        let result = sqlx::query_as!(
            Vehicle,
            r#"SELECT * FROM vehicles WHERE id = ? AND vin = ?"#,
            id.as_ref().unwrap(),
            vin.as_ref().unwrap()
        )
        .fetch_optional(db_pool)
        .await?;

        Ok(result)
    } else {
        let err = ApiError::server_error("Missing parameters (id, vin)".to_string());
        error!("{}", err);

        Err(err)
    }
}

async fn get_vehicle_by_id(db_pool: &MySqlPool, id: &str) -> ApiResult<Option<Vehicle>> {
    let result = sqlx::query_as!(Vehicle, r#"SELECT * FROM vehicles WHERE id = ?"#, id)
        .fetch_optional(db_pool)
        .await?;

    Ok(result)
}

async fn get_vehicle_by_vin(db_pool: &MySqlPool, vin: &str) -> ApiResult<Option<Vehicle>> {
    let result = sqlx::query_as!(Vehicle, r#"SELECT * FROM vehicles WHERE vin = ?"#, vin)
        .fetch_optional(db_pool)
        .await?;

    Ok(result)
}

pub async fn get_vehicles(db_pool: &MySqlPool) -> ApiResult<Vec<Vehicle>> {
    let result = sqlx::query_as!(Vehicle, r#"SELECT * FROM vehicles"#)
        .fetch_all(db_pool)
        .await?;

    Ok(result)
}

pub async fn get_vehicle_status(
    db_pool: &MySqlPool,
    vehicle_id: &str,
) -> ApiResult<Option<VehicleStatus>> {
    let vehicle = get_vehicle_by_id(db_pool, vehicle_id).await?;

    match vehicle {
        Some(vehicle) => {
            let current_charge_session =
                services::charge_session::get_current_charge_session_for_vehicle(
                    db_pool, vehicle_id,
                )
                .await?;

            Ok(Some(VehicleStatus {
                vehicle,
                is_charging: current_charge_session.is_some(),
                current_charge_session,
            }))
        }
        None => Ok(None),
    }
}

pub async fn create_vehicle(
    db_pool: &MySqlPool,
    vin: &str,
    target_soc: i8,
) -> ApiResult<Option<Vehicle>> {
    let vehicle = get_vehicle_by_vin(db_pool, vin).await?;

    match vehicle {
        Some(_) => Err(ApiError::server_error(
            "Vehicle with this VIN already exists!".to_string(),
        )),
        None => {
            let uuid = Uuid::new_v4().to_string();
            let mut tx = db_pool.begin().await?;
            sqlx::query!(r#"INSERT INTO vehicles(`id`, `vin`, `soc`, `target_soc`, `last_update_time`) VALUES (?, ?, NULL, ?, NOW())"#, 
                uuid, vin, target_soc)
                .execute(&mut tx).await?;
            tx.commit().await?;

            let created_vehicle = get_vehicle_by_id(db_pool, &uuid).await?;

            Ok(created_vehicle)
        }
    }
}

pub async fn update_vehicle(
    db_pool: &MySqlPool,
    id: &str,
    soc: Option<i8>,
    target_soc: Option<i8>,
) -> ApiResult<Option<Vehicle>> {
    let vehicle = get_vehicle_by_id(db_pool, id).await?;

    match vehicle {
        Some(_) => {
            let mut tx = db_pool.begin().await?;
            {
                if let Some(soc) = soc {
                    sqlx::query!(
                        r#"UPDATE vehicles SET soc = ?, last_update_time = NOW() WHERE id = ?"#,
                        soc,
                        id
                    )
                    .execute(&mut tx)
                    .await?;
                }

                if let Some(target_soc) = target_soc {
                    sqlx::query!(
                        r#"UPDATE vehicles SET target_soc = ?, last_update_time = NOW() WHERE id = ?"#,
                        target_soc,
                        id
                    )
                    .execute(&mut tx)
                    .await?;
                }
            }
            tx.commit().await?;

            let updated_vehicle = get_vehicle_by_id(db_pool, id)
                .await?
                .expect("Failed fetching updated Vehicle");

            info!(
                "Updated vehicle [id={}, vin={}, soc={:?}, targetSoc={}]",
                updated_vehicle.id,
                updated_vehicle.vin,
                updated_vehicle.soc,
                updated_vehicle.target_soc
            );

            Ok(Some(updated_vehicle))
        }
        None => Ok(None),
    }
}
