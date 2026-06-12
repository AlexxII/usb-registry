use std::fs;

use crate::models::device::UsbDevice;

use axum::routing::get;
use axum::{Json, Router};

pub fn router() -> Router {
    Router::new().route("/usb", get(get_devices))
}

async fn get_devices() -> Json<Vec<UsbDevice>> {
    let content = fs::read_to_string("src/devices.json").expect("read");
    let devices: Vec<UsbDevice> = serde_json::from_str(&content).expect("read");
    Json(devices)
}
