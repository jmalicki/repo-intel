//! HTTP client functionality for the common library

pub mod client;
pub mod rate_limiter;
pub mod retry;
pub mod auth;

// Re-exports for convenient usage
pub use client::{APIClient, HttpClientConfig};
pub use rate_limiter::RateLimiter;
pub use retry::RetryConfig;
pub use auth::{AuthConfig, AuthManager};
