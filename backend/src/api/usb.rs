use std::fs;

use axum::routing::get;
use axum::{Json, Router};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UsbDevice {
    id: String,
    label: String,
    manufacturer: String,
    serial: String,
    assigned_number: String,
    filesystem: String,
    capacity: String,

    registered: bool,
    secret: bool,
    special: bool,

    secclass: Option<String>,
    maxsecclass: Option<String>,

    owner: Option<String>,
    register_number: Option<u32>,
    prescription: Option<u32>,
    conclusion_number: Option<u32>,
    zones: Option<String>,

    first_seen: String,
    last_seen: String,
}

pub fn router() -> Router {
    Router::new().route("/usb", get(get_devices))
}

async fn get_devices() -> Json<Vec<UsbDevice>> {
    println!("cwd = {:?}", std::env::current_dir().unwrap());
    let content = fs::read_to_string("src/devices.json").expect("read");
    let devices: Vec<UsbDevice> = serde_json::from_str(&content).expect("read");
    Json(devices)
}
