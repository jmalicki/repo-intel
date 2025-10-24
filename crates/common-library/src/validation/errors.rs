//! Validation error reporting and suggestions
//!
//! This module provides comprehensive error reporting for validation failures
//! including error categorization, suggestions, and error aggregation.

use crate::error::Result;
use crate::logging::Logger;
use serde_json::Value;
use std::collections::HashMap;

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub path: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub suggestion: Option<ValidationSuggestion>,
    pub context: Option<Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Types of validation errors
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ValidationErrorType {
    SchemaValidation,
    TypeValidation,
    IntegrityViolation,
    ConstraintViolation,
    FormatError,
    RequiredFieldMissing,
    InvalidValue,
    CustomValidation,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Validation suggestion
#[derive(Debug, Clone)]
pub struct ValidationSuggestion {
    pub title: String,
    pub description: String,
    pub action: SuggestionAction,
    pub confidence: f64,
}

/// Types of suggestion actions
#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionAction {
    Fix,
    Replace,
    Add,
    Remove,
    Convert,
    Validate,
    Custom(String),
}

/// Validation error reporter
pub struct ValidationErrorReporter {
    logger: Logger,
    errors: Vec<ValidationError>,
    error_patterns: HashMap<String, ErrorPattern>,
    suggestion_engine: SuggestionEngine,
}

/// Error pattern for automatic error detection
#[derive(Debug, Clone)]
pub struct ErrorPattern {
    pub pattern: String,
    pub error_type: ValidationErrorType,
    pub severity: ErrorSeverity,
    pub suggestion_template: String,
}

/// Suggestion engine for generating helpful suggestions
#[derive(Debug, Clone)]
pub struct SuggestionEngine {
    pub patterns: HashMap<String, SuggestionPattern>,
    pub confidence_threshold: f64,
}

/// Suggestion pattern for generating suggestions
#[derive(Debug, Clone)]
pub struct SuggestionPattern {
    pub pattern: String,
    pub suggestion_template: String,
    pub confidence: f64,
    pub action: SuggestionAction,
}

impl ValidationErrorReporter {
    /// Create a new validation error reporter
    pub fn new() -> Self {
        Self {
            logger: Logger::new("validation_error_reporter"),
            errors: Vec::new(),
            error_patterns: HashMap::new(),
            suggestion_engine: SuggestionEngine::new(),
        }
    }

    /// Add a validation error
    pub fn add_error(&mut self, error: ValidationError) {
        self.logger
            .info(&format!("Adding validation error: {}", error.message));
        self.errors.push(error);
    }

    /// Add multiple validation errors
    pub fn add_errors(&mut self, errors: Vec<ValidationError>) {
        self.logger
            .info(&format!("Adding {} validation errors", errors.len()));
        self.errors.extend(errors);
    }

