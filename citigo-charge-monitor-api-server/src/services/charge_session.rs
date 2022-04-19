use log::info;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::{error::ApiResult, models::ChargeSession};

pub async fn get_charge_session_by_id(
    connection: &MySqlPool,
    charge_session_id: &str,
) -> ApiResult<Option<ChargeSession>> {
    let result = sqlx::query_as!(
        ChargeSession,
        r#"SELECT * FROM charge_sessions WHERE id = ?"#,
        charge_session_id
    )
    .fetch_optional(connection)
    .await?;

    Ok(result)
}

pub async fn get_finished_charge_sessions_for_vehicle(
    connection: &MySqlPool,
    vehicle_id: &str,
    index: u32,
    page_size: u32,
) -> ApiResult<Vec<ChargeSession>> {
    let result = sqlx::query_as!(
        ChargeSession,
        r#"SELECT * FROM charge_sessions WHERE vehicle_id = ? AND stop_time IS NOT NULL ORDER BY start_time DESC LIMIT ?, ?"#,
        vehicle_id,
        index,
        page_size
    )
    .fetch_all(connection)
    .await?;

    Ok(result)
}

pub async fn get_finished_charge_sessions_for_vehicle_count(
    connection: &MySqlPool,
    vehicle_id: &str,
) -> ApiResult<i64> {
    let result = sqlx::query!(
        r#"SELECT COUNT(*) AS cnt FROM charge_sessions WHERE vehicle_id = ? AND stop_time IS NOT NULL"#,
        vehicle_id
    )
    .fetch_one(connection)
    .await?;

    Ok(result.cnt)
}

pub async fn get_current_charge_session_for_vehicle(
    connection: &MySqlPool,
    vehicle_id: &str,
) -> ApiResult<Option<ChargeSession>> {
    let charge_session = sqlx::query_as!(ChargeSession, r#"SELECT * FROM charge_sessions WHERE vehicle_id = ? AND stop_time IS NULL ORDER BY start_time DESC LIMIT 1"#, vehicle_id)
        .fetch_optional(connection)
        .await?;

    Ok(charge_session)
}

pub async fn create_or_update_charge_session(
    connection: &MySqlPool,
    soc: i8,
    vehicle_id: &str,
    stop: bool,
) -> ApiResult<ChargeSession> {
    // Check if there is charge session in progress
    let charge_session_in_progress =
        get_current_charge_session_for_vehicle(connection, vehicle_id).await?;

    // We want to update existing session
    let charge_session = if let Some(charge_session_in_progress) = charge_session_in_progress {
        let tx = connection.begin().await?;
        {
            // Stop existing charge session
            if stop {
                sqlx::query!(r#"UPDATE charge_sessions SET current_soc = ?, stop_time = NOW(), stop_soc = ?, last_update_time = NOW() WHERE id = ?"#, 
                    soc, soc, charge_session_in_progress.id)
                        .execute(connection).await?;
            // Update SOC in existing charge session
            } else {
                sqlx::query!(r#"UPDATE charge_sessions SET current_soc = ?, last_update_time = NOW() WHERE id = ?"#, 
                    soc, charge_session_in_progress.id)
                        .execute(connection).await?;
            }

            sqlx::query!(
                r#"UPDATE vehicles SET soc = ?, last_update_time = NOW() WHERE id = ?"#,
                soc,
                vehicle_id
            )
            .execute(connection)
            .await?;
        }
        tx.commit().await?;

        let updated_charge_session =
            get_charge_session_by_id(connection, &charge_session_in_progress.id)
                .await?
                .expect("Failed fetching updated Charge session");

        info!("Updated charge session [id={}, startTime={}, stopTime={:?}, startSoc={}, currentSoc={:?}, stopSoc={:?}]",
            updated_charge_session.id, updated_charge_session.start_time, updated_charge_session.stop_time, updated_charge_session.start_soc, updated_charge_session.current_soc,
            updated_charge_session.stop_soc);

        updated_charge_session
    // We want to create new Charge session
    } else {
        let uuid = Uuid::new_v4().to_string();
        let tx = connection.begin().await?;
        {
            sqlx::query!(r#"INSERT INTO charge_sessions(`id`, `start_time`, `start_soc`, `current_soc`, `vehicle_id`, `last_update_time`) VALUES (?, NOW(), ?, ?, ?, NOW())"#, 
                uuid, soc, soc, vehicle_id)
                .execute(connection).await?;

            sqlx::query!(
                r#"UPDATE vehicles SET soc = ?, last_update_time = NOW() WHERE id = ?"#,
                soc,
                vehicle_id
            )
            .execute(connection)
            .await?;
        }
        tx.commit().await?;

        let created_charge_session = get_charge_session_by_id(connection, &uuid)
            .await?
            .expect("Failed fetching created Charge session");

        info!(
            "Created charge session [id={}, startTime={}, startSoc={}]",
            created_charge_session.id,
            created_charge_session.start_time,
            created_charge_session.start_soc
        );

        created_charge_session
    };

    Ok(charge_session)
}
