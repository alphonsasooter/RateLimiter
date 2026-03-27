use redis::{AsyncCommands, Client, Script};
use crate::errors::{Result, RateLimiterError};

#[derive(Clone)]
pub struct RedisStore {
    client: Client,
}

impl RedisStore {
    pub fn new(redis_url: &str) -> Result<Self> {
        let client = Client::open(redis_url)
            .map_err(RateLimiterError::RedisError)?;
        Ok(Self { client })
    }

    pub async fn run_lua(
        &self,
        script: &str,
        keys: &[&str],
        args: &[&str],
    ) -> Result<Vec<i64>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let script = Script::new(script);
        let mut invocation = script.prepare_invoke();

        for key in keys {
            invocation.key(*key);
        }

        for arg in args {
            invocation.arg(*arg);
        }

        let result: Vec<i64> = invocation.invoke_async(&mut conn).await?;
        Ok(result)
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.del(key).await?;
        Ok(())
    }

    pub async fn delete_pattern(&self, pattern: &str) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let keys: Vec<String> = conn.keys(pattern).await?;

        if !keys.is_empty() {
            let _: () = conn.del(keys).await?;
        }

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = conn.get(key).await?;
        Ok(result)
    }

    pub async fn set_with_expiry(&self, key: &str, value: &str, ttl_secs: u64) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.set_ex(key, value, ttl_secs).await?;
        Ok(())
    }
}