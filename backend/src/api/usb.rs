use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

pub fn router() -> Router {
    Router::new().route("/usb", get(show_all))
}

async fn show_all() -> Result<Json<Value>, String> {
    Ok(Json(json!({
        "status": "ok",
        "message": "I show you some usb-flashes"
    })))
}
