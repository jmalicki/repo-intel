//! Storage module for database and file operations
//!
//! This module provides async database operations with diesel-async,
//! file system operations for JSON data, connection pooling,
//! migration management, and backup/restore functionality.

pub mod backup;
pub mod database;
pub mod filesystem;
pub mod migrations;

// Re-exports for convenient usage
pub use backup::{BackupManager, BackupStrategy};
pub use database::{ConnectionPool, DatabaseManager, Transaction};
pub use filesystem::{FileManager, JsonFileManager};
pub use migrations::{Migration, MigrationManager};
