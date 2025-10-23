//! Collection Metadata Models
//!
//! This module contains models for tracking collection runs, errors,
//! rate limits, and conflicts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collection run metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRun {
    /// Collection run ID
    pub id: String,
    /// Collection start time
    pub started_at: DateTime<Utc>,
    /// Collection end time
    pub ended_at: Option<DateTime<Utc>>,
    /// Collection status
    pub status: CollectionStatus,
    /// Number of packages collected
    pub packages_collected: u32,
    /// Number of packages failed
    pub packages_failed: u32,
    /// Total execution time in seconds
    pub execution_time_seconds: Option<u64>,
    /// Collection configuration used
    pub configuration: HashMap<String, serde_json::Value>,
    /// Collection errors encountered
    pub errors: Vec<CollectionError>,
}

/// Collection status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Collection error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionError {
    /// Error ID
    pub id: String,
    /// Collection run ID this error belongs to
    pub collection_run_id: String,
    /// Error timestamp
    pub occurred_at: DateTime<Utc>,
    /// Error type
    pub error_type: ErrorType,
    /// Error message
    pub message: String,
    /// Package name that caused the error (if applicable)
    pub package_name: Option<String>,
    /// Registry that caused the error (if applicable)
    pub registry: Option<String>,
    /// HTTP status code (if applicable)
    pub http_status_code: Option<u16>,
    /// Additional error context
    pub context: HashMap<String, serde_json::Value>,
}

/// Error type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorType {
    NetworkError,
    AuthenticationError,
    RateLimitExceeded,
    PackageNotFound,
    InvalidResponse,
    DatabaseError,
    ConfigurationError,
    UnknownError,
}

/// API rate limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRateLimit {
    /// Rate limit ID
    pub id: String,
    /// Registry name
    pub registry: String,
    /// Rate limit timestamp
    pub timestamp: DateTime<Utc>,
    /// Requests remaining
    pub requests_remaining: u32,
    /// Requests limit
    pub requests_limit: u32,
    /// Reset time
    pub reset_time: DateTime<Utc>,
    /// Rate limit window in seconds
    pub window_seconds: u32,
}

/// Package conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConflict {
    /// Conflict ID
    pub id: String,
    /// Package name in conflict
    pub package_name: String,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Conflict severity
    pub severity: ConflictSeverity,
    /// Conflict description
    pub description: String,
    /// Conflicting data sources
    pub conflicting_sources: Vec<ConflictSource>,
    /// Conflict resolution status
    pub resolution_status: ResolutionStatus,
    /// Conflict discovered at
    pub discovered_at: DateTime<Utc>,
    /// Conflict resolved at
    pub resolved_at: Option<DateTime<Utc>>,
    /// Conflict resolution notes
    pub resolution_notes: Option<String>,
}

/// Conflict type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    VersionMismatch,
    LicenseConflict,
    DependencyConflict,
    MetadataConflict,
    RepositoryConflict,
}

/// Conflict severity enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Conflict source information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictSource {
    /// Source registry
    pub registry: String,
    /// Source data
    pub data: serde_json::Value,
    /// Source priority
    pub priority: u8,
    /// Source timestamp
    pub timestamp: DateTime<Utc>,
}

/// Resolution status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStatus {
    Unresolved,
    Resolved,
    Ignored,
    RequiresManualReview,
}
