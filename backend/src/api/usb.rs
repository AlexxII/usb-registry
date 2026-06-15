use std::fs;

use crate::AppState;
use crate::models::device::{CreateDevice, Device};

use axum::extract::State;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("usb/devices", get(list_devices))
        .route("usb/devices", post(create_device))
        .route("usb/devices", delete(delete_devices))
        .route("usb/devices/import", post(import_devices))
        .route("usb/devices/{id}", put(update_device))
        .route("usb/devices/{id}", delete(delete_device))
}

async fn list_devices(State(state): State<AppState>) -> Json<Vec<Device>> {
    let content = fs::read_to_string("src/devices.json").expect("read");
    let devices: Vec<Device> = serde_json::from_str(&content).expect("read");
    Json(devices)
}

async fn create_device(State(state): State<AppState>) {}

async fn delete_devices(State(state): State<AppState>, Json(ids): Json<Vec<i64>>) {}

async fn import_devices(State(state): State<AppState>, Json(ids): Json<Vec<CreateDevice>>) {}

async fn update_device(State(state): State<AppState>, Json(id): Json<i64>) {}
async fn delete_device(State(state): State<AppState>, Json(id): Json<i64>) {}
