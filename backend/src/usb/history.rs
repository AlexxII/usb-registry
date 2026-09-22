use std::fs::File;
use std::thread;
use std::time::Duration;

use sqlx::SqlitePool;

use crate::db::devices::get_devices;
use crate::errors::AppError;
use crate::models::device::{MappedDevice, UsbDevice};

use super::utils::map_devices;

pub async fn get_history_usb() -> Result<Vec<UsbDevice>, AppError> {
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

pub async fn get_history_usb_from_os() -> Result<Vec<UsbDevice>, AppError> {
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

pub async fn get_history_usb_mapped(pool: &SqlitePool) -> Result<Vec<MappedDevice>, AppError> {
    thread::sleep(Duration::from_millis(500));

    let connected_usb = get_history_usb_from_os().await?;
    let usb_in_db = get_devices(&pool).await?;

    let mapped_devices = map_devices(connected_usb, usb_in_db, true)?;

    Ok(mapped_devices)
}
