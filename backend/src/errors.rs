use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Ошибка WinAPI: {0}")]
    WinApi(String),

    #[error("Ошибка реестра Windows: {0}")]
    Registry(String),

    #[error("Ошибка сопоставления данных: {0}")]
    Mapping(String),
}

pub enum AppError {
    Validation(String),
    BadRequest(String),
    BatchValidation(Vec<BatchErrorItem>),
    NotFound,
    Database(sqlx::Error),
    Device(DeviceError),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct BatchErrorResponse {
    error: String,
    errors: Vec<BatchErrorItem>,
}

#[derive(Serialize)]
pub struct BatchErrorItem {
    pub line: usize,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Validation(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: message }),
            )
                .into_response(),
            AppError::BadRequest(error) => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse { error })).into_response()
            }
            AppError::BatchValidation(errors) => {
                let response = BatchErrorResponse {
                    error: format!("Импорт не завершён: {} ошибок", errors.len()),
                    errors,
                };
                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            }
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Not found".into(),
                }),
            )
                .into_response(),

            AppError::Device(err) => {
                tracing::error!("Device subsystem error occurred: {:?}", err);
                let message = err.to_string();

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse { error: message }),
                )
                    .into_response()
            }

            AppError::Database(err) => {
                tracing::error!("Database error occurred: {:?}", err);

                let (status, message) = match &err {
                    sqlx::Error::RowNotFound => (
                        StatusCode::NOT_FOUND,
                        "Requested resource was not found".to_string(),
                    ),
                    sqlx::Error::Database(db_err) => {
                        if db_err.is_unique_violation() {
                            (
                                StatusCode::CONFLICT,
                                "Resource with this data already exists".to_string(),
                            )
                        } else if db_err.is_foreign_key_violation() {
                            (
                                StatusCode::BAD_REQUEST,
                                "Related record does not exist".to_string(),
                            )
                        } else {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Database operation failed".to_string(),
                            )
                        }
                    }
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    ),
                };
                (status, Json(ErrorResponse { error: message })).into_response()
            }
        }
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::BadRequest(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<DeviceError> for AppError {
    fn from(err: DeviceError) -> Self {
        AppError::Device(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;
