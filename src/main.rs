use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tower_http::services::ServeDir;

use crate::cache::CacheManager;
use crate::handlers::*;
use crate::logging::init_logging;
use crate::models::AppState;

mod cache;
mod handlers;
mod logging;
mod models;

const STATIC_PATH: &str = concat!(env!("OUT_DIR"), "/static");

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Initialize cache
    let cache_manager = CacheManager::new();

    // Initialize app state
    let app_state = Arc::new(AppState {
        cache_manager: Arc::new(cache_manager),
        visits: Arc::new(TokioMutex::new(Vec::new())),
    });

    // Build the router
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/resume", get(resume_handler))
        .route("/resume2", get(resume2_handler))
        .route("/resume3", get(handlers::resume3_handler))
        .route("/media", get(media_handler))
        .route("/media/gallery", get(media_gallery_handler))
        .route("/media3", get(handlers::media3_handler))
        .route("/media/storyline", get(media_storyline_handler))
        .route("/media/experiences", get(media_experiences_handler))
        .nest_service("/static", ServeDir::new(STATIC_PATH))
        .with_state(app_state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address 0.0.0.0:3000");
    tracing::info!("Server running on http://0.0.0.0:3000");
    if let Err(e) = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    {
        tracing::error!("Server error: {}", e);
    }
}
