//! Type validation and constraints
//!
//! This module provides type validation functionality including type checking,
//! constraint validation, and type conversion utilities.

use crate::error::{Error, Result};
use crate::logging::Logger;
use serde_json::Value;
use std::collections::HashMap;

/// Type validation result
#[derive(Debug, Clone)]
pub struct TypeValidationResult {
    pub is_valid: bool,
    pub actual_type: String,
    pub expected_type: String,
    pub errors: Vec<TypeError>,
    pub warnings: Vec<TypeError>,
    pub validation_time_ms: u64,
}

/// Type validation error
#[derive(Debug, Clone)]
pub struct TypeError {
    pub path: String,
    pub message: String,
    pub error_type: TypeErrorType,
    pub suggestion: Option<String>,
}

/// Types of type validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum TypeErrorType {
    TypeMismatch,
    ConstraintViolation,
    ConversionError,
    RangeViolation,
    FormatError,
    RequiredFieldMissing,
    InvalidValue,
}

/// Type constraint definition
#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub value: Option<Value>,
    pub severity: ConstraintSeverity,
}

/// Types of constraints
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    Required,
    MinLength,
    MaxLength,
    MinValue,
    MaxValue,
    Pattern,
    Enum,
    Custom,
}

/// Constraint severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintSeverity {
    Warning,
    Error,
    Critical,
}

/// Type validator
pub struct TypeValidator {
    logger: Logger,
    type_registry: HashMap<String, TypeDefinition>,
    constraints: Vec<TypeConstraint>,
}

/// Type definition
#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub base_type: String,
    pub constraints: Vec<TypeConstraint>,
    pub description: Option<String>,
}

