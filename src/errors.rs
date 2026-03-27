use thiserror::Error;

#[derive(Error, Debug)]
pub enum RateLimiterError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Rate limit exceeded for key: {0}")]
    RateLimitExceeded(String),

    #[error("Rule not found: {0}")]
    RuleNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

pub type Result<T> = std::result::Result<T, RateLimiterError>;