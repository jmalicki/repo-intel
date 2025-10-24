//! Schema registry management
//!
//! This module provides schema registry functionality including schema storage,
//! versioning, metadata management, and schema discovery.

use crate::error::{Error, Result};
use crate::logging::Logger;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

/// Schema registry
pub struct SchemaRegistry {
    logger: Logger,
    schemas: HashMap<String, SchemaMetadata>,
    versions: HashMap<String, Vec<SchemaVersion>>,
    tags: HashMap<String, Vec<String>>,
}

/// Schema metadata
#[derive(Debug, Clone)]
pub struct SchemaMetadata {
    pub name: String,
    pub version: String,
    pub schema: Value,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: Option<String>,
    pub dependencies: Vec<String>,
    pub is_deprecated: bool,
}

/// Schema version information
#[derive(Debug, Clone)]
pub struct SchemaVersion {
    pub version: String,
    pub schema: Value,
    pub created_at: DateTime<Utc>,
    pub is_current: bool,
    pub changelog: Option<String>,
}

/// Schema registry error
#[derive(Debug, Clone)]
pub struct SchemaRegistryError {
    pub error_type: RegistryErrorType,
    pub message: String,
    pub suggestion: Option<String>,
}

/// Types of registry errors
#[derive(Debug, Clone, PartialEq)]
pub enum RegistryErrorType {
    SchemaNotFound,
    VersionNotFound,
    DuplicateSchema,
    InvalidSchema,
    DependencyNotFound,
    CircularDependency,
    AccessDenied,
    ValidationFailed,
}

impl SchemaRegistry {
    /// Create a new schema registry
    pub fn new() -> Self {
        Self {
            logger: Logger::new("schema_registry"),
            schemas: HashMap::new(),
            versions: HashMap::new(),
            tags: HashMap::new(),
        }
    }

    /// Register a new schema
    pub fn register_schema(&mut self, metadata: SchemaMetadata) -> Result<()> {
        self.logger.info(&format!(
            "Registering schema: {} v{}",
            metadata.name, metadata.version
        ));

        // Validate schema
        self.validate_schema(&metadata.schema)?;

        // Check for duplicates
        let schema_key = format!("{}:{}", metadata.name, metadata.version);
        if self.schemas.contains_key(&schema_key) {
            return Err(Error::validation(format!(
                "Schema '{}' version '{}' already exists",
                metadata.name, metadata.version
            )));
        }

        // Check dependencies
        self.validate_dependencies(&metadata.dependencies)?;

        // Register schema
        self.schemas.insert(schema_key.clone(), metadata.clone());

        // Add to version history
        let version = SchemaVersion {
            version: metadata.version.clone(),
            schema: metadata.schema.clone(),
            created_at: metadata.created_at,
            is_current: true,
            changelog: None,
        };

        self.versions
            .entry(metadata.name.clone())
            .or_insert_with(Vec::new)
            .push(version);

        // Update tags
        for tag in &metadata.tags {
            self.tags
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(metadata.name.clone());
        }

        self.logger.info(&format!(
            "Schema '{}' v{} registered successfully",
            metadata.name, metadata.version
        ));
        Ok(())
    }

    /// Get a schema by name and version
    pub fn get_schema(&self, name: &str, version: &str) -> Result<&SchemaMetadata> {
        let schema_key = format!("{}:{}", name, version);
        self.schemas.get(&schema_key).ok_or_else(|| {
            Error::validation(format!("Schema '{}' version '{}' not found", name, version))
        })
    }

    /// Get the latest version of a schema
    pub fn get_latest_schema(&self, name: &str) -> Result<&SchemaMetadata> {
        if let Some(versions) = self.versions.get(name) {
            if let Some(latest) = versions.last() {
                return self.get_schema(name, &latest.version);
            }
        }
        Err(Error::validation(format!("Schema '{}' not found", name)))
    }

    /// Get all versions of a schema
    pub fn get_schema_versions(&self, name: &str) -> Result<&[SchemaVersion]> {
        self.versions
            .get(name)
            .map(|v| v.as_slice())
            .ok_or_else(|| Error::validation(format!("Schema '{}' not found", name)))
    }

