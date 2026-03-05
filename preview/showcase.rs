/// Thread-safe rate limiter using the token bucket algorithm.
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

const DEFAULT_CAPACITY: u32 = 100;
const REFILL_RATE: f64 = 10.0; // tokens per second

#[derive(Debug, Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<Bucket>>,
}

impl RateLimiter {
    pub fn new(capacity: u32, rate: f64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Bucket {
                tokens: capacity as f64,
                capacity,
                last_refill: Instant::now(),
                rate,
            })),
        }
    }

    /// Attempt to acquire `n` tokens. Returns remaining capacity.
    pub async fn acquire(&self, n: u32) -> Result<u32, RateLimitError> {
        let mut bucket = self.inner.lock().await;
        if bucket.tokens >= n as f64 {
            bucket.tokens -= n as f64;
            Ok(bucket.tokens as u32)
        } else {
            Err(RateLimitError { retry_after: Duration::from_secs(1) })
        }
    }
}
