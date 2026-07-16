use crate::AppState;
use crate::db::devices::{
    delete_device, force_delete_device, get_all_devices, get_devices, insert_device, set_destroyed,
    update_device,
};
use crate::errors::{AppError, AppResult, BatchErrorItem};
use crate::models::device::{Device, DeviceImport, DeviceUpload};
use std::{fs, usize};

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use serde_json::json;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/usb/devices", get(list_devices))
        .route("/usb/devices/all", get(list_devices_all))
        .route("/usb/devices", post(create_device))
        .route("/usb/devices", delete(delete_devices))
        .route("/usb/devices/import", post(import_devices_ex))
        .route("/usb/devices/{id}", put(update))
        .route("/usb/devices/{id}", delete(delete_device_from_bd))
        .route("/usb/devices/{id}/force", delete(force_delet_device))
        .route("/usb/devices/{id}/destroy", put(mark_destroyed))
        .route("/usb/devices/{id}/undestroy", put(unmark_destroyed))
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

async fn unmark_destroyed(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    set_destroyed(&state.pool, id, false).await?;
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

async fn force_delet_device(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    force_delete_device(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

async fn delete_devices(State(state): State<AppState>) {}

#[derive(serde::Serialize)]
struct ImportResponse {
    success_count: usize,
    error_count: usize,
    errors: Vec<BatchErrorItem>,
}

async fn import_devices_ex(
    State(state): State<AppState>,
    Json(devices): Json<String>,
) -> AppResult<(StatusCode, Json<ImportResponse>)> {
    let mut successes: Vec<(usize, DeviceUpload)> = Vec::new();
    let mut errors: Vec<BatchErrorItem> = Vec::new();

    for (line_no, line) in devices.lines().enumerate() {
        let line_idx = line_no + 1;
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(";").collect();
        if parts.len() != 12 {
            errors.push(BatchErrorItem {
                line: line_idx,
                message: format!("Неверное количество колонок (ожидалось 12): {}", line),
            });
            continue;
        }

        fn opt_str(s: &str) -> Option<String> {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        }
        fn bool_str(s: &str) -> bool {
            if s == "false" { false } else { true }
        }
        let device_import = DeviceImport {
            manufacturer: parts[0].to_string(),
            capacity: parts[1].to_string(),
            serial: parts[2].to_string(),
            assigned_number: opt_str(parts[3]),
            register_number: opt_str(parts[4]),
            conclusion_number: opt_str(parts[5]),
            prescription: opt_str(parts[6]),
            owner: opt_str(parts[7]),
            secclass: opt_str(parts[8]),
            max_secclass: opt_str(parts[9]),
            zones: opt_str(parts[10]),
            destroyed: bool_str(parts[11]),
        };

        match device_import.try_into() {
            Ok(device_upload) => successes.push((line_idx, device_upload)),
            Err(e) => {
                let message = match &e {
                    AppError::Validation(m) => m.as_str(),
                    _ => "Неизвестная ошибка преобразования",
                };
                errors.push(BatchErrorItem {
                    line: line_idx,
                    message: message.to_string(),
                });
            }
        }
    }

    let success_count = successes.len();
    let error_count = errors.len();
    let mut actual_successes = 0;

    for (line_idx, device) in successes {
        // Передаем ссылку на пул &state.pool и ссылку на устройство &device
        match insert_device(&state.pool, &device).await {
            Ok(_) => {
                actual_successes += 1;
            }
            Err(sqlx_err) => {
                tracing::error!("Ошибка вставки на строке {}: {:?}", line_idx, sqlx_err);

                let friendly_message = match &sqlx_err {
                    sqlx::Error::Database(db_err) => {
                        if db_err.is_unique_violation() {
                            format!(
                                "Устройство с серийным номером '{}' уже существует в системе",
                                device.serial
                            )
                        } else if db_err.is_foreign_key_violation() {
                            "Нарушена связь с другой таблицей (неверный ID)".to_string()
                        } else {
                            format!("Ошибка базы данных: {}", db_err.message())
                        }
                    }
                    sqlx::Error::RowNotFound => "Запись для обновления не найдена".to_string(),
                    _ => "Непредвиденная ошибка при сохранении в базу".to_string(),
                };

                errors.push(BatchErrorItem {
                    line: line_idx,
                    message: friendly_message,
                });
            }
        }
    }

    let response = ImportResponse {
        success_count: actual_successes,
        error_count: errors.len(),
        errors,
    };

    let status = if success_count == 0 && error_count > 0 {
        StatusCode::BAD_REQUEST
    } else {
        StatusCode::OK
    };

    Ok((status, Json(response)))
}
