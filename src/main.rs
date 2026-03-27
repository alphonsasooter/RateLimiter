use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber;

mod algorithms;
mod api;
mod config;
mod errors;
mod store;

use algorithms::token_bucket::TokenBucket;
use api::routes::{
    check_rate_limit, create_rule, delete_rule, health_check,
    list_rules, reset_key, AppState,
};
use config::AppConfig;
use store::RedisStore;

#[tokio::main]
async fn main() {
    // Init logging
    tracing_subscriber::fmt::init();

    // Load config
    let config = AppConfig::default();

    // Connect to Redis
    let store = RedisStore::new(&config.redis_url)
        .expect("Failed to connect to Redis");

    // Token Bucket limiter
    let limiter = Arc::new(TokenBucket::new(store));

    // Shared state
    let state = AppState {
        limiter,
        rules: Arc::new(RwLock::new(vec![])),
    };

    // Router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/check", post(check_rate_limit))
        .route("/rules", post(create_rule))
        .route("/rules", get(list_rules))
        .route("/rules/:id", delete(delete_rule))
        .route("/reset/:key", post(reset_key))
        .with_state(state);

    // 🔥 FIX: Change port from 8080 → 3000
    let addr = format!("127.0.0.1:{}", 3000);

    tracing::info!("Rate Limiter listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}