use super::utils::map_devices;
use crate::db::devices::get_devices;
use crate::errors::AppError;
use crate::models::device::MappedDevice;
use crate::os::get_history_usb_from_os;
use sqlx::SqlitePool;
use std::thread;
use std::time::Duration;

pub async fn get_history_usb_mapped(pool: &SqlitePool) -> Result<Vec<MappedDevice>, AppError> {
    thread::sleep(Duration::from_millis(100));

    let connected_usb = get_history_usb_from_os().await?;
    let usb_in_db = get_devices(&pool).await?;

    let mapped_devices = map_devices(connected_usb, usb_in_db, true)?;

    Ok(mapped_devices)
}
