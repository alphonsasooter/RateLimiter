use async_trait::async_trait;
use crate::errors::{Result, RateLimiterError};
use crate::store::RedisStore;
use super::{RateLimiter, RuleConfig, RateLimitResult};

pub struct TokenBucket {
    store: RedisStore,
}

impl TokenBucket {
    pub fn new(store: RedisStore) -> Self {
        Self { store }
    }
}

#[async_trait]
impl RateLimiter for TokenBucket {
    async fn check(&self, rule: &RuleConfig) -> Result<RateLimitResult> {
        let capacity = rule.burst.unwrap_or(rule.max_requests);
        let refill_rate = rule.max_requests as f64 / rule.window_secs as f64;

        let bucket_key = format!("tb:{}:{}", rule.key, "tokens");
        let time_key = format!("tb:{}:{}", rule.key, "last_refill");

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let lua_script = r#"
            local tokens_key = KEYS[1]
            local time_key = KEYS[2]
            local capacity = tonumber(ARGV[1])
            local refill_rate = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])

            local tokens = tonumber(redis.call('GET', tokens_key)) or capacity
            local last_refill = tonumber(redis.call('GET', time_key)) or now

            local elapsed = now - last_refill
            tokens = math.min(capacity, tokens + elapsed * refill_rate)

            local allowed = 0
            if tokens >= 1 then
                tokens = tokens - 1
                allowed = 1
            end

            redis.call('SET', tokens_key, tokens, 'EX', ARGV[4])
            redis.call('SET', time_key, now, 'EX', ARGV[4])

            return {allowed, math.floor(tokens)}
        "#;

        let result: Vec<i64> = self.store.run_lua(
            lua_script,
            &[&bucket_key, &time_key],
            &[
                &capacity.to_string(),
                &refill_rate.to_string(),
                &now.to_string(),
                &(rule.window_secs * 2).to_string(),
            ],
        ).await?;

        let allowed = result[0] == 1;
        let remaining = result[1] as u64;

        Ok(RateLimitResult {
            allowed,
            remaining,
            retry_after_secs: if !allowed {
                Some((1.0 / refill_rate).ceil() as u64)
            } else {
                None
            },
        })
    }

    async fn reset(&self, key: &str) -> Result<()> {
        self.store.delete(&format!("tb:{}:tokens", key)).await?;
        self.store.delete(&format!("tb:{}:last_refill", key)).await?;
        Ok(())
    }
}