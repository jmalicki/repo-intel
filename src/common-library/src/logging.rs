//! Logging functionality for the common library

use tracing::{info, warn, error, debug, Level};
use tracing_subscriber::{fmt, EnvFilter, Registry, prelude::*};
use crate::error::{Error, Result};

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: Level,
    pub format: LogFormat,
    pub output: LogOutput,
}

#[derive(Debug, Clone)]
pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

#[derive(Debug, Clone)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            format: LogFormat::Pretty,
            output: LogOutput::Stdout,
        }
    }
}

/// Initialize the global logger with default configuration
pub fn init() -> Result<()> {
    init_with_config(LoggerConfig::default())
}

/// Initialize the global logger with custom configuration
pub fn init_with_config(config: LoggerConfig) -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("{}", config.level)));

    let registry = Registry::default().with(filter);

    match config.output {
        LogOutput::Stdout => {
            let fmt_layer = match config.format {
                LogFormat::Json => fmt::layer().json().boxed(),
                LogFormat::Pretty => fmt::layer().pretty().boxed(),
                LogFormat::Compact => fmt::layer().compact().boxed(),
            };
            registry.with(fmt_layer).init();
        }
        LogOutput::Stderr => {
            let fmt_layer = match config.format {
                LogFormat::Json => fmt::layer().json().with_writer(std::io::stderr).boxed(),
                LogFormat::Pretty => fmt::layer().pretty().with_writer(std::io::stderr).boxed(),
                LogFormat::Compact => fmt::layer().compact().with_writer(std::io::stderr).boxed(),
            };
            registry.with(fmt_layer).init();
        }
        LogOutput::File(path) => {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .map_err(|e| Error::Io(e))?;

            let fmt_layer = match config.format {
                LogFormat::Json => fmt::layer().json().with_writer(file).boxed(),
                LogFormat::Pretty => fmt::layer().pretty().with_writer(file).boxed(),
                LogFormat::Compact => fmt::layer().compact().with_writer(file).boxed(),
            };
            registry.with(fmt_layer).init();
        }
    }

    Ok(())
}

/// Logger struct for structured logging
pub struct Logger {
    target: String,
}

impl Logger {
    /// Create a new logger for the specified target
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
        }
    }

    /// Log an info message
    pub fn info(&self, message: &str) {
        info!("[{}] {}", self.target, message);
    }

    /// Log an info message with fields
    pub fn info_with_fields(&self, message: &str, _fields: &[(&str, &str)]) {
        // Simplified implementation - just log the message
        info!("[{}] {}", self.target, message);
    }

    /// Log a warning message
    pub fn warn(&self, message: &str) {
        warn!("[{}] {}", self.target, message);
    }

    /// Log a warning message with fields
    pub fn warn_with_fields(&self, message: &str, _fields: &[(&str, &str)]) {
        // Simplified implementation - just log the message
        warn!("[{}] {}", self.target, message);
    }

    /// Log an error message
    pub fn error(&self, message: &str) {
        error!("[{}] {}", self.target, message);
    }

    /// Log an error message with fields
    pub fn error_with_fields(&self, message: &str, _fields: &[(&str, &str)]) {
        // Simplified implementation - just log the message
        error!("[{}] {}", self.target, message);
    }

    /// Log a debug message
    pub fn debug(&self, message: &str) {
        debug!("[{}] {}", self.target, message);
    }

    /// Log a debug message with fields
    pub fn debug_with_fields(&self, message: &str, _fields: &[(&str, &str)]) {
        // Simplified implementation - just log the message
        debug!("[{}] {}", self.target, message);
    }

    /// Log performance metrics
    pub fn log_performance(&self, operation: &str, duration: std::time::Duration) {
        let duration_ms = duration.as_millis();
        info!(
            "[{}] Performance: {} completed in {}ms",
            self.target, operation, duration_ms
        );
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new("common-library")
    }
}
