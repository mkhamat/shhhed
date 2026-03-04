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
#[derive(Debug)]
struct Bucket {
    tokens: f64,
    capacity: u32,
    last_refill: Instant,
    rate: f64,
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
        bucket.refill();

        if bucket.tokens >= n as f64 {
            bucket.tokens -= n as f64;
            Ok(bucket.tokens as u32)
        } else {
            let wait = Duration::from_secs_f64(
                (n as f64 - bucket.tokens) / bucket.rate,
            );
            Err(RateLimitError { retry_after: wait })
        }
    }
}

impl Bucket {
    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.rate)
            .min(self.capacity as f64);
        self.last_refill = Instant::now();
    }
}

#[derive(Debug)]
pub struct RateLimitError {
    pub retry_after: Duration,
}
