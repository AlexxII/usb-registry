use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(check_health))
}

async fn check_health(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            eprintln!("Health check failed: DATABASE error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok("OK")
}
