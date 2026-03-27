use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

// replace your existing app definition with this:
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::DELETE])
    .allow_headers(Any);

let app = Router::new()
    .route("/health",      get(health_check))
    .route("/check",       post(check_rate_limit))
    .route("/rules",       post(create_rule))
    .route("/rules",       get(list_rules))
    .route("/rules/:id",   delete(delete_rule))
    .route("/reset/:key",  post(reset_key))
    .layer(cors)
    .with_state(state);