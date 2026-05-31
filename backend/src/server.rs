use crate::api;
use axum::Router;

pub fn main_router() -> Router {
    Router::new()
        .merge(api::health::router())
        .merge(api::usb::router())
        .merge(api::testing::router())
}
