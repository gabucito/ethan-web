use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;

use crate::logging::init_logging;
use crate::cache::CacheManager;
use crate::models::AppState;
use crate::handlers::*;

mod models;
mod handlers;
mod logging;
mod cache;

const STATIC_PATH: &str = concat!(env!("OUT_DIR"), "/static");

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Initialize cache
    let cache_manager = CacheManager::new();

    // Get the static files directory relative to the executable
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let static_path = exe_dir.join("static");

    // Initialize app state
    let app_state = Arc::new(AppState {
        cache_manager: Arc::new(cache_manager),
        visits: Arc::new(TokioMutex::new(Vec::new())),
        version: Arc::new(Mutex::new(1)),
    });

    // Build the router
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/resume", get(resume_handler))
        .route("/resume2", get(resume2_handler))
        .nest_service("/static", ServeDir::new(STATIC_PATH))
        .with_state(app_state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