    /// Get schemas by tag
    pub fn get_schemas_by_tag(&self, tag: &str) -> Vec<&SchemaMetadata> {
        if let Some(schema_names) = self.tags.get(tag) {
            schema_names
                .iter()
                .filter_map(|name| self.get_latest_schema(name).ok())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Search schemas by name pattern
    pub fn search_schemas(&self, pattern: &str) -> Vec<&SchemaMetadata> {
        self.schemas
            .values()
            .filter(|schema| schema.name.contains(pattern))
            .collect()
    }

    /// List all schema names
    pub fn list_schemas(&self) -> Vec<String> {
        self.schemas
            .keys()
            .map(|key| key.split(':').next().unwrap_or(key).to_string())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Update a schema
    pub fn update_schema(&mut self, name: &str, version: &str, new_schema: Value) -> Result<()> {
        self.logger
            .info(&format!("Updating schema: {} v{}", name, version));

        // Validate new schema
        self.validate_schema(&new_schema)?;

        // Get existing schema
        let schema_key = format!("{}:{}", name, version);
        let mut metadata = self
            .schemas
            .get(&schema_key)
            .ok_or_else(|| {
                Error::validation(format!("Schema '{}' version '{}' not found", name, version))
            })?
            .clone();

        // Update schema
        metadata.schema = new_schema;
        metadata.updated_at = Utc::now();

        // Replace in registry
        self.schemas.insert(schema_key, metadata);

        self.logger.info(&format!(
            "Schema '{}' v{} updated successfully",
            name, version
        ));
        Ok(())
    }

    /// Deprecate a schema
    pub fn deprecate_schema(&mut self, name: &str, version: &str) -> Result<()> {
        self.logger
            .info(&format!("Deprecating schema: {} v{}", name, version));

        let schema_key = format!("{}:{}", name, version);
        if let Some(metadata) = self.schemas.get_mut(&schema_key) {
            metadata.is_deprecated = true;
            metadata.updated_at = Utc::now();
            self.logger.info(&format!(
                "Schema '{}' v{} deprecated successfully",
                name, version
            ));
            Ok(())
        } else {
            Err(Error::validation(format!(
                "Schema '{}' version '{}' not found",
                name, version
            )))
        }
    }

    /// Remove a schema
    pub fn remove_schema(&mut self, name: &str, version: &str) -> Result<SchemaMetadata> {
        self.logger
            .info(&format!("Removing schema: {} v{}", name, version));

        let schema_key = format!("{}:{}", name, version);
        if let Some(metadata) = self.schemas.remove(&schema_key) {
            // Remove from versions
            if let Some(versions) = self.versions.get_mut(name) {
                versions.retain(|v| v.version != version);
                if versions.is_empty() {
                    self.versions.remove(name);
                }
            }

            // Remove from tags
            for tag in &metadata.tags {
                if let Some(schema_names) = self.tags.get_mut(tag) {
                    schema_names.retain(|n| n != name);
                    if schema_names.is_empty() {
                        self.tags.remove(tag);
                    }
                }
            }

            self.logger.info(&format!(
                "Schema '{}' v{} removed successfully",
                name, version
            ));
            Ok(metadata)
        } else {
            Err(Error::validation(format!(
                "Schema '{}' version '{}' not found",
                name, version
            )))
        }
    }

    /// Validate a schema structure
    fn validate_schema(&self, schema: &Value) -> Result<()> {
        if let Some(obj) = schema.as_object() {
            // Check for required schema properties
            if !obj.contains_key("type") && !obj.contains_key("$ref") && !obj.contains_key("allOf")
            {
                return Err(Error::validation(
                    "Schema must have 'type', '$ref', or 'allOf' property".to_string(),
                ));
            }

            // Validate schema structure recursively
            self.validate_schema_recursive(schema)?;
        } else {
            return Err(Error::validation(
                "Schema must be a JSON object".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate schema structure recursively
    fn validate_schema_recursive(&self, schema: &Value) -> Result<()> {
        if let Some(obj) = schema.as_object() {
            // Check for valid properties
            let valid_properties = [
                "type",
                "$ref",
                "allOf",
                "anyOf",
                "oneOf",
                "not",
                "properties",
                "additionalProperties",
                "items",
                "required",
                "minLength",
                "maxLength",
                "pattern",
                "minimum",
                "maximum",
                "minItems",
                "maxItems",
                "uniqueItems",
                "enum",
                "const",
                "title",
                "description",
                "default",
                "examples",
            ];

            for (key, _) in obj {
                if !valid_properties.contains(&key.as_str()) {
                    self.logger
                        .warn(&format!("Unknown schema property: {}", key));
                }
            }

            // Validate nested schemas
            for (key, value) in obj {
                match key.as_str() {
                    "properties" | "additionalProperties" | "items" => {
                        if let Some(nested_obj) = value.as_object() {
                            for (_, nested_schema) in nested_obj {
                                self.validate_schema_recursive(nested_schema)?;
                            }
                        } else if value.is_object() {
                            self.validate_schema_recursive(value)?;
                        }
                    }
                    "allOf" | "anyOf" | "oneOf" => {
                        if let Some(array) = value.as_array() {
                            for item in array {
                                self.validate_schema_recursive(item)?;
                            }
                        }
                    }
                    "not" => {
                        self.validate_schema_recursive(value)?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Validate schema dependencies
    fn validate_dependencies(&self, dependencies: &[String]) -> Result<()> {
        for dep in dependencies {
            if !self
                .schemas
                .keys()
                .any(|key| key.starts_with(&format!("{}:", dep)))
            {
                return Err(Error::validation(format!("Dependency '{}' not found", dep)));
            }
        }
        Ok(())
    }

    /// Get registry statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        let total_schemas = self.schemas.len();
        let total_versions = self.versions.values().map(|v| v.len()).sum();
        let deprecated_count = self.schemas.values().filter(|s| s.is_deprecated).count();
        let tag_count = self.tags.len();

        RegistryStatistics {
            total_schemas,
            total_versions,
            deprecated_count,
            tag_count,
            schema_names: self.list_schemas(),
        }
    }

    /// Export registry to JSON
    pub fn export_registry(&self) -> Result<Value> {
        let schemas_json: Vec<Value> = self
            .schemas
            .values()
            .map(|schema| {
                serde_json::json!({
                    "name": schema.name,
                    "version": schema.version,
                    "description": schema.description,
                    "tags": schema.tags,
                    "created_at": schema.created_at.to_rfc3339(),
                    "updated_at": schema.updated_at.to_rfc3339(),
                    "author": schema.author,
                    "dependencies": schema.dependencies,
                    "is_deprecated": schema.is_deprecated
                })
            })
            .collect();

        Ok(serde_json::json!({
            "schemas": schemas_json,
            "statistics": self.get_statistics()
        }))
    }

    /// Clear all schemas
    pub fn clear_registry(&mut self) {
        self.logger.info("Clearing schema registry");
        self.schemas.clear();
        self.versions.clear();
        self.tags.clear();
    }
}

/// Registry statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct RegistryStatistics {
    pub total_schemas: usize,
    pub total_versions: usize,
    pub deprecated_count: usize,
    pub tag_count: usize,
    pub schema_names: Vec<String>,
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}
