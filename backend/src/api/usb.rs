use std::fs;

use crate::AppState;
use crate::models::device::Device;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/usb", get(get_devices))
}

async fn get_devices(State(pool): State<AppState>) -> Json<Vec<Device>> {
    let content = fs::read_to_string("src/devices.json").expect("read");
    let devices: Vec<Device> = serde_json::from_str(&content).expect("read");
    Json(devices)
}
