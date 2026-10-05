use std::thread;
use std::time::Duration;

use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::AppState;
use crate::db::devices::get_devices;
use crate::errors::{AppError, AppResult};
use crate::models::device::MappedDevice;
use crate::os::get_current_usb_flash_drives;
use crate::usb::utils::map_devices;

pub async fn get_current_usb_mapped(pool: &SqlitePool) -> Result<Vec<MappedDevice>, AppError> {
    thread::sleep(Duration::from_millis(200));

    let connected_usb = get_current_usb_flash_drives().await?;
    let usb_in_db = get_devices(&pool).await?;

    let mapped_devices = map_devices(connected_usb, usb_in_db, true)?;

    Ok(mapped_devices)
}

pub async fn get_current_usb_mapped_e(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<MappedDevice>>> {
    let connected_usb = get_current_usb_flash_drives().await?;
    let usb_in_db = get_devices(&state.pool).await?;

    let mapped_devices = map_devices(connected_usb, usb_in_db, true)?;

    Ok(Json(mapped_devices))
}
