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

pub fn route() -> Router {
    Router::new().route("/auth/admin", post(check_admin))
}

async fn check_admin(Json(payload): Json<AuthRequest>) -> Result<Json<AuthResponse>, StatusCode> {
    if payload.password != "12345" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(AuthResponse {
        token: "Some token".to_string(),
    }))
}
