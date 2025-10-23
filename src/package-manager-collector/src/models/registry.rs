//! Registry-Specific Package Data Models
//!
//! This module contains data models specific to each package manager registry,
//! including NPM, PyPI, Crates.io, Maven Central, Go Modules, RubyGems, Packagist, and NuGet.

// Note: DateTime, Utc, and Decimal imports are reserved for future use in registry-specific timestamps and metrics
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NPM-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmPackageData {
    /// NPM data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// NPM package scope
    pub scope: Option<String>,
    /// NPM package scripts
    pub scripts: HashMap<String, String>,
    /// NPM package engines
    pub engines: HashMap<String, String>,
    /// NPM package bin commands
    pub bin: HashMap<String, String>,
    /// NPM package main entry point
    pub main: Option<String>,
    /// NPM package types/module types
    pub types: Option<String>,
    /// NPM package exports
    pub exports: Option<serde_json::Value>,
    /// NPM package repository information
    pub repository: Option<NpmRepository>,
    /// NPM package bugs information
    pub bugs: Option<NpmBugs>,
    /// NPM package funding information
    pub funding: Option<serde_json::Value>,
}

/// NPM repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmRepository {
    /// Repository type (git, svn, etc.)
    pub r#type: String,
    /// Repository URL
    pub url: String,
    /// Repository directory
    pub directory: Option<String>,
}

/// NPM bugs information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpmBugs {
    /// Bugs URL
    pub url: String,
    /// Bugs email
    pub email: Option<String>,
}

/// PyPI-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PypiPackageData {
    /// PyPI data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// PyPI package classifiers
    pub classifiers: Vec<String>,
    /// PyPI package keywords
    pub keywords: Vec<String>,
    /// PyPI package project URLs
    pub project_urls: HashMap<String, String>,
    /// PyPI package requires Python version
    pub requires_python: Option<String>,
    /// PyPI package platform information
    pub platform: Vec<String>,
    /// PyPI package supported platforms
    pub supported_platform: Vec<String>,
    /// PyPI package license classifier
    pub license_classifier: Option<String>,
    /// PyPI package summary
    pub summary: Option<String>,
}

/// Crates.io-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CratesPackageData {
    /// Crates data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// Crate features
    pub features: HashMap<String, Vec<String>>,
    /// Crate categories
    pub categories: Vec<String>,
    /// Crate keywords
    pub keywords: Vec<String>,
    /// Crate license file
    pub license_file: Option<String>,
    /// Crate documentation URL
    pub documentation: Option<String>,
    /// Crate homepage URL
    pub homepage: Option<String>,
    /// Crate repository URL
    pub repository: Option<String>,
    /// Crate readme file
    pub readme: Option<String>,
    /// Crate badges
    pub badges: Option<serde_json::Value>,
}

/// Maven Central-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavenPackageData {
    /// Maven data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// Maven group ID
    pub group_id: String,
    /// Maven artifact ID
    pub artifact_id: String,
    /// Maven packaging type
    pub packaging: String,
    /// Maven parent information
    pub parent: Option<MavenParent>,
    /// Maven properties
    pub properties: HashMap<String, String>,
    /// Maven profiles
    pub profiles: Vec<MavenProfile>,
    /// Maven dependencies management
    pub dependency_management: Option<serde_json::Value>,
}

/// Maven parent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavenParent {
    /// Parent group ID
    pub group_id: String,
    /// Parent artifact ID
    pub artifact_id: String,
    /// Parent version
    pub version: String,
}

/// Maven profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavenProfile {
    /// Profile ID
    pub id: String,
    /// Profile activation conditions
    pub activation: Option<serde_json::Value>,
    /// Profile properties
    pub properties: HashMap<String, String>,
}

/// Go Modules-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoPackageData {
    /// Go data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// Go module path
    pub module_path: String,
    /// Go module version
    pub go_version: Option<String>,
    /// Go module checksum
    pub checksum: Option<String>,
    /// Go module indirect dependencies
    pub indirect: bool,
    /// Go module replace directives
    pub replace: Option<serde_json::Value>,
    /// Go module exclude directives
    pub exclude: Vec<String>,
}

/// RubyGems-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubyGemsPackageData {
    /// RubyGems data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// Gem specification
    pub spec: Option<serde_json::Value>,
    /// Gem platform
    pub platform: Option<String>,
    /// Gem required Ruby version
    pub required_ruby_version: Option<String>,
    /// Gem required RubyGems version
    pub required_rubygems_version: Option<String>,
    /// Gem dependencies
    pub dependencies: Vec<RubyGemsDependency>,
    /// Gem development dependencies
    pub development_dependencies: Vec<RubyGemsDependency>,
}

/// RubyGems dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubyGemsDependency {
    /// Dependency name
    pub name: String,
    /// Dependency version requirement
    pub requirement: String,
    /// Dependency type
    pub r#type: String,
}

/// Packagist-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagistPackageData {
    /// Packagist data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// Composer package type
    pub package_type: String,
    /// Composer autoload configuration
    pub autoload: Option<serde_json::Value>,
    /// Composer autoload-dev configuration
    pub autoload_dev: Option<serde_json::Value>,
    /// Composer scripts
    pub scripts: HashMap<String, Vec<String>>,
    /// Composer config
    pub config: HashMap<String, serde_json::Value>,
    /// Composer minimum stability
    pub minimum_stability: Option<String>,
    /// Composer prefer-stable flag
    pub prefer_stable: bool,
}

/// NuGet-specific package data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuGetPackageData {
    /// NuGet data ID
    pub id: String,
    /// Package ID this data belongs to
    pub package_id: String,
    /// NuGet package ID
    pub nuget_id: String,
    /// NuGet package authors
    pub authors: Vec<String>,
    /// NuGet package owners
    pub owners: Vec<String>,
    /// NuGet package project URL
    pub project_url: Option<String>,
    /// NuGet package icon URL
    pub icon_url: Option<String>,
    /// NuGet package license URL
    pub license_url: Option<String>,
    /// NuGet package tags
    pub tags: Vec<String>,
    /// NuGet package summary
    pub summary: Option<String>,
    /// NuGet package release notes
    pub release_notes: Option<String>,
    /// NuGet package copyright
    pub copyright: Option<String>,
    /// NuGet package language
    pub language: Option<String>,
}
