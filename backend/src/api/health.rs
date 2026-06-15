use crate::{AppState, db};
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(check_health))
}

async fn check_health(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    db::health::check_health(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("Ok! That's all right!")
}
