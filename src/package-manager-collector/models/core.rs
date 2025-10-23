//! Core Package Data Models
//!
//! This module defines the core data structures for package information,
//! versions, metadata, health, and statistics.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Unique package identifier
    pub id: String,
    /// Package name
    pub name: String,
    /// Package description
    pub description: Option<String>,
    /// Package version
    pub version: String,
    /// Package license
    pub license: Option<String>,
    /// Package homepage URL
    pub homepage: Option<String>,
    /// Package repository URL
    pub repository: Option<String>,
    /// Package keywords/tags
    pub keywords: Vec<String>,
    /// Package authors
    pub authors: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Package version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    /// Version identifier
    pub id: String,
    /// Package ID this version belongs to
    pub package_id: String,
    /// Version number
    pub version: String,
    /// Version description
    pub description: Option<String>,
    /// Release date
    pub released_at: DateTime<Utc>,
    /// Whether this is a pre-release
    pub is_prerelease: bool,
    /// Whether this is the latest version
    pub is_latest: bool,
    /// Version-specific metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Package metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Metadata ID
    pub id: String,
    /// Package ID this metadata belongs to
    pub package_id: String,
    /// Package size in bytes
    pub size: Option<u64>,
    /// Package file checksum
    pub checksum: Option<String>,
    /// Package dependencies
    pub dependencies: Vec<PackageDependency>,
    /// Package development dependencies
    pub dev_dependencies: Vec<PackageDependency>,
    /// Package peer dependencies
    pub peer_dependencies: Vec<PackageDependency>,
    /// Package scripts (build, test, etc.)
    pub scripts: HashMap<String, String>,
    /// Package engines (Node.js version, etc.)
    pub engines: HashMap<String, String>,
    /// Package main entry point
    pub main: Option<String>,
    /// Package types/module types
    pub types: Option<String>,
    /// Additional metadata
    pub additional: HashMap<String, serde_json::Value>,
}

/// Package dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    /// Dependency name
    pub name: String,
    /// Dependency version constraint
    pub version_constraint: String,
    /// Whether this is an optional dependency
    pub is_optional: bool,
    /// Dependency type (runtime, dev, peer, etc.)
    pub dependency_type: DependencyType,
}

/// Types of package dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Runtime,
    Development,
    Peer,
    Optional,
    Bundle,
}

/// Package health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageHealth {
    /// Health ID
    pub id: String,
    /// Package ID this health belongs to
    pub package_id: String,
    /// Overall health score (0.0 - 1.0)
    pub overall_score: Decimal,
    /// Maintenance score (0.0 - 1.0)
    pub maintenance_score: Decimal,
    /// Security score (0.0 - 1.0)
    pub security_score: Decimal,
    /// Community score (0.0 - 1.0)
    pub community_score: Decimal,
    /// Code quality score (0.0 - 1.0)
    pub code_quality_score: Decimal,
    /// Number of known vulnerabilities
    pub vulnerability_count: u32,
    /// Last security audit date
    pub last_security_audit: Option<DateTime<Utc>>,
    /// Health assessment timestamp
    pub assessed_at: DateTime<Utc>,
}

/// Package statistics and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageStatistics {
    /// Statistics ID
    pub id: String,
    /// Package ID this statistics belongs to
    pub package_id: String,
    /// Total downloads
    pub total_downloads: u64,
    /// Downloads in the last 30 days
    pub downloads_last_30_days: u64,
    /// Downloads in the last 7 days
    pub downloads_last_7_days: u64,
    /// Downloads today
    pub downloads_today: u64,
    /// Number of stars (for GitHub repositories)
    pub stars: Option<u32>,
    /// Number of forks (for GitHub repositories)
    pub forks: Option<u32>,
    /// Number of watchers (for GitHub repositories)
    pub watchers: Option<u32>,
    /// Number of open issues
    pub open_issues: Option<u32>,
    /// Number of open pull requests
    pub open_pull_requests: Option<u32>,
    /// Statistics collection timestamp
    pub collected_at: DateTime<Utc>,
}
