use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use crate::algorithms::{RuleConfig, RateLimiter};

// ── Shared app state ──────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AppState {
    pub limiter: Arc<dyn RateLimiter>,
    pub rules: Arc<tokio::sync::RwLock<Vec<Rule>>>,
}

// ── Rule model ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub key: String,
    pub max_requests: u64,
    pub window_secs: u64,
    pub burst: Option<u64>,
    pub algorithm: String,
}

// ── Request / Response bodies ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CheckRequest {
    pub key: String,
    pub max_requests: u64,
    pub window_secs: u64,
    pub burst: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct CheckResponse {
    pub allowed: bool,
    pub remaining: u64,
    pub retry_after_secs: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRuleRequest {
    pub key: String,
    pub max_requests: u64,
    pub window_secs: u64,
    pub burst: Option<u64>,
    pub algorithm: String,  // "token_bucket" | "fixed_window"
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// POST /check
/// Check if a request is allowed under the given rule
pub async fn check_rate_limit(
    State(state): State<AppState>,
    Json(payload): Json<CheckRequest>,
) -> Result<Json<CheckResponse>, (StatusCode, Json<Value>)> {
    let rule = RuleConfig {
        key: payload.key,
        max_requests: payload.max_requests,
        window_secs: payload.window_secs,
        burst: payload.burst,
    };

    match state.limiter.check(&rule).await {
        Ok(result) => Ok(Json(CheckResponse {
            allowed: result.allowed,
            remaining: result.remaining,
            retry_after_secs: result.retry_after_secs,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

/// POST /rules
/// Create a new rate limit rule
pub async fn create_rule(
    State(state): State<AppState>,
    Json(payload): Json<CreateRuleRequest>,
) -> Result<Json<Rule>, (StatusCode, Json<Value>)> {
    if payload.algorithm != "token_bucket" && payload.algorithm != "fixed_window" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "algorithm must be 'token_bucket' or 'fixed_window'" })),
        ));
    }

    let rule = Rule {
        id: Uuid::new_v4().to_string(),
        key: payload.key,
        max_requests: payload.max_requests,
        window_secs: payload.window_secs,
        burst: payload.burst,
        algorithm: payload.algorithm,
    };

    state.rules.write().await.push(rule.clone());
    Ok(Json(rule))
}

/// GET /rules
/// List all rules
pub async fn list_rules(
    State(state): State<AppState>,
) -> Json<Value> {
    let rules = state.rules.read().await;
    Json(json!({ "rules": *rules, "count": rules.len() }))
}

/// DELETE /rules/:id
/// Delete a rule by ID
pub async fn delete_rule(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mut rules = state.rules.write().await;
    let initial_len = rules.len();
    rules.retain(|r| r.id != id);

    if rules.len() < initial_len {
        Ok(Json(json!({ "message": "Rule deleted", "id": id })))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("Rule {} not found", id) })),
        ))
    }
}

/// POST /reset/:key
/// Reset counters for a given key
pub async fn reset_key(
    State(state): State<AppState>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match state.limiter.reset(&key).await {
        Ok(_) => Ok(Json(json!({ "message": format!("Reset successful for key: {}", key) }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

/// GET /health
/// Health check endpoint
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}