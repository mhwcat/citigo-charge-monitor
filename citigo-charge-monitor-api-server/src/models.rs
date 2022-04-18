use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: String,
    pub vin: String,
    pub soc: Option<i8>,
    pub target_soc: i8,
    pub last_update_time: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVehicle {
    pub id: Option<String>,
    pub vin: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeSession {
    pub id: String,
    pub start_time: NaiveDateTime,
    pub stop_time: Option<NaiveDateTime>,
    pub start_soc: i8,
    pub current_soc: Option<i8>,
    pub stop_soc: Option<i8>,
    pub vehicle_id: String,
    pub last_update_time: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChargeSessionsForVehicle {
    pub vehicle_id: String,
    pub index: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateChargeSession {
    pub soc: i8,
    pub vehicle_id: String,
    pub stop: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleStatus {
    pub is_charging: bool,
    pub vehicle: Vehicle,
    pub current_charge_session: Option<ChargeSession>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVehicleStatus {
    pub id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentChargeSessions {
    pub charge_sessions: Vec<ChargeSession>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVehicle {
    pub vin: String,
    pub target_soc: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVehicle {
    pub id: String,
    pub soc: Option<i8>,
    pub target_soc: Option<i8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChargeSession {
    pub id: String,
    pub stop_time: Option<NaiveDateTime>,
    pub current_soc: Option<u8>,
    pub stop_soc: Option<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub creation_time: NaiveDateTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUser {
    pub id: String,
    pub username: String,
    pub creation_time: NaiveDateTime,
}

impl From<User> for SimpleUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            creation_time: user.creation_time,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub creation_time: NaiveDateTime,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutUser {
    pub user_session_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}
