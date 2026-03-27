use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use crate::algorithms::{RuleConfig, RateLimiter};

#[derive(Clone)]
pub struct RateLimitState {
    pub limiter: Arc<dyn RateLimiter>,
    pub max_requests: u64,
    pub window_secs: u64,
}

pub async fn rate_limit_middleware(
    State(state): State<RateLimitState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    // Extract IP from request headers
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let rule = RuleConfig {
        key: format!("ip:{}", ip),
        max_requests: state.max_requests,
        window_secs: state.window_secs,
        burst: None,
    };

    match state.limiter.check(&rule).await {
        Ok(result) if result.allowed => {
            let mut response = next.run(req).await;
            response.headers_mut().insert(
                "X-RateLimit-Remaining",
                result.remaining.to_string().parse().unwrap(),
            );
            response
        }
        Ok(result) => {
            let retry_after = result.retry_after_secs.unwrap_or(60);
            (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": "Rate limit exceeded",
                    "retry_after_secs": retry_after
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}