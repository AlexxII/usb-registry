use crate::{AppState, api};

use axum::{
    Router,
    body::Body,
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use tower_http::cors::CorsLayer;

#[derive(RustEmbed)]
#[folder = "dist/"] 
struct Assets;

pub fn main_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .merge(api::health::router())
        .merge(api::usb::router())
        .merge(api::auth::route())
        .with_state(state);

    Router::new()
        .merge(api_routes)
        .layer(CorsLayer::permissive())
        .fallback(static_handler)
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    // Если путь пустой (корень), запрашиваем index.html
    let asset_path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(asset_path) {
        Some(content) => {
            let mime = mime_guess::from_path(asset_path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap()
        }
        None => {
            // Важно для Svelte 5 SPA: если файл не найден по этому пути (например, /dashboard),
            // отдаем index.html, чтобы фронтенд-роутер сам обработал этот путь на клиенте.
            match Assets::get("index.html") {
                Some(index) => Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(Body::from(index.data))
                    .unwrap(),
                None => StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}
