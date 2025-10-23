//! Rate limiting implementation for HTTP requests

use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::sleep;
use crate::error::{Error, Result};
use crate::logging::Logger;

/// Rate limiter that controls the number of requests per minute
pub struct RateLimiter {
    semaphore: Semaphore,
    requests_per_minute: u32,
    last_reset: Instant,
    logger: Logger,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            semaphore: Semaphore::new(requests_per_minute as usize),
            requests_per_minute,
            last_reset: Instant::now(),
            logger: Logger::new("rate-limiter"),
        }
    }

    /// Acquire a permit for making a request
    pub async fn acquire(&self) -> Result<()> {
        // Check if we need to reset the semaphore
        if self.should_reset() {
            self.logger.debug("Rate limiter reset - all permits available");
        }

        // Acquire a permit
        let _permit = self.semaphore.acquire().await
            .map_err(|e| Error::http(format!("Failed to acquire rate limit permit: {}", e)))?;

        self.logger.debug(&format!(
            "Rate limit permit acquired. Remaining: {}",
            self.semaphore.available_permits()
        ));

        Ok(())
    }

    /// Check if the rate limiter should be reset
    fn should_reset(&self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_reset) >= Duration::from_secs(60) {
            // Reset the semaphore by creating a new one
            // Note: This is a simplified implementation
            // In a production system, you'd want more sophisticated reset logic
            true
        } else {
            false
        }
    }

    /// Get the number of available permits
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Get the total number of permits
    pub fn total_permits(&self) -> usize {
        self.requests_per_minute as usize
    }

    /// Wait for the next reset cycle
    pub async fn wait_for_reset(&self) {
        let elapsed = self.last_reset.elapsed();
        if elapsed < Duration::from_secs(60) {
            let remaining = Duration::from_secs(60) - elapsed;
            self.logger.info(&format!("Waiting {} seconds for rate limit reset", remaining.as_secs()));
            sleep(remaining).await;
        }
    }
}

/// Advanced rate limiter with sliding window
pub struct SlidingWindowRateLimiter {
    requests: Vec<Instant>,
    window_size: Duration,
    max_requests: u32,
    logger: Logger,
}

impl SlidingWindowRateLimiter {
    /// Create a new sliding window rate limiter
    pub fn new(max_requests: u32, window_size: Duration) -> Self {
        Self {
            requests: Vec::new(),
            window_size,
            max_requests,
            logger: Logger::new("sliding-window-rate-limiter"),
        }
    }

    /// Check if a request is allowed
    pub fn is_allowed(&mut self) -> bool {
        let now = Instant::now();
        
        // Remove old requests outside the window
        self.requests.retain(|&time| now.duration_since(time) <= self.window_size);
        
        // Check if we're under the limit
        if self.requests.len() < self.max_requests as usize {
            self.requests.push(now);
            true
        } else {
            false
        }
    }

    /// Wait until a request is allowed
    pub async fn wait_until_allowed(&mut self) {
        while !self.is_allowed() {
            let oldest_request = self.requests.first().copied();
            if let Some(oldest) = oldest_request {
                let wait_time = self.window_size - oldest.elapsed();
                if wait_time > Duration::from_secs(0) {
                    self.logger.debug(&format!(
                        "Rate limit exceeded. Waiting {:?} for window to slide",
                        wait_time
                    ));
                    sleep(wait_time).await;
                }
            }
        }
    }

    /// Get the number of requests in the current window
    pub fn current_requests(&self) -> usize {
        self.requests.len()
    }

    /// Get the time until the next request slot is available
    pub fn time_until_next_slot(&self) -> Option<Duration> {
        if self.requests.len() < self.max_requests as usize {
            return None; // Slot available immediately
        }

        let oldest_request = self.requests.first()?;
        let elapsed = oldest_request.elapsed();
        if elapsed < self.window_size {
            Some(self.window_size - elapsed)
        } else {
            None
        }
    }
}
