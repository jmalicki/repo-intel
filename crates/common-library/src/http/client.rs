//! HTTP client implementation with rate limiting and retry logic

use super::{AuthConfig, RateLimiter, RetryConfig};
use crate::error::{Error, Result};
use crate::logging::Logger;
use reqwest::{Client, ClientBuilder, Request, Response};
use std::time::Duration;

/// HTTP client configuration
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout: Duration,
    pub max_retries: u32,
    pub rate_limit_per_minute: u32,
    pub user_agent: String,
    pub base_url: Option<String>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            rate_limit_per_minute: 60,
            user_agent: "common-library/0.1.0".to_string(),
            base_url: None,
        }
    }
}

/// Main HTTP client with rate limiting and retry logic
pub struct APIClient {
    client: Client,
    rate_limiter: RateLimiter,
    retry_config: RetryConfig,
    auth_config: Option<AuthConfig>,
    config: HttpClientConfig,
    logger: Logger,
}

impl APIClient {
    /// Create a new API client with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(HttpClientConfig::default())
    }

    /// Create a new API client with custom configuration
    pub fn with_config(config: HttpClientConfig) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| Error::http(format!("Failed to create HTTP client: {}", e)))?;

        let rate_limiter = RateLimiter::new(config.rate_limit_per_minute);
        let retry_config = RetryConfig::default();
        let logger = Logger::new("http-client");

        Ok(Self {
            client,
            rate_limiter,
            retry_config,
            auth_config: None,
            config,
            logger,
        })
    }

    /// Set authentication configuration
    pub fn set_auth(&mut self, auth_config: AuthConfig) {
        self.auth_config = Some(auth_config);
    }

    /// Set retry configuration
    pub fn set_retry_config(&mut self, retry_config: RetryConfig) {
        self.retry_config = retry_config;
    }

    /// Set rate limiting configuration
    pub fn set_rate_limit(&mut self, requests_per_minute: u32) {
        self.rate_limiter = RateLimiter::new(requests_per_minute);
    }

    /// Make a GET request
    pub async fn get(&self, url: &str) -> Result<Response> {
        let request = self.build_request("GET", url, None)?;
        self.execute_request(request).await
    }

    /// Make a GET request with query parameters
    pub async fn get_with_params(&self, url: &str, params: &[(&str, &str)]) -> Result<Response> {
        let mut request_builder = self.client.get(url);

        for (key, value) in params {
            request_builder = request_builder.query(&[(key, value)]);
        }

        let request = self.build_request_from_builder(request_builder)?;
        self.execute_request(request).await
    }

    /// Make a POST request
    pub async fn post(&self, url: &str, body: Option<&str>) -> Result<Response> {
        let request = self.build_request("POST", url, body)?;
        self.execute_request(request).await
    }

    /// Make a POST request with JSON body
    pub async fn post_json<T>(&self, url: &str, body: &T) -> Result<Response>
    where
        T: serde::Serialize,
    {
        let json_body = serde_json::to_string(body)
            .map_err(|e| Error::http(format!("Failed to serialize JSON: {}", e)))?;

        let request = self.build_request("POST", url, Some(&json_body))?;
        self.execute_request(request).await
    }

    /// Make a PUT request
    pub async fn put(&self, url: &str, body: Option<&str>) -> Result<Response> {
        let request = self.build_request("PUT", url, body)?;
        self.execute_request(request).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, url: &str) -> Result<Response> {
        let request = self.build_request("DELETE", url, None)?;
        self.execute_request(request).await
    }

    /// Build a request with authentication and headers
    fn build_request(&self, method: &str, url: &str, body: Option<&str>) -> Result<Request> {
        let full_url = if let Some(base_url) = &self.config.base_url {
            format!("{}{}", base_url, url)
        } else {
            url.to_string()
        };

        let mut request_builder = match method {
            "GET" => self.client.get(&full_url),
            "POST" => self.client.post(&full_url),
            "PUT" => self.client.put(&full_url),
            "DELETE" => self.client.delete(&full_url),
            _ => return Err(Error::http(format!("Unsupported HTTP method: {}", method))),
        };

        // Add authentication if configured
        if let Some(auth) = &self.auth_config {
            request_builder = auth.apply_to_request(request_builder);
        }

        // Add body if provided
        if let Some(body_str) = body {
            request_builder = request_builder.body(body_str.to_string());
        }

        self.build_request_from_builder(request_builder)
    }

    /// Build request from a request builder
    fn build_request_from_builder(
        &self,
        mut request_builder: reqwest::RequestBuilder,
    ) -> Result<Request> {
        // Add common headers
        request_builder = request_builder
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        request_builder
            .build()
            .map_err(|e| Error::http(format!("Failed to build request: {}", e)))
    }

    /// Execute a request with rate limiting and retry logic
    async fn execute_request(&self, request: Request) -> Result<Response> {
        let mut attempt = 0;
        let max_retries = self.retry_config.max_retries;

        loop {
            // Apply rate limiting
            self.rate_limiter.acquire().await?;

            // Clone the request for retry attempts
            let request_clone = request
                .try_clone()
                .ok_or_else(|| Error::http("Failed to clone request".to_string()))?;

            match self.client.execute(request_clone).await {
                Ok(response) => {
                    self.logger.info(&format!(
                        "HTTP request successful: {} {}",
                        response.status(),
                        response.url()
                    ));
                    return Ok(response);
                }
                Err(e) if attempt < max_retries => {
                    attempt += 1;
                    let backoff = self.retry_config.calculate_backoff(attempt);

                    self.logger.warn(&format!(
                        "HTTP request failed (attempt {}/{}): {}. Retrying in {:?}",
                        attempt, max_retries, e, backoff
                    ));

                    tokio::time::sleep(backoff).await;
                    continue;
                }
                Err(e) => {
                    self.logger.error(&format!(
                        "HTTP request failed after {} attempts: {}",
                        max_retries, e
                    ));
                    return Err(Error::http(format!("Request failed: {}", e)));
                }
            }
        }
    }

    /// Get the underlying HTTP client for advanced usage
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the current configuration
    pub fn config(&self) -> &HttpClientConfig {
        &self.config
    }
}

impl Default for APIClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default APIClient")
    }
}
