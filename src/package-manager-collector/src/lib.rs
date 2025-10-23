//! Package Manager Collector
//!
//! A comprehensive package manager collector that integrates with multiple package registries
//! to collect package metadata, perform health analysis, and resolve data conflicts.

pub mod config;
pub mod models;

// Re-export commonly used types
pub use config::Config;
pub use models::*;

/// Package Manager Collector version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Package Manager Collector name
pub const NAME: &str = env!("CARGO_PKG_NAME");
