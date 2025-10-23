//! Authentication support for HTTP requests

use reqwest::{RequestBuilder, header::AUTHORIZATION};
use crate::error::{Error, Result};
use base64::Engine;

/// Authentication configuration
#[derive(Debug, Clone)]
pub enum AuthConfig {
    /// Bearer token authentication
    Bearer(String),
    /// Basic authentication with username and password
    Basic { username: String, password: String },
    /// API key authentication with custom header
    ApiKey { key: String, header: String },
    /// Custom header authentication
    Custom { header: String, value: String },
}

impl AuthConfig {
    /// Create a new bearer token authentication
    pub fn bearer(token: impl Into<String>) -> Self {
        Self::Bearer(token.into())
    }

    /// Create a new basic authentication
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Create a new API key authentication
    pub fn api_key(key: impl Into<String>, header: impl Into<String>) -> Self {
        Self::ApiKey {
            key: key.into(),
            header: header.into(),
        }
    }

    /// Create a new custom header authentication
    pub fn custom(header: impl Into<String>, value: impl Into<String>) -> Self {
        Self::Custom {
            header: header.into(),
            value: value.into(),
        }
    }

    /// Apply authentication to a request builder
    pub fn apply_to_request(&self, request_builder: RequestBuilder) -> RequestBuilder {
        match self {
            AuthConfig::Bearer(token) => {
                let auth_header = format!("Bearer {}", token);
                request_builder.header(AUTHORIZATION, auth_header)
            }
            AuthConfig::Basic { username, password } => {
                let credentials = base64::engine::general_purpose::STANDARD
                    .encode(format!("{}:{}", username, password));
                let auth_header = format!("Basic {}", credentials);
                request_builder.header(AUTHORIZATION, auth_header)
            }
            AuthConfig::ApiKey { key, header } => {
                request_builder.header(header, key)
            }
            AuthConfig::Custom { header, value } => {
                request_builder.header(header, value)
            }
        }
    }

    /// Get the authentication type as a string
    pub fn auth_type(&self) -> &'static str {
        match self {
            AuthConfig::Bearer(_) => "Bearer",
            AuthConfig::Basic { .. } => "Basic",
            AuthConfig::ApiKey { .. } => "API Key",
            AuthConfig::Custom { .. } => "Custom",
        }
    }

    /// Check if the authentication is valid
    pub fn is_valid(&self) -> bool {
        match self {
            AuthConfig::Bearer(token) => !token.is_empty(),
            AuthConfig::Basic { username, password } => !username.is_empty() && !password.is_empty(),
            AuthConfig::ApiKey { key, header } => !key.is_empty() && !header.is_empty(),
            AuthConfig::Custom { header, value } => !header.is_empty() && !value.is_empty(),
        }
    }
}

/// Authentication manager for handling multiple authentication methods
pub struct AuthManager {
    auth_configs: Vec<AuthConfig>,
    current_index: usize,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new() -> Self {
        Self {
            auth_configs: Vec::new(),
            current_index: 0,
        }
    }

    /// Add an authentication configuration
    pub fn add_auth(&mut self, auth: AuthConfig) {
        self.auth_configs.push(auth);
    }

    /// Get the current authentication configuration
    pub fn current_auth(&self) -> Option<&AuthConfig> {
        self.auth_configs.get(self.current_index)
    }

    /// Rotate to the next authentication configuration
    pub fn rotate_auth(&mut self) -> Option<&AuthConfig> {
        if self.auth_configs.is_empty() {
            return None;
        }

        self.current_index = (self.current_index + 1) % self.auth_configs.len();
        self.current_auth()
    }

    /// Set the current authentication index
    pub fn set_current_index(&mut self, index: usize) -> Result<()> {
        if index >= self.auth_configs.len() {
            return Err(Error::http(format!(
                "Invalid auth index: {}. Available: {}",
                index, self.auth_configs.len()
            )));
        }
        self.current_index = index;
        Ok(())
    }

    /// Get the number of available authentication methods
    pub fn auth_count(&self) -> usize {
        self.auth_configs.len()
    }

    /// Clear all authentication configurations
    pub fn clear(&mut self) {
        self.auth_configs.clear();
        self.current_index = 0;
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Token refresh functionality for expired tokens
pub struct TokenRefresher {
    refresh_fn: Box<dyn Fn() -> Result<String> + Send + Sync>,
}

impl TokenRefresher {
    /// Create a new token refresher
    pub fn new<F>(refresh_fn: F) -> Self
    where
        F: Fn() -> Result<String> + Send + Sync + 'static,
    {
        Self {
            refresh_fn: Box::new(refresh_fn),
        }
    }

    /// Refresh the token
    pub fn refresh(&self) -> Result<String> {
        (self.refresh_fn)()
    }
}

/// Authentication with automatic token refresh
pub struct AutoRefreshAuth {
    auth_config: AuthConfig,
    refresher: Option<TokenRefresher>,
}

impl AutoRefreshAuth {
    /// Create a new auto-refresh authentication
    pub fn new(auth_config: AuthConfig) -> Self {
        Self {
            auth_config,
            refresher: None,
        }
    }

    /// Set the token refresher
    pub fn with_refresher(mut self, refresher: TokenRefresher) -> Self {
        self.refresher = Some(refresher);
        self
    }

    /// Get the current authentication configuration
    pub fn auth_config(&self) -> &AuthConfig {
        &self.auth_config
    }

    /// Refresh the token if a refresher is configured
    pub fn refresh_token(&self) -> Result<Option<String>> {
        if let Some(refresher) = &self.refresher {
            Ok(Some(refresher.refresh()?))
        } else {
            Ok(None)
        }
    }
}
