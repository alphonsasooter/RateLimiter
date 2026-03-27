use async_trait::async_trait;
use crate::errors::Result;

#[derive(Debug, Clone)]
pub struct RuleConfig {
    pub key: String,
    pub max_requests: u64,
    pub window_secs: u64,
    pub burst: Option<u64>,
}

#[derive(Debug)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: u64,
    pub retry_after_secs: Option<u64>,
}

#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn check(&self, rule: &RuleConfig) -> Result<RateLimitResult>;
    async fn reset(&self, key: &str) -> Result<()>;
}

pub mod token_bucket;
pub mod fixed_window;