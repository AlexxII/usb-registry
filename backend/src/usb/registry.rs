use sqlx::SqlitePool;

use crate::db::devices::get_devices;
use crate::errors::AppError;
use crate::models::device::{Device, MappedDevice};

pub async fn get_usb_from_db(pool: &SqlitePool) -> Result<Vec<MappedDevice>, AppError> {
    let devices = get_devices(&pool).await?;
    let devices: Vec<MappedDevice> = devices.into_iter().map(|d| d.into()).collect();

    Ok(devices)
}

impl From<Device> for MappedDevice {
    fn from(device: Device) -> Self {
        Self {
            id: Some(device.id),
            manufacturer: Some(device.manufacturer),
            serial: Some(device.serial),
            filesystem: None,
            capacity: Some(device.capacity),

            registered: device.registered,
            assigned_number: Some(device.assigned_number),
            owner: device.owner,
            register_number: device.register_number,
            conclusion_number: device.conclusion_number,
            prescription: device.prescription,

            secret: device.secret,
            special: device.special,

            secclass: device.secclass,
            max_secclass: device.max_secclass,

            zones: device.zones,
            connected: false,
            destroyed: device.destroyed,
            deleted: device.deleted,
        }
    }
}
