use async_trait::async_trait;
use crate::errors::Result;
use crate::store::RedisStore;
use super::{RateLimiter, RuleConfig, RateLimitResult};

pub struct FixedWindow {
    store: RedisStore,
}

impl FixedWindow {
    pub fn new(store: RedisStore) -> Self {
        Self { store }
    }
}

#[async_trait]
impl RateLimiter for FixedWindow {
    async fn check(&self, rule: &RuleConfig) -> Result<RateLimitResult> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let window_id = now / rule.window_secs;
        let redis_key = format!("fw:{}:{}", rule.key, window_id);

        let window_ends_at = (window_id + 1) * rule.window_secs;
        let ttl = window_ends_at - now;

        let lua_script = r#"
            local key = KEYS[1]
            local limit = tonumber(ARGV[1])
            local ttl = tonumber(ARGV[2])

            local count = redis.call('INCR', key)
            if count == 1 then
                redis.call('EXPIRE', key, ttl)
            end

            if count > limit then
                return {0, 0, ttl}
            else
                return {1, limit - count, ttl}
            end
        "#;

        let result: Vec<i64> = self.store.run_lua(
            lua_script,
            &[&redis_key],
            &[&rule.max_requests.to_string(), &ttl.to_string()],
        ).await?;

        let allowed = result[0] == 1;
        let remaining = result[1] as u64;
        let retry_after = result[2] as u64;

        Ok(RateLimitResult {
            allowed,
            remaining,
            retry_after_secs: if !allowed { Some(retry_after) } else { None },
        })
    }

    async fn reset(&self, key: &str) -> Result<()> {
        self.store.delete_pattern(&format!("fw:{}:*", key)).await?;
        Ok(())
    }
}