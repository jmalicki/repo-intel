//! Retry logic with exponential backoff for HTTP requests

use std::time::Duration;
use crate::error::{Error, Result};
use crate::logging::Logger;

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            ..Default::default()
        }
    }

    /// Calculate the backoff delay for a given attempt
    pub fn calculate_backoff(&self, attempt: u32) -> Duration {
        let base_delay = self.initial_delay.as_millis() as f64;
        let multiplier = self.backoff_multiplier.powi(attempt as i32 - 1);
        let delay_ms = base_delay * multiplier;

        let mut delay = Duration::from_millis(delay_ms as u64);

        // Apply jitter to prevent thundering herd
        if self.jitter {
            let jitter_factor = 0.1; // 10% jitter
            let jitter_range = (delay.as_millis() as f64 * jitter_factor) as u64;
            let jitter = fastrand::u64(0..=jitter_range);
            delay = Duration::from_millis(delay.as_millis() as u64 + jitter);
        }

        // Cap at maximum delay
        if delay > self.max_delay {
            delay = self.max_delay;
        }

        delay
    }

    /// Check if we should retry based on the attempt number
    pub fn should_retry(&self, attempt: u32) -> bool {
        attempt <= self.max_retries
    }
}

/// Retry executor that handles retry logic
pub struct RetryExecutor {
    config: RetryConfig,
    logger: Logger,
}

impl RetryExecutor {
    /// Create a new retry executor
    pub fn new(config: RetryConfig) -> Self {
        Self {
            config,
            logger: Logger::new("retry-executor"),
        }
    }

    /// Execute a function with retry logic
    pub async fn execute<F, Fut, T>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut attempt = 1;
        #[allow(unused_assignments)]
        let mut last_error = None;

        loop {
            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        self.logger.info(&format!(
                            "Operation succeeded on attempt {}",
                            attempt
                        ));
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e);

                    if !self.config.should_retry(attempt) {
                        self.logger.error(&format!(
                            "Operation failed after {} attempts. Last error: {}. Giving up.",
                            attempt,
                            last_error.as_ref().unwrap()
                        ));
                        break;
                    }

                    let backoff = self.config.calculate_backoff(attempt);
                    self.logger.warn(&format!(
                        "Operation failed on attempt {}/{}. Retrying in {:?}",
                        attempt, self.config.max_retries, backoff
                    ));

                    tokio::time::sleep(backoff).await;
                    attempt += 1;
                }
            }
        }

        // Use last_error to avoid unused assignment warning
        let final_error = last_error.unwrap_or_else(|| Error::http("Retry exhausted".to_string()));
        Err(final_error)
    }

    /// Execute with custom retry condition
    pub async fn execute_with_condition<F, Fut, T, C>(
        &self,
        mut operation: F,
        mut should_retry: C,
    ) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
        C: FnMut(&Error) -> bool,
    {
        let mut attempt = 1;
        #[allow(unused_assignments)]
        let mut last_error = None;

        loop {
            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        self.logger.info(&format!(
                            "Operation succeeded on attempt {}",
                            attempt
                        ));
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e.clone());

                    if !self.config.should_retry(attempt) || !should_retry(&e) {
                        self.logger.error(&format!(
                            "Operation failed after {} attempts or retry condition not met. Last error: {}. Giving up.",
                            attempt,
                            last_error.as_ref().unwrap()
                        ));
                        break;
                    }

                    let backoff = self.config.calculate_backoff(attempt);
                    self.logger.warn(&format!(
                        "Operation failed on attempt {}/{}. Retrying in {:?}",
                        attempt, self.config.max_retries, backoff
                    ));

                    tokio::time::sleep(backoff).await;
                    attempt += 1;
                }
            }
        }

        // Use last_error to avoid unused assignment warning
        let final_error = last_error.unwrap_or_else(|| Error::http("Retry exhausted".to_string()));
        Err(final_error)
    }
}

/// Retry strategies for different types of errors
pub mod strategies {
    use super::*;

    /// Retry strategy for HTTP status codes
    pub fn http_status_retry(error: &Error) -> bool {
        match error {
            Error::Http(msg) => {
                // Retry on 5xx server errors and 429 (rate limited)
                msg.contains("500") ||
                msg.contains("502") ||
                msg.contains("503") ||
                msg.contains("504") ||
                msg.contains("429")
            }
            _ => false,
        }
    }

    /// Retry strategy for network errors
    pub fn network_retry(error: &Error) -> bool {
        match error {
            Error::Http(msg) => {
                msg.contains("timeout") ||
                msg.contains("connection") ||
                msg.contains("network")
            }
            _ => false,
        }
    }

    /// Retry strategy for rate limiting
    pub fn rate_limit_retry(error: &Error) -> bool {
        match error {
            Error::Http(msg) => {
                msg.contains("429") || msg.contains("rate limit")
            }
            _ => false,
        }
    }

    /// Combined retry strategy
    pub fn combined_retry(error: &Error) -> bool {
        http_status_retry(error) || network_retry(error) || rate_limit_retry(error)
    }
}
