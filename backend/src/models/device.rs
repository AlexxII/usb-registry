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
        Ok(DeviceUpload {
            manufacturer: self.manufacturer,
            capacity: self.capacity,
            serial: self.serial,
            assigned_number: self.assigned_number,
            // задается явно
            registered: true,

            // TODO!
            secret: is_secret(
                &self.secclass,
                &self.max_secclass,
                &self.zones,
                &self.conclusion_number,
                &self.prescription,
            ),
            special: is_special(&self.register_number),
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

// если заполнено хоть одно поле, то носитель для ГТ
fn is_secret(
    secclass: &Option<String>,
    max_secclass: &Option<String>,
    zones: &Option<String>,
    conclusion_number: &Option<String>,
    prescription: &Option<String>,
) -> bool {
    if secclass.is_some() {
        return true;
    }
    if max_secclass.is_some() {
        return true;
    }
    if zones.is_some() {
        return true;
    }
    if conclusion_number.is_some() {
        return true;
    }
    if prescription.is_some() {
        return true;
    }
    false
}

// если строка рег.номера содержит -СД, то это носитель для СД
fn is_special(register_number: &Option<String>) -> bool {
    register_number
        .as_deref()
        .map_or(false, |s| s.contains("-СД"))
}
