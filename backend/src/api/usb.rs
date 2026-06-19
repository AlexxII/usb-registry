use crate::AppState;
use crate::db::devices::{delete_device, get_all_devices, get_devices, insert_device, set_destroyed, update_device};
use crate::errors::AppResult;
use crate::models::device::{Device, DeviceUpload};
use std::fs;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/usb/devices", get(list_devices))
        .route("/usb/devices/all", get(list_devices_all))
        .route("/usb/devices", post(create_device))
        .route("/usb/devices", delete(delete_devices))
        .route("/usb/devices/import", post(import_devices))
        .route("/usb/devices/{id}", put(update))
        .route("/usb/devices/{id}", delete(delete_device_from_bd))
        .route("/usb/devices/{id}/destroy", put(mark_destroyed))
}

#[allow(dead_code)]
async fn list_devices_from_file(State(_): State<AppState>) -> Json<Vec<Device>> {
    let content = fs::read_to_string("src/devices.json").expect("read");
    let devices: Vec<Device> = serde_json::from_str(&content).expect("read");
    Json(devices)
}

async fn list_devices(State(state): State<AppState>) -> AppResult<Json<Vec<Device>>> {
    let devices = get_devices(&state.pool).await?;
    Ok(Json(devices))
}

async fn list_devices_all(State(state): State<AppState>) -> AppResult<Json<Vec<Device>>> {
    let devices = get_all_devices(&state.pool).await?;
    Ok(Json(devices))
}

async fn create_device(
    State(state): State<AppState>,
    Json(device): Json<DeviceUpload>,
) -> AppResult<StatusCode> {
    insert_device(&state.pool, &device).await?;
    Ok(StatusCode::CREATED)
}

async fn update(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(device): Json<DeviceUpload>,
) -> AppResult<StatusCode> {
    update_device(&state.pool, id, &device).await?;
    Ok(StatusCode::OK)
}

async fn mark_destroyed(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    set_destroyed(&state.pool, id, true).await?;
    Ok(StatusCode::OK)
}

async fn delete_device_from_bd(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(flag): Json<bool>,
) -> AppResult<StatusCode> {
    delete_device(&state.pool, id, flag).await?;
    Ok(StatusCode::OK)
}

async fn delete_devices(State(state): State<AppState>) {}

async fn import_devices(State(state): State<AppState>, Json(ids): Json<Vec<DeviceUpload>>) {}
