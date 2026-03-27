#  Rate Limiter

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.76+-CE422B?style=for-the-badge&logo=rust&logoColor=white)
![Redis](https://img.shields.io/badge/Redis-7.0-DC382D?style=for-the-badge&logo=redis&logoColor=white)
![React](https://img.shields.io/badge/React-18-61DAFB?style=for-the-badge&logo=react&logoColor=black)
![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?style=for-the-badge&logo=docker&logoColor=white)

**A high-performance, distributed API rate limiter built with Rust and Redis.**  
Sub-millisecond latency · Atomic Lua scripts · React Admin Dashboard

[Getting Started](#-getting-started) · [API Reference](#-api-reference) · [Benchmarks](#-benchmarks) · [Dashboard](#-dashboard)

</div>

---

## 📖 Overview

Rate Limiter is a production-ready rate limiting service that protects your APIs from abuse, DDoS attacks, and runaway clients. Built in Rust for maximum performance, it uses Redis atomic Lua scripts to guarantee correctness across distributed deployments.

```
Client Request → Rate Limiter Service → ✅ Allow / ❌ Deny → Upstream API
                        ↕
                      Redis
                 (atomic counters)
```

---

## ✨ Features

- 🚀 **Sub-millisecond latency** — avg response time under 1ms at 1000+ req/s
- 🔒 **Race-condition safe** — Redis Lua scripts guarantee atomic operations
- 🌐 **Distributed** — consistent limits across multiple instances
- 🧠 **Two algorithms** — Token Bucket and Fixed Window
- 📊 **Admin Dashboard** — live traffic chart, rule management, manual checker
- 🐳 **Docker ready** — single command to run everything
- 🧪 **Fully tested** — 13 integration tests across both algorithms

---

## 🧠 Algorithms

### Token Bucket
Tokens are added at a fixed rate up to a max capacity. Each request consumes one token. Supports **burst** — allows short spikes while enforcing long-term averages.

```
Capacity: 10 tokens
Refill:   2 tokens/sec
Burst:    allows up to 10 simultaneous requests
```

### Fixed Window
Counts requests in fixed time windows. Simple, predictable, and memory-efficient.

```
Limit:  100 requests
Window: 60 seconds
Reset:  counter clears at the start of each new window
```

---

## 🛠 Tech Stack

| Layer       | Technology              | Purpose                        |
|-------------|-------------------------|--------------------------------|
| Backend     | Rust + Axum             | High-performance HTTP server   |
| Algorithms  | Custom Rust + Lua       | Token Bucket, Fixed Window     |
| Storage     | Redis 7                 | Atomic counters, TTL, Lua      |
| Frontend    | React + Recharts        | Admin dashboard                |
| Container   | Docker + Compose        | One-command deployment         |
| Proxy       | Nginx                   | Serve dashboard + proxy API    |

---

## 🚀 Getting Started

### Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop)
- [Rust 1.76+](https://rustup.rs) *(for manual setup)*
- [Node.js 20+](https://nodejs.org) *(for manual setup)*

### Option A — Docker (Recommended)

```bash
# Clone the repo
git clone https://github.com/yourusername/rate-limiter.git
cd rate-limiter

# Build and start all services
docker compose up --build
```

| Service   | URL                          |
|-----------|------------------------------|
| API       | http://localhost:8080        |
| Dashboard | http://localhost:3000        |
| Redis     | localhost:6379               |

---

### Option B — Manual (Development)

**Terminal 1 — Redis:**
```bash
docker compose up -d redis
```

**Terminal 2 — Rust Backend:**
```bash
cargo run
```

**Terminal 3 — React Dashboard:**
```bash
cd dashboard
npm install
npm run dev
# http://localhost:5173
```

---

## 📡 API Reference

### Health Check
```http
GET /health
```
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

### Check Rate Limit
```http
POST /check
Content-Type: application/json
```
```json
{
  "key": "user_123",
  "max_requests": 10,
  "window_secs": 60,
  "burst": 20
}
```
**Response — Allowed:**
```json
{
  "allowed": true,
  "remaining": 9,
  "retry_after_secs": null
}
```
**Response — Blocked:**
```json
{
  "allowed": false,
  "remaining": 0,
  "retry_after_secs": 45
}
```

---

### Create Rule
```http
POST /rules
Content-Type: application/json
```
```json
{
  "key": "api_key_abc",
  "max_requests": 100,
  "window_secs": 60,
  "burst": 150,
  "algorithm": "token_bucket"
}
```

---

### List Rules
```http
GET /rules
```
```json
{
  "rules": [...],
  "count": 3
}
```

---

### Delete Rule
```http
DELETE /rules/:id
```

---

### Reset Key
```http
POST /reset/:key
```

---

## 📊 Dashboard

The React admin dashboard gives you full visibility and control:

| Page          | Description                                      |
|---------------|--------------------------------------------------|
| **Dashboard** | Live traffic chart, server status, rule count    |
| **Rules**     | Create, view, delete, and reset rules            |
| **Checker**   | Manually test if any key is allowed or blocked   |

---

## 📈 Benchmarks

Run the built-in benchmark tool:

```bash
cargo run --bin benchmark
```

**Results on a standard laptop:**

```
────────────────────────────────
Total requests : 1000
Concurrency    : 50
────────────────────────────────
Results:
  Total time   : 0.85s
  Requests/sec : 1176
  Success      : 1000
  Failed       : 0
  Avg latency  : 0.42ms
  Min latency  : 0.18ms
  Max latency  : 3.21ms

✅ Goal achieved: avg latency < 1ms
────────────────────────────────
```

---

## 🧪 Tests

```bash
# Run all tests (requires Redis running)
docker compose up -d redis
cargo test

# Run specific test suites
cargo test --test token_bucket_test
cargo test --test fixed_window_test

# Run with output
cargo test -- --nocapture
```

**Test coverage:**

| Suite         | Tests | What's covered                                      |
|---------------|-------|-----------------------------------------------------|
| Token Bucket  | 6     | Limits, blocking, remaining, burst, reset, isolation |
| Fixed Window  | 7     | Limits, blocking, retry-after, reset, boundary edge cases |

---

## 📁 Project Structure

```
rate-limiter/
├── src/
│   ├── main.rs                  # Entry point + Axum server
│   ├── errors.rs                # Custom error types
│   ├── algorithms/
│   │   ├── mod.rs               # RateLimiter trait
│   │   ├── token_bucket.rs      # Token Bucket implementation
│   │   └── fixed_window.rs      # Fixed Window implementation
│   ├── store/
│   │   ├── mod.rs
│   │   └── redis_store.rs       # Redis + Lua script runner
│   ├── api/
│   │   ├── routes.rs            # All HTTP handlers
│   │   └── middleware.rs        # Rate limit middleware
│   ├── config/
│   │   └── mod.rs               # App config
│   └── bin/
│       └── benchmark.rs         # Benchmark tool
├── tests/
│   ├── token_bucket_test.rs
│   └── fixed_window_test.rs
├── dashboard/                   # React admin dashboard
│   ├── src/
│   │   ├── api/                 # API client
│   │   ├── components/          # UI components
│   │   ├── hooks/               # React Query hooks
│   │   └── pages/               # Dashboard, Rules, Checker
│   ├── Dockerfile
│   └── nginx.conf
├── Dockerfile
├── docker-compose.yml
└── README.md
```

---

## 🔧 Environment Variables

| Variable     | Default                    | Description              |
|--------------|----------------------------|--------------------------|
| `REDIS_URL`  | `redis://127.0.0.1:6379`   | Redis connection string  |
| `RUST_LOG`   | `info`                     | Log level                |

---

## 🗺 Roadmap

- [ ] Sliding Window algorithm
- [ ] Leaky Bucket algorithm
- [ ] Prometheus metrics endpoint
- [ ] Per-endpoint rule matching
- [ ] JWT / API key authentication
- [ ] Persistent rules in PostgreSQL
- [ ] Webhook alerts on threshold breach


---

<div align="center">

Built with ❤️ using Rust · Redis · React

</div>