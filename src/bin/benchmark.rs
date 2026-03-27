use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let url = "http://localhost:8080/check";
    let total_requests = 1000;
    let concurrency = 50;

    println!("Starting benchmark...");
    println!("Total requests : {}", total_requests);
    println!("Concurrency    : {}", concurrency);
    println!("Target         : {}", url);
    println!("────────────────────────────────");

    let client = Arc::new(reqwest::Client::new());
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut handles = vec![];

    let start = Instant::now();

    for i in 0..total_requests {
        let client = client.clone();
        let sem = semaphore.clone();
        let url = url.to_string();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let req_start = Instant::now();

            let body = serde_json::json!({
                "key": format!("bench_user_{}", i % 100),
                "max_requests": 1000,
                "window_secs": 60
            });

            let result = client
                .post(&url)
                .json(&body)
                .send()
                .await;

            let latency = req_start.elapsed();

            match result {
                Ok(resp) => (true, resp.status().as_u16(), latency),
                Err(_)   => (false, 0u16, latency),
            }
        });

        handles.push(handle);
    }

    let mut success = 0;
    let mut failed = 0;
    let mut total_latency = Duration::ZERO;
    let mut max_latency = Duration::ZERO;
    let mut min_latency = Duration::MAX;

    for handle in handles {
        if let Ok((ok, _status, latency)) = handle.await {
            if ok { success += 1; } else { failed += 1; }
            total_latency += latency;
            if latency > max_latency { max_latency = latency; }
            if latency < min_latency { min_latency = latency; }
        }
    }

    let elapsed = start.elapsed();
    let avg_latency = total_latency / total_requests as u32;
    let rps = total_requests as f64 / elapsed.as_secs_f64();

    println!("────────────────────────────────");
    println!("Results:");
    println!("  Total time   : {:.2}s",     elapsed.as_secs_f64());
    println!("  Requests/sec : {:.0}",       rps);
    println!("  Success      : {}",           success);
    println!("  Failed       : {}",           failed);
    println!("  Avg latency  : {:.2}ms",     avg_latency.as_secs_f64() * 1000.0);
    println!("  Min latency  : {:.2}ms",     min_latency.as_secs_f64() * 1000.0);
    println!("  Max latency  : {:.2}ms",     max_latency.as_secs_f64() * 1000.0);

    if avg_latency.as_millis() < 1 {
        println!("\n✅ Goal achieved: avg latency < 1ms");
    } else {
        println!("\n⚠️  Avg latency above 1ms target");
    }
}