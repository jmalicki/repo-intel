//! HTTP client functionality for the common library

pub mod auth;
pub mod client;
pub mod rate_limiter;
pub mod retry;

// Re-exports for convenient usage
pub use auth::{AuthConfig, AuthManager};
pub use client::{APIClient, HttpClientConfig};
pub use rate_limiter::RateLimiter;
pub use retry::RetryConfig;
