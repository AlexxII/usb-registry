use std::fs::File;
use std::thread;
use std::time::Duration;

use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::AppState;
use crate::db::devices::get_devices;
use crate::errors::{AppError, AppResult};
use crate::models::device::{MappedDevice, UsbDevice};
use crate::usb::utils::map_devices;

pub async fn get_current_usb_from_os() -> Result<Vec<UsbDevice>, AppError> {
    let file = File::open("./usb.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(file);
    let mut result = vec![];
    for r in rdr.deserialize() {
        let record: UsbDevice = r?;
        result.push(record)
    }
    Ok(result)
}

pub async fn get_current_usb_mapped(pool: &SqlitePool) -> Result<Vec<MappedDevice>, AppError> {
    thread::sleep(Duration::from_millis(500));

    let connected_usb = get_current_usb_from_os().await?;
    let usb_in_db = get_devices(&pool).await?;

    let mapped_devices = map_devices(connected_usb, usb_in_db, true)?;

    Ok(mapped_devices)
}

pub async fn get_current_usb_mapped_e(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<MappedDevice>>> {
    let connected_usb = get_current_usb_from_os().await?;
    let usb_in_db = get_devices(&state.pool).await?;

    let mapped_devices = map_devices(connected_usb, usb_in_db, true)?;

    Ok(Json(mapped_devices))
}
