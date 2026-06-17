use std::fs;

use crate::AppState;
use crate::db::devices::{get_devices, insert_device};
use crate::errors::AppResult;
use crate::models::device::{CreateDevice, Device};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/usb/devices", get(list_devices))
        .route("/usb/devices", post(create_device))
        .route("/usb/devices", delete(delete_devices))
        .route("/usb/devices/import", post(import_devices))
        .route("/usb/devices/{id}", put(update_device))
        .route("/usb/devices/{id}", delete(delete_device))
}

// async fn list_devices(State(state): State<AppState>) -> Json<Vec<Device>> {
//     let content = fs::read_to_string("src/devices.json").expect("read");
//     let devices: Vec<Device> = serde_json::from_str(&content).expect("read");
//     Json(devices)
// }

async fn list_devices(State(state): State<AppState>) -> AppResult<Json<Vec<Device>>> {
    let device = get_devices(&state.pool).await?;
    Ok(Json(device))
}

async fn create_device(
    State(state): State<AppState>,
    Json(device): Json<CreateDevice>,
) -> AppResult<StatusCode> {
    insert_device(&state.pool, &device).await?;
    Ok(StatusCode::CREATED)
}

async fn update_device(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(device): Json<Device>,
) -> AppResult<StatusCode> {
    println!("{:?}", device);
    println!("{:?}", id);
    Ok(StatusCode::OK)
}

async fn delete_device(State(state): State<AppState>, Json(id): Json<i64>) {}

async fn delete_devices(State(state): State<AppState>, Json(ids): Json<Vec<i64>>) {}

async fn import_devices(State(state): State<AppState>, Json(ids): Json<Vec<CreateDevice>>) {}
