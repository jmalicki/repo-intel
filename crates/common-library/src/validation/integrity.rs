//! Data integrity checks
//!
//! This module provides data integrity validation including checksum verification,
//! data consistency checks, and referential integrity validation.

use crate::error::Result;
use crate::logging::Logger;
use serde_json::Value;
use std::collections::HashMap;

/// Data integrity check result
#[derive(Debug, Clone)]
pub struct IntegrityResult {
    pub is_valid: bool,
    pub violations: Vec<IntegrityViolation>,
    pub checksum_valid: bool,
    pub consistency_score: f64,
    pub check_time_ms: u64,
}

/// Data integrity violation
#[derive(Debug, Clone)]
pub struct IntegrityViolation {
    pub violation_type: IntegrityViolationType,
    pub path: String,
    pub message: String,
    pub severity: ViolationSeverity,
    pub suggestion: Option<String>,
}

/// Types of integrity violations
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrityViolationType {
    ChecksumMismatch,
    ReferentialIntegrityViolation,
    DataConsistencyViolation,
    ConstraintViolation,
    FormatViolation,
    DuplicateKeyViolation,
    NullConstraintViolation,
    RangeViolation,
}

/// Severity levels for violations
#[derive(Debug, Clone, PartialEq)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Data integrity checker
pub struct DataIntegrityChecker {
    logger: Logger,
    checksums: HashMap<String, String>,
    constraints: Vec<IntegrityConstraint>,
}

/// Integrity constraint definition
#[derive(Debug, Clone)]
pub struct IntegrityConstraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub path: String,
    pub value: Option<Value>,
    pub severity: ViolationSeverity,
}

/// Types of integrity constraints
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    NotNull,
    Unique,
    ForeignKey,
    Range,
    Format,
    Custom,
}

