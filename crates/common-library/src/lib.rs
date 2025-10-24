//! # Common Library
//!
//! A shared Rust library that provides reusable functionality for all Phase 1 project selection tools.
//! It eliminates code duplication and ensures consistent behavior across all tools.
//!
//! ## Features
//!
//! - **HTTP Client**: Robust HTTP client with rate limiting, retry logic, and authentication
//! - **Storage**: Database and file operations with async support
//! - **Configuration**: Centralized configuration management with validation
//! - **Logging**: Structured logging with configurable levels and output formats
//! - **Metrics**: Statistical calculations and metrics processing
//! - **Validation**: Data validation and schema management
//! - **Processing**: Data transformation, cleaning, and aggregation
//! - **Utils**: Utility functions for common operations
//!
//! ## Usage
//!
//! ```rust
//! use common_library::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize logging
//!     common_library::logging::init()?;
//!
//!     // Load configuration
//!     let config = common_library::config::ConfigManager::new()?;
//!
//!     // Create a logger
//!     let logger = Logger::new("my-app");
//!     logger.info("Application started");
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod logging;
pub mod utils;

// HTTP client module
pub mod http;

// Storage module
pub mod storage;

// Metrics module
pub mod metrics;

/// Re-exports for convenient usage
pub mod prelude {
    pub use crate::config::ConfigManager;
    pub use crate::error::{Error, Result};
    pub use crate::http::*;
    pub use crate::logging::Logger;
    pub use crate::storage::*;
    pub use crate::metrics::*;
    pub use crate::utils::*;
}

/// Common result type used throughout the library
pub type Result<T> = std::result::Result<T, error::Error>;
