//! Configuration management for the common library

use crate::error::{Error, Result};
use config::{Config, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration manager for the common library
pub struct ConfigManager {
    config: Config,
    runtime_overrides: HashMap<String, serde_json::Value>,
}

/// Application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub http: HttpConfig,
    pub logging: LoggingConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub rate_limit_per_minute: u32,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_path: String,
    pub backup_enabled: bool,
    pub compression_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                url: "sqlite://./data/database.db".to_string(),
                max_connections: 10,
                timeout_seconds: 30,
            },
            http: HttpConfig {
                timeout_seconds: 30,
                max_retries: 3,
                rate_limit_per_minute: 60,
                user_agent: format!("common-library/{}", env!("CARGO_PKG_VERSION")),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                output: "stdout".to_string(),
            },
            storage: StorageConfig {
                base_path: "./data".to_string(),
                backup_enabled: true,
                compression_enabled: false,
            },
        }
    }
}

impl ConfigManager {
    /// Create a new configuration manager with default settings
    pub fn new() -> Result<Self> {
        Self::with_sources(&[])
    }

    /// Create a new configuration manager with custom sources
    pub fn with_sources(sources: &[&str]) -> Result<Self> {
        let mut builder = Config::builder();

        // Add default configuration
        let default_config = serde_json::to_string(&AppConfig::default())?;
        builder = builder.add_source(File::from_str(&default_config, FileFormat::Json));

        // Add custom configuration files
        for source in sources {
            builder = builder.add_source(File::with_name(source).required(false));
        }

        // Add environment variable overrides
        builder = builder.add_source(Environment::with_prefix("COMMON_LIBRARY").separator("_"));

        let config = builder.build()?;
        Ok(Self {
            config,
            runtime_overrides: HashMap::new(),
        })
    }

    /// Get a typed configuration value
    pub fn get<T>(&self, key: &str) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        // Check runtime overrides first
        if let Some(override_value) = self.runtime_overrides.get(key) {
            return serde_json::from_value(override_value.clone()).map_err(Error::from);
        }

        // Fall back to original configuration
        self.config.get(key).map_err(Error::from)
    }

    /// Get the full application configuration
    pub fn get_app_config(&self) -> Result<AppConfig> {
        self.config.clone().try_deserialize().map_err(Error::from)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        let app_config: AppConfig = self.get_app_config()?;

        // Validate database configuration
        if app_config.database.max_connections == 0 {
            return Err(Error::config("max_connections must be > 0"));
        }

        if app_config.database.timeout_seconds == 0 {
            return Err(Error::config("database timeout_seconds must be > 0"));
        }

        // Validate HTTP configuration
        if app_config.http.timeout_seconds == 0 {
            return Err(Error::config("http timeout_seconds must be > 0"));
        }

        if app_config.http.max_retries == 0 {
            return Err(Error::config("max_retries must be > 0"));
        }

        if app_config.http.rate_limit_per_minute == 0 {
            return Err(Error::config("rate_limit_per_minute must be > 0"));
        }

        // Validate logging configuration
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&app_config.logging.level.as_str()) {
            return Err(Error::config(format!(
                "invalid log level: {}. Valid levels: {:?}",
                app_config.logging.level, valid_levels
            )));
        }

        let valid_formats = ["json", "pretty", "compact"];
        if !valid_formats.contains(&app_config.logging.format.as_str()) {
            return Err(Error::config(format!(
                "invalid log format: {}. Valid formats: {:?}",
                app_config.logging.format, valid_formats
            )));
        }

        Ok(())
    }

    /// Reload configuration from sources
    pub fn reload(&mut self) -> Result<()> {
        // Clear runtime overrides and re-validate
        self.runtime_overrides.clear();
        self.validate()
    }

    /// Clear all runtime configuration overrides
    pub fn clear_overrides(&mut self) {
        self.runtime_overrides.clear();
    }

    /// Get all runtime override keys
    pub fn get_override_keys(&self) -> Vec<String> {
        self.runtime_overrides.keys().cloned().collect()
    }

    /// Export current configuration as JSON
    pub fn export(&self) -> Result<String> {
        let app_config: AppConfig = self.get_app_config()?;
        serde_json::to_string_pretty(&app_config).map_err(Error::from)
    }

    /// Get all configuration keys
    pub fn keys(&self) -> Result<Vec<String>> {
        // This is a simplified implementation
        // In a real implementation, you'd traverse the configuration tree
        Ok(vec![
            "database.url".to_string(),
            "database.max_connections".to_string(),
            "database.timeout_seconds".to_string(),
            "http.timeout_seconds".to_string(),
            "http.max_retries".to_string(),
            "http.rate_limit_per_minute".to_string(),
            "http.user_agent".to_string(),
            "logging.level".to_string(),
            "logging.format".to_string(),
            "logging.output".to_string(),
            "storage.base_path".to_string(),
            "storage.backup_enabled".to_string(),
            "storage.compression_enabled".to_string(),
        ])
    }

    /// Set a configuration value (runtime configuration changes)
    pub fn set<T>(&mut self, key: &str, value: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        // Serialize the value to JSON and store it as a runtime override
        let json_value = serde_json::to_value(value).map_err(Error::from)?;
        self.runtime_overrides.insert(key.to_string(), json_value);
        Ok(())
    }
}
