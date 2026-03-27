#[cfg(test)]
mod tests {
    use rate_limiter::algorithms::{RuleConfig, RateLimiter};
    use rate_limiter::algorithms::fixed_window::FixedWindow;
    use rate_limiter::store::RedisStore;

    fn get_store() -> RedisStore {
        RedisStore::new("redis://127.0.0.1:6379")
            .expect("Failed to connect to Redis")
    }

    fn make_rule(key: &str, max_requests: u64, window_secs: u64) -> RuleConfig {
        RuleConfig {
            key: key.to_string(),
            max_requests,
            window_secs,
            burst: None,
        }
    }

    /// Allow requests within the window limit
    #[tokio::test]
    async fn test_allows_within_window() {
        let limiter = FixedWindow::new(get_store());
        let rule = make_rule("fw_test_allow", 5, 60);

        limiter.reset(&rule.key).await.unwrap();

        for i in 0..5 {
            let result = limiter.check(&rule).await.unwrap();
            assert!(result.allowed, "Request {} should be allowed", i + 1);
        }
    }

    /// Block requests that exceed the window limit
    #[tokio::test]
    async fn test_blocks_over_limit() {
        let limiter = FixedWindow::new(get_store());
        let rule = make_rule("fw_test_block", 3, 60);

        limiter.reset(&rule.key).await.unwrap();

        for _ in 0..3 {
            limiter.check(&rule).await.unwrap();
        }

        let result = limiter.check(&rule).await.unwrap();
        assert!(!result.allowed, "Should be blocked after exceeding limit");
        assert!(result.retry_after_secs.is_some(), "Should return retry_after");
    }

    /// Remaining count goes to zero at limit
    #[tokio::test]
    async fn test_remaining_reaches_zero() {
        let limiter = FixedWindow::new(get_store());
        let rule = make_rule("fw_test_zero", 3, 60);

        limiter.reset(&rule.key).await.unwrap();

        let mut last_remaining = 0;
        for _ in 0..3 {
            let result = limiter.check(&rule).await.unwrap();
            last_remaining = result.remaining;
        }

        assert_eq!(last_remaining, 0, "Remaining should be 0 at the limit");
    }

    /// retry_after is returned when blocked
    #[tokio::test]
    async fn test_retry_after_on_block() {
        let limiter = FixedWindow::new(get_store());
        let rule = make_rule("fw_test_retry", 1, 60);

        limiter.reset(&rule.key).await.unwrap();

        limiter.check(&rule).await.unwrap(); // consume the 1 allowed request

        let result = limiter.check(&rule).await.unwrap();
        assert!(!result.allowed);
        let retry = result.retry_after_secs.unwrap();
        assert!(retry > 0 && retry <= 60, "retry_after should be within window");
    }

    /// Reset clears the window counter
    #[tokio::test]
    async fn test_reset_clears_counter() {
        let limiter = FixedWindow::new(get_store());
        let rule = make_rule("fw_test_reset", 2, 60);

        limiter.reset(&rule.key).await.unwrap();

        limiter.check(&rule).await.unwrap();
        limiter.check(&rule).await.unwrap();

        let blocked = limiter.check(&rule).await.unwrap();
        assert!(!blocked.allowed, "Should be blocked before reset");

        limiter.reset(&rule.key).await.unwrap();

        let after_reset = limiter.check(&rule).await.unwrap();
        assert!(after_reset.allowed, "Should be allowed after reset");
    }

    /// Two different keys don't interfere
    #[tokio::test]
    async fn test_keys_are_isolated() {
        let limiter = FixedWindow::new(get_store());
        let rule_x = make_rule("fw_test_x", 1, 60);
        let rule_y = make_rule("fw_test_y", 1, 60);

        limiter.reset(&rule_x.key).await.unwrap();
        limiter.reset(&rule_y.key).await.unwrap();

        // Exhaust key_x
        limiter.check(&rule_x).await.unwrap();
        let blocked_x = limiter.check(&rule_x).await.unwrap();
        assert!(!blocked_x.allowed, "key_x should be blocked");

        // key_y should be unaffected
        let allowed_y = limiter.check(&rule_y).await.unwrap();
        assert!(allowed_y.allowed, "key_y should still be allowed");
    }

    /// Exact limit boundary — last allowed request has remaining = 0
    #[tokio::test]
    async fn test_boundary_condition() {
        let limiter = FixedWindow::new(get_store());
        let rule = make_rule("fw_test_boundary", 4, 60);

        limiter.reset(&rule.key).await.unwrap();

        for _ in 0..3 {
            limiter.check(&rule).await.unwrap();
        }

        let last_allowed = limiter.check(&rule).await.unwrap();
        assert!(last_allowed.allowed, "4th request should be allowed");
        assert_eq!(last_allowed.remaining, 0, "Remaining should be 0 on last allowed");

        let over_limit = limiter.check(&rule).await.unwrap();
        assert!(!over_limit.allowed, "5th request should be blocked");
    }
}