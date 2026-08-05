use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::errors::AppError;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Device {
    pub id: i64,

    pub manufacturer: String,
    pub serial: String,
    pub capacity: String,

    pub assigned_number: String,
    pub registered: bool,
    pub secret: bool,
    pub special: bool,

    pub secclass: Option<String>,
    pub max_secclass: Option<String>,

    pub owner: Option<String>,

    pub register_number: Option<String>,
    pub prescription: Option<String>,
    pub conclusion_number: Option<String>,
    pub zones: Option<String>,

    pub destroyed: bool,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceUpload {
    pub manufacturer: String,
    pub serial: String,
    pub capacity: String,

    pub assigned_number: Option<String>,
    pub registered: bool,
    pub secret: bool,
    pub special: bool,

    pub secclass: Option<String>,
    pub max_secclass: Option<String>,

    pub owner: Option<String>,

    pub register_number: Option<String>,
    pub prescription: Option<String>,
    pub conclusion_number: Option<String>,
    pub zones: Option<String>,

    pub destroyed: bool,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceImport {
    pub manufacturer: String,
    pub capacity: String,
    pub serial: String,

    pub assigned_number: Option<String>,
    pub register_number: Option<String>,
    pub conclusion_number: Option<String>,
    pub prescription: Option<String>,

    pub owner: Option<String>,

    pub secclass: Option<String>,
    pub max_secclass: Option<String>,
    pub zones: Option<String>,

    pub destroyed: bool,
}

impl TryInto<DeviceUpload> for DeviceImport {
    type Error = AppError;
    fn try_into(self) -> Result<DeviceUpload, Self::Error> {
        let secret = [
            &self.secclass,
            &self.max_secclass,
            &self.zones,
            &self.conclusion_number,
            &self.prescription,
        ]
        .iter()
        .any(|field| field.is_some());

        let special = self
            .register_number
            .as_deref()
            .map_or(false, |s| s.contains("-СД"));

        Ok(DeviceUpload {
            manufacturer: self.manufacturer,
            capacity: self.capacity,
            serial: self.serial,
            assigned_number: self.assigned_number,
            // задается явно
            registered: true,
            secret,
            special,
            secclass: self.secclass,
            max_secclass: self.max_secclass,

            owner: self.owner,
            register_number: self.register_number,

            prescription: self.prescription,
            conclusion_number: self.conclusion_number,
            zones: self.zones,

            destroyed: self.destroyed,
            deleted: false,
        })
    }
}

// устройства, полученные от ОС
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UsbDevice {
    pub manufacturer: String,
    pub product: String,
    pub serial: String,
    pub filesystem: Option<String>,
    pub capacity: Option<String>,
    pub vid: Option<u16>,
    pub pid: Option<u16>,
}

// сопоставленные устройства
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MappedDevice {
    pub id: Option<i64>,
    pub manufacturer: Option<String>,
    pub serial: Option<String>,
    pub filesystem: Option<String>,
    pub capacity: Option<String>,

    pub registered: bool,
    pub assigned_number: Option<String>,
    pub owner: Option<String>,
    pub register_number: Option<String>,
    pub conclusion_number: Option<String>,
    pub prescription: Option<String>,

    pub secret: bool,
    pub special: bool,

    pub secclass: Option<String>,
    pub max_secclass: Option<String>,

    pub zones: Option<String>,
    pub connected: bool, // подключены в данный момент
    pub destroyed: bool,
    pub deleted: bool,
}
