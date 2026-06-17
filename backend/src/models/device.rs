use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDevice {
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
}
