use crate::api;
use axum::Router;
use tower_http::cors::CorsLayer;

pub fn main_router() -> Router {
    Router::new()
        .merge(api::health::router())
        .merge(api::usb::router())
        .merge(api::auth::route())
        .layer(CorsLayer::permissive())
}
