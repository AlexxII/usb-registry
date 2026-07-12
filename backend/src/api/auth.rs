use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AuthRequest {
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

pub fn route() -> Router<AppState> {
    Router::new()
        .route("/auth/admin", post(check_admin))
        .route("/auth/admin/password", post(change_password))
}

async fn check_admin(
    State(pool): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    if payload.password != "12345" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(AuthResponse {
        token: "Some token".to_string(),
    }))
}

// смена пароля
async fn change_password(
    State(pool): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}
