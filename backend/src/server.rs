use crate::{AppState, api};
use axum::Router;
use tower_http::cors::CorsLayer;

pub fn main_router(state: AppState) -> Router {
    Router::new()
        .merge(api::health::router())
        .merge(api::usb::router())
        .merge(api::auth::route())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