impl DataIntegrityChecker {
    /// Create a new data integrity checker
    pub fn new() -> Self {
        Self {
            logger: Logger::new("data_integrity_checker"),
            checksums: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Add a checksum for data integrity verification
    pub fn add_checksum(&mut self, key: &str, checksum: &str) {
        self.logger
            .info(&format!("Adding checksum for key: {}", key));
        self.checksums.insert(key.to_string(), checksum.to_string());
    }

    /// Add an integrity constraint
    pub fn add_constraint(&mut self, constraint: IntegrityConstraint) {
        self.logger
            .info(&format!("Adding constraint: {}", constraint.name));
        self.constraints.push(constraint);
    }

    /// Check data integrity
    pub fn check_integrity(&self, data: &Value, key: &str) -> Result<IntegrityResult> {
        let start_time = std::time::Instant::now();

        self.logger
            .info(&format!("Checking data integrity for key: {}", key));

        let mut violations = Vec::new();
        let mut checksum_valid = true;

        // Check checksum if available
        if let Some(expected_checksum) = self.checksums.get(key) {
            let actual_checksum = self.calculate_checksum(data);
            if actual_checksum != *expected_checksum {
                checksum_valid = false;
                violations.push(IntegrityViolation {
                    violation_type: IntegrityViolationType::ChecksumMismatch,
                    path: "root".to_string(),
                    message: format!(
                        "Checksum mismatch: expected {}, got {}",
                        expected_checksum, actual_checksum
                    ),
                    severity: ViolationSeverity::Critical,
                    suggestion: Some("Verify data source and recalculate checksum".to_string()),
                });
            }
        }

        // Check constraints
        for constraint in &self.constraints {
            if let Some(violation) = self.check_constraint(data, constraint) {
                violations.push(violation);
            }
        }

        // Check data consistency
        let consistency_violations = self.check_data_consistency(data);
        violations.extend(consistency_violations);

        let check_time = start_time.elapsed().as_millis() as u64;
        let is_valid = violations.is_empty();
        let consistency_score = self.calculate_consistency_score(&violations);

        let result = IntegrityResult {
            is_valid,
            violations,
            checksum_valid,
            consistency_score,
            check_time_ms: check_time,
        };

        self.logger.info(&format!(
            "Integrity check completed: {} violations, consistency score: {:.2}",
            result.violations.len(),
            consistency_score
        ));

        Ok(result)
    }

    /// Calculate checksum for data
    fn calculate_checksum(&self, data: &Value) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Check a specific constraint
    fn check_constraint(
        &self,
        data: &Value,
        constraint: &IntegrityConstraint,
    ) -> Option<IntegrityViolation> {
        match constraint.constraint_type {
            ConstraintType::NotNull => self.check_not_null_constraint(data, constraint),
            ConstraintType::Unique => self.check_unique_constraint(data, constraint),
            ConstraintType::ForeignKey => self.check_foreign_key_constraint(data, constraint),
            ConstraintType::Range => self.check_range_constraint(data, constraint),
            ConstraintType::Format => self.check_format_constraint(data, constraint),
            ConstraintType::Custom => self.check_custom_constraint(data, constraint),
        }
    }

    /// Check not null constraint
    fn check_not_null_constraint(
        &self,
        data: &Value,
        constraint: &IntegrityConstraint,
    ) -> Option<IntegrityViolation> {
        if let Some(value) = self.get_value_by_path(data, &constraint.path) {
            if value.is_null() {
                return Some(IntegrityViolation {
                    violation_type: IntegrityViolationType::NullConstraintViolation,
                    path: constraint.path.clone(),
                    message: format!("Field '{}' cannot be null", constraint.path),
                    severity: constraint.severity.clone(),
                    suggestion: Some("Provide a non-null value for this field".to_string()),
                });
            }
        }
        None
    }

    /// Check unique constraint
    fn check_unique_constraint(
        &self,
        data: &Value,
        constraint: &IntegrityConstraint,
    ) -> Option<IntegrityViolation> {
        // This is a simplified implementation
        // In a real system, you'd check against a database or index
        if let Some(value) = self.get_value_by_path(data, &constraint.path) {
            if value.is_null() {
                return Some(IntegrityViolation {
                    violation_type: IntegrityViolationType::DuplicateKeyViolation,
                    path: constraint.path.clone(),
                    message: format!("Field '{}' must be unique", constraint.path),
                    severity: constraint.severity.clone(),
                    suggestion: Some("Ensure the value is unique across all records".to_string()),
                });
            }
        }
        None
    }

    /// Check foreign key constraint
    fn check_foreign_key_constraint(
        &self,
        data: &Value,
        constraint: &IntegrityConstraint,
    ) -> Option<IntegrityViolation> {
        if let Some(value) = self.get_value_by_path(data, &constraint.path) {
            if let Some(referenced_value) = &constraint.value {
                if *value != *referenced_value {
                    return Some(IntegrityViolation {
                        violation_type: IntegrityViolationType::ReferentialIntegrityViolation,
                        path: constraint.path.clone(),
                        message: format!(
                            "Foreign key '{}' references non-existent value",
                            constraint.path
                        ),
                        severity: constraint.severity.clone(),
                        suggestion: Some("Ensure the referenced value exists".to_string()),
                    });
                }
            }
        }
        None
    }

    /// Check range constraint
    fn check_range_constraint(
        &self,
        data: &Value,
        constraint: &IntegrityConstraint,
    ) -> Option<IntegrityViolation> {
        if let Some(value) = self.get_value_by_path(data, &constraint.path) {
            if let Some(num_value) = value.as_f64() {
                if let Some(constraint_value) = &constraint.value {
                    if let Some(min_value) = constraint_value.get("min").and_then(|v| v.as_f64()) {
                        if num_value < min_value {
                            return Some(IntegrityViolation {
                                violation_type: IntegrityViolationType::RangeViolation,
                                path: constraint.path.clone(),
                                message: format!(
                                    "Value {} is below minimum {}",
                                    num_value, min_value
                                ),
                                severity: constraint.severity.clone(),
                                suggestion: Some(
                                    "Increase the value to meet the minimum requirement"
                                        .to_string(),
                                ),
                            });
                        }
                    }
                    if let Some(max_value) = constraint_value.get("max").and_then(|v| v.as_f64()) {
                        if num_value > max_value {
                            return Some(IntegrityViolation {
                                violation_type: IntegrityViolationType::RangeViolation,
                                path: constraint.path.clone(),
                                message: format!(
                                    "Value {} exceeds maximum {}",
                                    num_value, max_value
                                ),
                                severity: constraint.severity.clone(),
                                suggestion: Some(
                                    "Decrease the value to meet the maximum requirement"
                                        .to_string(),
                                ),
                            });
                        }
                    }
                }
            }
        }
        None
    }

    /// Check format constraint
    fn check_format_constraint(
        &self,
        data: &Value,
        constraint: &IntegrityConstraint,
    ) -> Option<IntegrityViolation> {
        if let Some(value) = self.get_value_by_path(data, &constraint.path) {
            if let Some(str_value) = value.as_str() {
                if let Some(format_value) = &constraint.value {
                    if let Some(format_str) = format_value.as_str() {
                        if !self.validate_format(str_value, format_str) {
                            return Some(IntegrityViolation {
                                violation_type: IntegrityViolationType::FormatViolation,
                                path: constraint.path.clone(),
                                message: format!(
                                    "Value '{}' does not match format '{}'",
                                    str_value, format_str
                                ),
                                severity: constraint.severity.clone(),
                                suggestion: Some(format!(
                                    "Update the value to match format '{}'",
                                    format_str
                                )),
                            });
                        }
                    }
                }
            }
        }
        None
    }

    /// Check custom constraint
    fn check_custom_constraint(
        &self,
        _data: &Value,
        _constraint: &IntegrityConstraint,
    ) -> Option<IntegrityViolation> {
        // This is a placeholder for custom constraint logic
        // In a real implementation, you'd have a way to define custom validation functions
        None
    }

    /// Validate format using regex patterns
    fn validate_format(&self, value: &str, format: &str) -> bool {
        match format {
            "email" => self.is_valid_email(value),
            "url" => self.is_valid_url(value),
            "date" => self.is_valid_date(value),
            "uuid" => self.is_valid_uuid(value),
            _ => true, // Unknown format, assume valid
        }
    }

    /// Check if string is a valid email
    fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    /// Check if string is a valid URL
    fn is_valid_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    /// Check if string is a valid date
    fn is_valid_date(&self, date: &str) -> bool {
        chrono::DateTime::parse_from_rfc3339(date).is_ok()
    }

    /// Check if string is a valid UUID
    fn is_valid_uuid(&self, uuid: &str) -> bool {
        uuid::Uuid::parse_str(uuid).is_ok()
    }

    /// Check data consistency
    fn check_data_consistency(&self, data: &Value) -> Vec<IntegrityViolation> {
        let mut violations = Vec::new();

        // Check for duplicate keys in objects
        if let Some(obj) = data.as_object() {
            let mut seen_keys = std::collections::HashSet::new();
            for key in obj.keys() {
                if !seen_keys.insert(key) {
                    violations.push(IntegrityViolation {
                        violation_type: IntegrityViolationType::DuplicateKeyViolation,
                        path: "root".to_string(),
                        message: format!("Duplicate key found: {}", key),
                        severity: ViolationSeverity::Medium,
                        suggestion: Some("Remove duplicate keys".to_string()),
                    });
                }
            }
        }

        // Check for circular references (simplified)
        if self.has_circular_reference(data) {
            violations.push(IntegrityViolation {
                violation_type: IntegrityViolationType::DataConsistencyViolation,
                path: "root".to_string(),
                message: "Circular reference detected".to_string(),
                severity: ViolationSeverity::High,
                suggestion: Some("Remove circular references".to_string()),
            });
        }

        violations
    }

    /// Check for circular references (simplified implementation)
    fn has_circular_reference(&self, _data: &Value) -> bool {
        // This is a simplified implementation
        // In a real system, you'd need more sophisticated cycle detection
        false
    }

    /// Calculate consistency score based on violations
    fn calculate_consistency_score(&self, violations: &[IntegrityViolation]) -> f64 {
        if violations.is_empty() {
            return 100.0;
        }

        let mut score: f64 = 100.0;
        for violation in violations {
            match violation.severity {
                ViolationSeverity::Low => score -= 5.0,
                ViolationSeverity::Medium => score -= 15.0,
                ViolationSeverity::High => score -= 30.0,
                ViolationSeverity::Critical => score -= 50.0,
            }
        }

        score.max(0.0)
    }

    /// Get value by path (simplified implementation)
    fn get_value_by_path<'a>(&self, data: &'a Value, path: &str) -> Option<&'a Value> {
        if path == "root" {
            return Some(data);
        }

        // Simple path resolution for nested objects
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = data;

        for part in parts {
            if let Some(obj) = current.as_object() {
                if let Some(value) = obj.get(part) {
                    current = value;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(current)
    }

    /// Get all registered checksums
    pub fn get_checksums(&self) -> &HashMap<String, String> {
        &self.checksums
    }

    /// Get all constraints
    pub fn get_constraints(&self) -> &[IntegrityConstraint] {
        &self.constraints
    }

    /// Clear all checksums
    pub fn clear_checksums(&mut self) {
        self.logger.info("Clearing all checksums");
        self.checksums.clear();
    }

    /// Clear all constraints
    pub fn clear_constraints(&mut self) {
        self.logger.info("Clearing all constraints");
        self.constraints.clear();
    }
}

impl Default for DataIntegrityChecker {
    fn default() -> Self {
        Self::new()
    }
}
