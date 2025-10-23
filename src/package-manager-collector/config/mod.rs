//! Configuration Management
//!
//! This module handles configuration loading and management for the package manager collector.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Database configuration
    pub database: DatabaseConfig,
    /// HTTP client configuration
    pub http_client: HttpClientConfig,
    /// Package manager configurations
    pub package_managers: HashMap<String, PackageManagerConfig>,
    /// Collection settings
    pub collection: CollectionConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL
    pub url: String,
    /// Maximum number of connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
}

/// HTTP client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpClientConfig {
    /// Request timeout in seconds
    pub timeout: u64,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Retry delay in seconds
    pub retry_delay: u64,
    /// User agent string
    pub user_agent: String,
}

/// Package manager specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagerConfig {
    /// Whether this package manager is enabled
    pub enabled: bool,
    /// API base URL
    pub api_base_url: String,
    /// Authentication token (if required)
    pub auth_token: Option<String>,
    /// Rate limit settings
    pub rate_limit: RateLimitConfig,
    /// Collection settings
    pub collection_settings: HashMap<String, serde_json::Value>,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Requests per hour
    pub requests_per_hour: u32,
    /// Burst limit
    pub burst_limit: u32,
}

/// Collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// Maximum number of concurrent collections
    pub max_concurrent: u32,
    /// Batch size for processing
    pub batch_size: u32,
    /// Whether to enable conflict resolution
    pub enable_conflict_resolution: bool,
    /// Whether to enable health analysis
    pub enable_health_analysis: bool,
    /// Collection timeout in seconds
    pub timeout: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format (json, pretty)
    pub format: String,
    /// Whether to enable file logging
    pub enable_file_logging: bool,
    /// Log file path
    pub file_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            http_client: HttpClientConfig::default(),
            package_managers: HashMap::new(),
            collection: CollectionConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://./package_manager_collector.db".to_string(),
            max_connections: 10,
            connection_timeout: 30,
        }
    }
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: 30,
            max_retries: 3,
            retry_delay: 1,
            user_agent: "PackageManagerCollector/1.0".to_string(),
        }
    }
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 5,
            batch_size: 100,
            enable_conflict_resolution: true,
            enable_health_analysis: true,
            timeout: 3600,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            enable_file_logging: false,
            file_path: None,
        }
    }
}
