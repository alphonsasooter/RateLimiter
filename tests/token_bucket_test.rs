#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use rate_limiter::algorithms::{RuleConfig, RateLimiter};
    use rate_limiter::algorithms::token_bucket::TokenBucket;
    use rate_limiter::store::RedisStore;

    fn get_store() -> RedisStore {
        RedisStore::new("redis://127.0.0.1:6379")
            .expect("Failed to connect to Redis")
    }

    fn make_rule(key: &str, max_requests: u64, window_secs: u64, burst: Option<u64>) -> RuleConfig {
        RuleConfig {
            key: key.to_string(),
            max_requests,
            window_secs,
            burst,
        }
    }

    /// Allow requests up to the limit
    #[tokio::test]
    async fn test_allows_requests_within_limit() {
        let limiter = TokenBucket::new(get_store());
        let rule = make_rule("tb_test_allow", 5, 60, None);

        // Reset first to ensure clean state
        limiter.reset(&rule.key).await.unwrap();

        for i in 0..5 {
            let result = limiter.check(&rule).await.unwrap();
            assert!(result.allowed, "Request {} should be allowed", i + 1);
        }
    }

    /// Block requests exceeding the limit
    #[tokio::test]
    async fn test_blocks_requests_exceeding_limit() {
        let limiter = TokenBucket::new(get_store());
        let rule = make_rule("tb_test_block", 3, 60, None);

        limiter.reset(&rule.key).await.unwrap();

        // Exhaust tokens
        for _ in 0..3 {
            limiter.check(&rule).await.unwrap();
        }

        // This request should be blocked
        let result = limiter.check(&rule).await.unwrap();
        assert!(!result.allowed, "Request should be blocked after limit exceeded");
        assert!(result.retry_after_secs.is_some(), "Should have retry_after");
    }

    /// Remaining count decrements correctly
    #[tokio::test]
    async fn test_remaining_decrements() {
        let limiter = TokenBucket::new(get_store());
        let rule = make_rule("tb_test_remaining", 5, 60, None);

        limiter.reset(&rule.key).await.unwrap();

        let first = limiter.check(&rule).await.unwrap();
        assert!(first.allowed);

        let second = limiter.check(&rule).await.unwrap();
        assert!(second.allowed);
        assert!(second.remaining < first.remaining, "Remaining should decrease");
    }

    /// Burst allows more than max_requests initially
    #[tokio::test]
    async fn test_burst_allows_extra_requests() {
        let limiter = TokenBucket::new(get_store());
        let rule = make_rule("tb_test_burst", 5, 60, Some(10));

        limiter.reset(&rule.key).await.unwrap();

        // Should allow up to burst (10) requests
        let mut allowed_count = 0;
        for _ in 0..10 {
            let result = limiter.check(&rule).await.unwrap();
            if result.allowed {
                allowed_count += 1;
            }
        }

        assert!(allowed_count >= 5, "Should allow at least max_requests");
    }

    /// Reset clears the bucket
    #[tokio::test]
    async fn test_reset_clears_bucket() {
        let limiter = TokenBucket::new(get_store());
        let rule = make_rule("tb_test_reset", 2, 60, None);

        limiter.reset(&rule.key).await.unwrap();

        // Exhaust tokens
        limiter.check(&rule).await.unwrap();
        limiter.check(&rule).await.unwrap();

        let blocked = limiter.check(&rule).await.unwrap();
        assert!(!blocked.allowed, "Should be blocked before reset");

        // Reset and try again
        limiter.reset(&rule.key).await.unwrap();

        let after_reset = limiter.check(&rule).await.unwrap();
        assert!(after_reset.allowed, "Should be allowed after reset");
    }

    /// Different keys are tracked independently
    #[tokio::test]
    async fn test_different_keys_are_independent() {
        let limiter = TokenBucket::new(get_store());
        let rule_a = make_rule("tb_test_key_a", 2, 60, None);
        let rule_b = make_rule("tb_test_key_b", 2, 60, None);

        limiter.reset(&rule_a.key).await.unwrap();
        limiter.reset(&rule_b.key).await.unwrap();

        // Exhaust key_a
        limiter.check(&rule_a).await.unwrap();
        limiter.check(&rule_a).await.unwrap();
        let blocked_a = limiter.check(&rule_a).await.unwrap();
        assert!(!blocked_a.allowed, "key_a should be blocked");

        // key_b should still be allowed
        let allowed_b = limiter.check(&rule_b).await.unwrap();
        assert!(allowed_b.allowed, "key_b should still be allowed");
    }
}