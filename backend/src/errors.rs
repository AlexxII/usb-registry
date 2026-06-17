use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Validation(String),
    NotFound,
    Database(sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Validation(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: message }),
            )
                .into_response(),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Not found".into(),
                }),
            )
                .into_response(),

            AppError::Database(err) => {
                tracing::error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".into(),
                    }),
                )
                    .into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;