impl TypeValidator {
    /// Create a new type validator
    pub fn new() -> Self {
        Self {
            logger: Logger::new("type_validator"),
            type_registry: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Register a custom type definition
    pub fn register_type(&mut self, type_def: TypeDefinition) -> Result<()> {
        self.logger
            .info(&format!("Registering type: {}", type_def.name));

        // Validate the type definition
        self.validate_type_definition(&type_def)?;

        let name = type_def.name.clone();
        self.type_registry.insert(name.clone(), type_def);
        self.logger
            .info(&format!("Type '{}' registered successfully", name));
        Ok(())
    }

    /// Add a type constraint
    pub fn add_constraint(&mut self, constraint: TypeConstraint) {
        self.logger
            .info(&format!("Adding constraint: {}", constraint.name));
        self.constraints.push(constraint);
    }

    /// Validate a value against a type
    pub fn validate_type(&self, value: &Value, type_name: &str) -> Result<TypeValidationResult> {
        let start_time = std::time::Instant::now();

        self.logger
            .info(&format!("Validating value against type: {}", type_name));

        let actual_type = self.get_value_type(value);
        let expected_type = type_name.to_string();

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check basic type compatibility
        if !self.is_type_compatible(&actual_type, &expected_type) {
            errors.push(TypeError {
                path: "root".to_string(),
                message: format!(
                    "Type mismatch: expected '{}', got '{}'",
                    expected_type, actual_type
                ),
                error_type: TypeErrorType::TypeMismatch,
                suggestion: Some(format!("Convert the value to type '{}'", expected_type)),
            });
        }

        // Validate against type definition if available
        if let Some(type_def) = self.type_registry.get(type_name) {
            self.validate_against_type_definition(value, type_def, "", &mut errors, &mut warnings);
        }

        // Validate against global constraints
        for constraint in &self.constraints {
            if let Some(error) = self.validate_constraint(value, constraint) {
                match constraint.severity {
                    ConstraintSeverity::Warning => warnings.push(error),
                    ConstraintSeverity::Error | ConstraintSeverity::Critical => errors.push(error),
                }
            }
        }

        let validation_time = start_time.elapsed().as_millis() as u64;
        let is_valid = errors.is_empty();

        let result = TypeValidationResult {
            is_valid,
            actual_type,
            expected_type,
            errors,
            warnings,
            validation_time_ms: validation_time,
        };

        self.logger.info(&format!(
            "Type validation completed: {} errors, {} warnings in {}ms",
            result.errors.len(),
            result.warnings.len(),
            validation_time
        ));

        Ok(result)
    }

    /// Validate a value against a type definition
    fn validate_against_type_definition(
        &self,
        value: &Value,
        type_def: &TypeDefinition,
        path: &str,
        errors: &mut Vec<TypeError>,
        warnings: &mut Vec<TypeError>,
    ) {
        // Validate base type
        let actual_type = self.get_value_type(value);
        if !self.is_type_compatible(&actual_type, &type_def.base_type) {
            errors.push(TypeError {
                path: path.to_string(),
                message: format!(
                    "Base type mismatch: expected '{}', got '{}'",
                    type_def.base_type, actual_type
                ),
                error_type: TypeErrorType::TypeMismatch,
                suggestion: Some(format!(
                    "Convert the value to base type '{}'",
                    type_def.base_type
                )),
            });
        }

        // Validate type constraints
        for constraint in &type_def.constraints {
            if let Some(error) = self.validate_constraint(value, constraint) {
                match constraint.severity {
                    ConstraintSeverity::Warning => warnings.push(error),
                    ConstraintSeverity::Error | ConstraintSeverity::Critical => errors.push(error),
                }
            }
        }
    }

    /// Validate a constraint
    fn validate_constraint(&self, value: &Value, constraint: &TypeConstraint) -> Option<TypeError> {
        match constraint.constraint_type {
            ConstraintType::Required => self.validate_required_constraint(value, constraint),
            ConstraintType::MinLength => self.validate_min_length_constraint(value, constraint),
            ConstraintType::MaxLength => self.validate_max_length_constraint(value, constraint),
            ConstraintType::MinValue => self.validate_min_value_constraint(value, constraint),
            ConstraintType::MaxValue => self.validate_max_value_constraint(value, constraint),
            ConstraintType::Pattern => self.validate_pattern_constraint(value, constraint),
            ConstraintType::Enum => self.validate_enum_constraint(value, constraint),
            ConstraintType::Custom => self.validate_custom_constraint(value, constraint),
        }
    }

    /// Validate required constraint
    fn validate_required_constraint(
        &self,
        value: &Value,
        constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        if value.is_null() {
            Some(TypeError {
                path: "root".to_string(),
                message: format!("Required field '{}' is missing", constraint.name),
                error_type: TypeErrorType::RequiredFieldMissing,
                suggestion: Some("Provide a value for this required field".to_string()),
            })
        } else {
            None
        }
    }

    /// Validate minimum length constraint
    fn validate_min_length_constraint(
        &self,
        value: &Value,
        constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        if let Some(str_value) = value.as_str() {
            if let Some(constraint_value) = &constraint.value {
                if let Some(min_length) = constraint_value.as_u64() {
                    if str_value.len() < min_length as usize {
                        return Some(TypeError {
                            path: "root".to_string(),
                            message: format!(
                                "String length {} is less than minimum {}",
                                str_value.len(),
                                min_length
                            ),
                            error_type: TypeErrorType::ConstraintViolation,
                            suggestion: Some("Increase the string length".to_string()),
                        });
                    }
                }
            }
        }
        None
    }

    /// Validate maximum length constraint
    fn validate_max_length_constraint(
        &self,
        value: &Value,
        constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        if let Some(str_value) = value.as_str() {
            if let Some(constraint_value) = &constraint.value {
                if let Some(max_length) = constraint_value.as_u64() {
                    if str_value.len() > max_length as usize {
                        return Some(TypeError {
                            path: "root".to_string(),
                            message: format!(
                                "String length {} exceeds maximum {}",
                                str_value.len(),
                                max_length
                            ),
                            error_type: TypeErrorType::ConstraintViolation,
                            suggestion: Some("Decrease the string length".to_string()),
                        });
                    }
                }
            }
        }
        None
    }

    /// Validate minimum value constraint
    fn validate_min_value_constraint(
        &self,
        value: &Value,
        constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        if let Some(num_value) = value.as_f64() {
            if let Some(constraint_value) = &constraint.value {
                if let Some(min_value) = constraint_value.as_f64() {
                    if num_value < min_value {
                        return Some(TypeError {
                            path: "root".to_string(),
                            message: format!(
                                "Value {} is less than minimum {}",
                                num_value, min_value
                            ),
                            error_type: TypeErrorType::RangeViolation,
                            suggestion: Some("Increase the value".to_string()),
                        });
                    }
                }
            }
        }
        None
    }

    /// Validate maximum value constraint
    fn validate_max_value_constraint(
        &self,
        value: &Value,
        constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        if let Some(num_value) = value.as_f64() {
            if let Some(constraint_value) = &constraint.value {
                if let Some(max_value) = constraint_value.as_f64() {
                    if num_value > max_value {
                        return Some(TypeError {
                            path: "root".to_string(),
                            message: format!("Value {} exceeds maximum {}", num_value, max_value),
                            error_type: TypeErrorType::RangeViolation,
                            suggestion: Some("Decrease the value".to_string()),
                        });
                    }
                }
            }
        }
        None
    }

    /// Validate pattern constraint
    fn validate_pattern_constraint(
        &self,
        value: &Value,
        constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        if let Some(str_value) = value.as_str() {
            if let Some(constraint_value) = &constraint.value {
                if let Some(pattern) = constraint_value.as_str() {
                    if let Ok(regex) = regex::Regex::new(pattern) {
                        if !regex.is_match(str_value) {
                            return Some(TypeError {
                                path: "root".to_string(),
                                message: format!(
                                    "Value '{}' does not match pattern '{}'",
                                    str_value, pattern
                                ),
                                error_type: TypeErrorType::FormatError,
                                suggestion: Some(format!(
                                    "Update the value to match pattern '{}'",
                                    pattern
                                )),
                            });
                        }
                    }
                }
            }
        }
        None
    }

    /// Validate enum constraint
    fn validate_enum_constraint(
        &self,
        value: &Value,
        constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        if let Some(constraint_value) = &constraint.value {
            if let Some(enum_array) = constraint_value.as_array() {
                if !enum_array.contains(value) {
                    return Some(TypeError {
                        path: "root".to_string(),
                        message: format!("Value '{}' is not in allowed values", value),
                        error_type: TypeErrorType::InvalidValue,
                        suggestion: Some("Use one of the allowed values".to_string()),
                    });
                }
            }
        }
        None
    }

    /// Validate custom constraint
    fn validate_custom_constraint(
        &self,
        _value: &Value,
        _constraint: &TypeConstraint,
    ) -> Option<TypeError> {
        // This is a placeholder for custom constraint logic
        // In a real implementation, you'd have a way to define custom validation functions
        None
    }

    /// Get the type of a JSON value
    fn get_value_type(&self, value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
        }
    }

    /// Check if two types are compatible
    fn is_type_compatible(&self, actual: &str, expected: &str) -> bool {
        match (actual, expected) {
            ("number", "integer") => true,
            ("integer", "number") => true,
            (a, e) => a == e,
        }
    }

    /// Validate a type definition
    fn validate_type_definition(&self, type_def: &TypeDefinition) -> Result<()> {
        if type_def.name.is_empty() {
            return Err(Error::validation("Type name cannot be empty".to_string()));
        }

        if type_def.base_type.is_empty() {
            return Err(Error::validation("Base type cannot be empty".to_string()));
        }

        // Validate base type
        let valid_base_types = [
            "string", "number", "integer", "boolean", "array", "object", "null",
        ];
        if !valid_base_types.contains(&type_def.base_type.as_str()) {
            return Err(Error::validation(format!(
                "Invalid base type: {}",
                type_def.base_type
            )));
        }

        Ok(())
    }

    /// Convert a value to a specific type
    pub fn convert_type(&self, value: &Value, target_type: &str) -> Result<Value> {
        match target_type {
            "string" => Ok(Value::String(value.to_string())),
            "number" => {
                if let Some(num) = value.as_f64() {
                    Ok(Value::Number(
                        serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)),
                    ))
                } else if let Some(str_val) = value.as_str() {
                    if let Ok(parsed) = str_val.parse::<f64>() {
                        Ok(Value::Number(
                            serde_json::Number::from_f64(parsed)
                                .unwrap_or(serde_json::Number::from(0)),
                        ))
                    } else {
                        Err(Error::validation("Cannot convert to number".to_string()))
                    }
                } else {
                    Err(Error::validation("Cannot convert to number".to_string()))
                }
            }
            "integer" => {
                if let Some(num) = value.as_f64() {
                    Ok(Value::Number(serde_json::Number::from(num as i64)))
                } else if let Some(str_val) = value.as_str() {
                    if let Ok(parsed) = str_val.parse::<i64>() {
                        Ok(Value::Number(serde_json::Number::from(parsed)))
                    } else {
                        Err(Error::validation("Cannot convert to integer".to_string()))
                    }
                } else {
                    Err(Error::validation("Cannot convert to integer".to_string()))
                }
            }
            "boolean" => {
                if let Some(bool_val) = value.as_bool() {
                    Ok(Value::Bool(bool_val))
                } else if let Some(str_val) = value.as_str() {
                    match str_val.to_lowercase().as_str() {
                        "true" | "1" | "yes" | "on" => Ok(Value::Bool(true)),
                        "false" | "0" | "no" | "off" => Ok(Value::Bool(false)),
                        _ => Err(Error::validation("Cannot convert to boolean".to_string())),
                    }
                } else {
                    Err(Error::validation("Cannot convert to boolean".to_string()))
                }
            }
            "array" => {
                if let Some(array_val) = value.as_array() {
                    Ok(Value::Array(array_val.clone()))
                } else {
                    Ok(Value::Array(vec![value.clone()]))
                }
            }
            "object" => {
                if let Some(obj_val) = value.as_object() {
                    Ok(Value::Object(obj_val.clone()))
                } else {
                    Err(Error::validation("Cannot convert to object".to_string()))
                }
            }
            "null" => Ok(Value::Null),
            _ => Err(Error::validation(format!(
                "Unknown target type: {}",
                target_type
            ))),
        }
    }

    /// Get all registered types
    pub fn get_registered_types(&self) -> Vec<String> {
        self.type_registry.keys().cloned().collect()
    }

    /// Check if a type is registered
    pub fn has_type(&self, name: &str) -> bool {
        self.type_registry.contains_key(name)
    }

    /// Remove a type definition
    pub fn remove_type(&mut self, name: &str) -> Option<TypeDefinition> {
        self.logger.info(&format!("Removing type: {}", name));
        self.type_registry.remove(name)
    }
}

impl Default for TypeValidator {
    fn default() -> Self {
        Self::new()
    }
}
