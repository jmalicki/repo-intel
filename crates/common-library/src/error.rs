//! Error types and handling for the common library

use thiserror::Error;

/// Common error type used throughout the library
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Processing error: {0}")]
    Processing(String),

    #[error("Metrics error: {0}")]
    Metrics(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration parsing error: {0}")]
    ConfigParse(#[from] config::ConfigError),

    #[error("Generic error: {0}")]
    Generic(String),
}

impl Error {
    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a new HTTP error
    pub fn http(msg: impl Into<String>) -> Self {
        Self::Http(msg.into())
    }

    /// Create a new database error
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// Create a new storage error
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }

    /// Create a new validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a new processing error
    pub fn processing(msg: impl Into<String>) -> Self {
        Self::Processing(msg.into())
    }

    /// Create a new metrics error
    pub fn metrics(msg: impl Into<String>) -> Self {
        Self::Metrics(msg.into())
    }

    /// Create a new generic error
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}

/// Convenience type alias for results
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        // Test: Error creation methods work correctly
        let config_error = Error::config("test config error");
        assert!(matches!(config_error, Error::Config(_)));

        let http_error = Error::http("test http error");
        assert!(matches!(http_error, Error::Http(_)));

        let database_error = Error::database("test database error");
        assert!(matches!(database_error, Error::Database(_)));

        let storage_error = Error::storage("test storage error");
        assert!(matches!(storage_error, Error::Storage(_)));

        let validation_error = Error::validation("test validation error");
        assert!(matches!(validation_error, Error::Validation(_)));

        let processing_error = Error::processing("test processing error");
        assert!(matches!(processing_error, Error::Processing(_)));

        let metrics_error = Error::metrics("test metrics error");
        assert!(matches!(metrics_error, Error::Metrics(_)));

        let generic_error = Error::generic("test generic error");
        assert!(matches!(generic_error, Error::Generic(_)));
    }

    #[test]
    fn test_error_display() {
        // Test: Error display formatting works correctly
        let config_error = Error::config("test message");
        assert!(config_error.to_string().contains("Configuration error"));
        assert!(config_error.to_string().contains("test message"));

        let http_error = Error::http("test message");
        assert!(http_error.to_string().contains("HTTP error"));
        assert!(http_error.to_string().contains("test message"));
    }

    #[test]
    fn test_error_from_io() {
        // Test: Error conversion from std::io::Error works
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let common_error: Error = io_error.into();
        assert!(matches!(common_error, Error::Io(_)));
    }

    #[test]
    fn test_error_from_serde() {
        // Test: Error conversion from serde_json::Error works
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let common_error: Error = json_error.into();
        assert!(matches!(common_error, Error::Serialization(_)));
    }
}
