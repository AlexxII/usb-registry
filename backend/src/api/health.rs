use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

pub fn router() -> Router {
    Router::new().route("/health", get(check_health))
}

async fn check_health() -> Result<Json<Value>, String> {
    Ok(Json(json!({
        "status": "Ok",
        "message": "Server health if Ok! Working bro!"
    })))
}