    /// Get all validation errors
    pub fn get_errors(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Get errors by type
    pub fn get_errors_by_type(&self, error_type: &ValidationErrorType) -> Vec<&ValidationError> {
        self.errors
            .iter()
            .filter(|error| &error.error_type == error_type)
            .collect()
    }

    /// Get errors by severity
    pub fn get_errors_by_severity(&self, severity: &ErrorSeverity) -> Vec<&ValidationError> {
        self.errors
            .iter()
            .filter(|error| &error.severity == severity)
            .collect()
    }

    /// Get errors by path
    pub fn get_errors_by_path(&self, path: &str) -> Vec<&ValidationError> {
        self.errors
            .iter()
            .filter(|error| error.path.contains(path))
            .collect()
    }

    /// Generate error summary
    pub fn generate_summary(&self) -> ErrorSummary {
        let total_errors = self.errors.len();
        let critical_errors = self.get_errors_by_severity(&ErrorSeverity::Critical).len();
        let high_errors = self.get_errors_by_severity(&ErrorSeverity::Error).len();
        let warnings = self.get_errors_by_severity(&ErrorSeverity::Warning).len();
        let info = self.get_errors_by_severity(&ErrorSeverity::Info).len();

        let error_types = self.get_error_type_counts();
        let severity_distribution = self.get_severity_distribution();

        ErrorSummary {
            total_errors,
            critical_errors,
            high_errors,
            warnings,
            info,
            error_types,
            severity_distribution,
            has_errors: total_errors > 0,
            has_critical_errors: critical_errors > 0,
        }
    }

    /// Get error type counts
    fn get_error_type_counts(&self) -> HashMap<ValidationErrorType, usize> {
        let mut counts = HashMap::new();
        for error in &self.errors {
            *counts.entry(error.error_type.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Get severity distribution
    fn get_severity_distribution(&self) -> HashMap<ErrorSeverity, usize> {
        let mut distribution = HashMap::new();
        for error in &self.errors {
            *distribution.entry(error.severity.clone()).or_insert(0) += 1;
        }
        distribution
    }

    /// Generate suggestions for errors
    pub fn generate_suggestions(&self) -> Vec<ValidationSuggestion> {
        let mut suggestions = Vec::new();

        for error in &self.errors {
            if let Some(suggestion) = self.suggestion_engine.generate_suggestion(error) {
                suggestions.push(suggestion);
            }
        }

        // Remove duplicate suggestions
        suggestions.sort_by(|a, b| a.title.cmp(&b.title));
        suggestions.dedup_by(|a, b| a.title == b.title);

        suggestions
    }

    /// Clear all errors
    pub fn clear_errors(&mut self) {
        self.logger.info("Clearing all validation errors");
        self.errors.clear();
    }

    /// Export errors to JSON
    pub fn export_errors(&self) -> Result<Value> {
        let errors_json: Vec<Value> = self
            .errors
            .iter()
            .map(|error| {
                serde_json::json!({
                    "error_type": format!("{:?}", error.error_type),
                    "path": error.path,
                    "message": error.message,
                    "severity": format!("{:?}", error.severity),
                    "suggestion": error.suggestion.as_ref().map(|s| serde_json::json!({
                        "title": s.title,
                        "description": s.description,
                        "action": format!("{:?}", s.action),
                        "confidence": s.confidence
                    })),
                    "context": error.context,
                    "timestamp": error.timestamp.to_rfc3339()
                })
            })
            .collect();

        Ok(serde_json::json!({
            "errors": errors_json,
            "summary": self.generate_summary()
        }))
    }

    /// Add error pattern for automatic error detection
    pub fn add_error_pattern(&mut self, pattern: ErrorPattern) {
        self.logger
            .info(&format!("Adding error pattern: {}", pattern.pattern));
        self.error_patterns.insert(pattern.pattern.clone(), pattern);
    }

    /// Detect errors using patterns
    pub fn detect_errors(&mut self, data: &Value, path: &str) {
        let mut errors_to_add = Vec::new();
        for (pattern_str, pattern) in &self.error_patterns {
            if let Some(error) = self.match_error_pattern(data, pattern_str, pattern, path) {
                errors_to_add.push(error);
            }
        }
        for error in errors_to_add {
            self.add_error(error);
        }
    }

    /// Match error pattern against data
    fn match_error_pattern(
        &self,
        data: &Value,
        pattern_str: &str,
        pattern: &ErrorPattern,
        path: &str,
    ) -> Option<ValidationError> {
        // This is a simplified implementation
        // In a real system, you'd have more sophisticated pattern matching
        if let Some(str_value) = data.as_str() {
            if str_value.contains(pattern_str) {
                return Some(ValidationError {
                    error_type: pattern.error_type.clone(),
                    path: path.to_string(),
                    message: format!("Pattern '{}' detected in data", pattern_str),
                    severity: pattern.severity.clone(),
                    suggestion: Some(ValidationSuggestion {
                        title: "Fix Pattern Match".to_string(),
                        description: pattern.suggestion_template.clone(),
                        action: SuggestionAction::Fix,
                        confidence: 0.8,
                    }),
                    context: Some(data.clone()),
                    timestamp: chrono::Utc::now(),
                });
            }
        }
        None
    }
}

impl SuggestionEngine {
    /// Create a new suggestion engine
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            confidence_threshold: 0.5,
        }
    }

    /// Add a suggestion pattern
    pub fn add_pattern(&mut self, pattern: SuggestionPattern) {
        self.patterns.insert(pattern.pattern.clone(), pattern);
    }

    /// Generate suggestion for an error
    pub fn generate_suggestion(&self, error: &ValidationError) -> Option<ValidationSuggestion> {
        // Look for matching patterns
        for (pattern_str, pattern) in &self.patterns {
            if self.matches_pattern(error, pattern_str) {
                return Some(ValidationSuggestion {
                    title: format!("Fix {:?}", error.error_type),
                    description: pattern.suggestion_template.clone(),
                    action: pattern.action.clone(),
                    confidence: pattern.confidence,
                });
            }
        }

        // Generate generic suggestions based on error type
        self.generate_generic_suggestion(error)
    }

    /// Check if error matches a pattern
    fn matches_pattern(&self, error: &ValidationError, pattern: &str) -> bool {
        error.message.contains(pattern) || error.path.contains(pattern)
    }

    /// Generate generic suggestion based on error type
    fn generate_generic_suggestion(&self, error: &ValidationError) -> Option<ValidationSuggestion> {
        match error.error_type {
            ValidationErrorType::SchemaValidation => Some(ValidationSuggestion {
                title: "Fix Schema Validation".to_string(),
                description: "Update the data to match the required schema".to_string(),
                action: SuggestionAction::Fix,
                confidence: 0.7,
            }),
            ValidationErrorType::TypeValidation => Some(ValidationSuggestion {
                title: "Fix Type Validation".to_string(),
                description: "Convert the value to the expected type".to_string(),
                action: SuggestionAction::Convert,
                confidence: 0.8,
            }),
            ValidationErrorType::RequiredFieldMissing => Some(ValidationSuggestion {
                title: "Add Required Field".to_string(),
                description: "Add the missing required field".to_string(),
                action: SuggestionAction::Add,
                confidence: 0.9,
            }),
            ValidationErrorType::ConstraintViolation => Some(ValidationSuggestion {
                title: "Fix Constraint Violation".to_string(),
                description: "Update the value to meet the constraint requirements".to_string(),
                action: SuggestionAction::Fix,
                confidence: 0.8,
            }),
            _ => Some(ValidationSuggestion {
                title: "Fix Validation Error".to_string(),
                description: "Review and correct the validation error".to_string(),
                action: SuggestionAction::Validate,
                confidence: 0.5,
            }),
        }
    }
}

/// Error summary
#[derive(Debug, Clone, serde::Serialize)]
pub struct ErrorSummary {
    pub total_errors: usize,
    pub critical_errors: usize,
    pub high_errors: usize,
    pub warnings: usize,
    pub info: usize,
    pub error_types: HashMap<ValidationErrorType, usize>,
    pub severity_distribution: HashMap<ErrorSeverity, usize>,
    pub has_errors: bool,
    pub has_critical_errors: bool,
}

impl Default for ValidationErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SuggestionEngine {
    fn default() -> Self {
        Self::new()
    }
}
