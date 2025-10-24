//! JSON schema validation
//!
//! This module provides JSON schema validation functionality including
//! schema parsing, validation, and error reporting.

use crate::error::{Error, Result};
use crate::logging::Logger;
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Schema validation result
#[derive(Debug, Clone)]
pub struct SchemaValidationResult {
    pub is_valid: bool,
    pub errors: Vec<SchemaError>,
    pub warnings: Vec<SchemaError>,
    pub validation_time_ms: u64,
}

/// Schema validation error
#[derive(Debug, Clone)]
pub struct SchemaError {
    pub path: String,
    pub message: String,
    pub error_type: SchemaErrorType,
    pub suggestion: Option<String>,
}

/// Types of schema validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaErrorType {
    RequiredFieldMissing,
    InvalidType,
    ConstraintViolation,
    FormatError,
    PatternMismatch,
    ArrayConstraintViolation,
    ObjectConstraintViolation,
    CustomValidationFailed,
}

/// JSON schema validator
pub struct SchemaValidator {
    logger: Logger,
    schemas: HashMap<String, Value>,
}

impl SchemaValidator {
    /// Create a new schema validator
    pub fn new() -> Self {
        Self {
            logger: Logger::new("schema_validator"),
            schemas: HashMap::new(),
        }
    }

    /// Register a schema for validation
    pub fn register_schema(&mut self, name: &str, schema: Value) -> Result<()> {
        self.logger.info(&format!("Registering schema: {}", name));

        // Validate the schema itself
        self.validate_schema_structure(&schema)?;

        self.schemas.insert(name.to_string(), schema);
        self.logger
            .info(&format!("Schema '{}' registered successfully", name));
        Ok(())
    }

    /// Validate data against a registered schema
    pub fn validate(&self, data: &Value, schema_name: &str) -> Result<SchemaValidationResult> {
        let start_time = std::time::Instant::now();

        self.logger
            .info(&format!("Validating data against schema: {}", schema_name));

        let schema = self
            .schemas
            .get(schema_name)
            .ok_or_else(|| Error::validation(format!("Schema '{}' not found", schema_name)))?;

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        self.validate_value(data, schema, "", &mut errors, &mut warnings);

        let validation_time = start_time.elapsed().as_millis() as u64;
        let is_valid = errors.is_empty();

        let result = SchemaValidationResult {
            is_valid,
            errors,
            warnings,
            validation_time_ms: validation_time,
        };

        self.logger.info(&format!(
            "Validation completed: {} errors, {} warnings in {}ms",
            result.errors.len(),
            result.warnings.len(),
            validation_time
        ));

        Ok(result)
    }

    /// Validate data against a schema directly
    pub fn validate_against_schema(
        &self,
        data: &Value,
        schema: &Value,
    ) -> Result<SchemaValidationResult> {
        let start_time = std::time::Instant::now();

        self.logger.info("Validating data against provided schema");

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        self.validate_value(data, schema, "", &mut errors, &mut warnings);

        let validation_time = start_time.elapsed().as_millis() as u64;
        let is_valid = errors.is_empty();

        let result = SchemaValidationResult {
            is_valid,
            errors,
            warnings,
            validation_time_ms: validation_time,
        };

        self.logger.info(&format!(
            "Validation completed: {} errors, {} warnings in {}ms",
            result.errors.len(),
            result.warnings.len(),
            validation_time
        ));

        Ok(result)
    }

    /// Validate the structure of a schema itself
    fn validate_schema_structure(&self, schema: &Value) -> Result<()> {
        if let Some(obj) = schema.as_object() {
            // Check for required schema properties
            if !obj.contains_key("type") && !obj.contains_key("$ref") && !obj.contains_key("allOf")
            {
                return Err(Error::validation(
                    "Schema must have 'type', '$ref', or 'allOf' property".to_string(),
                ));
            }
        } else {
            return Err(Error::validation(
                "Schema must be a JSON object".to_string(),
            ));
        }
        Ok(())
    }

    /// Recursively validate a value against a schema
    fn validate_value(
        &self,
        data: &Value,
        schema: &Value,
        path: &str,
        errors: &mut Vec<SchemaError>,
        warnings: &mut Vec<SchemaError>,
    ) {
        if let Some(schema_obj) = schema.as_object() {
            // Handle $ref
            if let Some(ref_value) = schema_obj.get("$ref") {
                if let Some(ref_str) = ref_value.as_str() {
                    if let Some(referenced_schema) = self.schemas.get(ref_str) {
                        self.validate_value(data, referenced_schema, path, errors, warnings);
                        return;
                    }
                }
            }

            // Handle allOf
            if let Some(all_of) = schema_obj.get("allOf") {
                if let Some(all_of_array) = all_of.as_array() {
                    for sub_schema in all_of_array {
                        self.validate_value(data, sub_schema, path, errors, warnings);
                    }
                    return;
                }
            }

            // Handle anyOf
            if let Some(any_of) = schema_obj.get("anyOf") {
                if let Some(any_of_array) = any_of.as_array() {
                    let mut has_valid_match = false;
                    let mut temp_errors = Vec::new();

                    for sub_schema in any_of_array {
                        let mut sub_errors = Vec::new();
                        let mut sub_warnings = Vec::new();
                        self.validate_value(
                            data,
                            sub_schema,
                            path,
                            &mut sub_errors,
                            &mut sub_warnings,
                        );

                        if sub_errors.is_empty() {
                            has_valid_match = true;
                            break;
                        } else {
                            temp_errors.extend(sub_errors);
                        }
                    }

                    if !has_valid_match {
                        errors.extend(temp_errors);
                    }
                    return;
                }
            }

            // Handle oneOf
            if let Some(one_of) = schema_obj.get("oneOf") {
                if let Some(one_of_array) = one_of.as_array() {
                    let mut valid_count = 0;
                    let mut all_errors = Vec::new();

                    for sub_schema in one_of_array {
                        let mut sub_errors = Vec::new();
                        let mut sub_warnings = Vec::new();
                        self.validate_value(
                            data,
                            sub_schema,
                            path,
                            &mut sub_errors,
                            &mut sub_warnings,
                        );

                        if sub_errors.is_empty() {
                            valid_count += 1;
                        } else {
                            all_errors.extend(sub_errors);
                        }
                    }

                    if valid_count != 1 {
                        errors.extend(all_errors);
                    }
                    return;
                }
            }

            // Handle not
            if let Some(not_schema) = schema_obj.get("not") {
                let mut temp_errors = Vec::new();
                let mut temp_warnings = Vec::new();
                self.validate_value(data, not_schema, path, &mut temp_errors, &mut temp_warnings);

                if temp_errors.is_empty() {
                    errors.push(SchemaError {
                        path: path.to_string(),
                        message: "Value matches 'not' schema".to_string(),
                        error_type: SchemaErrorType::CustomValidationFailed,
                        suggestion: Some("Value should not match the specified schema".to_string()),
                    });
                }
                return;
            }

            // Validate type
            if let Some(type_value) = schema_obj.get("type") {
                if let Some(type_str) = type_value.as_str() {
                    self.validate_type(data, type_str, path, errors);
                }
            }

            // Validate required fields
            if let Some(required) = schema_obj.get("required") {
                if let Some(required_array) = required.as_array() {
                    if let Some(data_obj) = data.as_object() {
                        for required_field in required_array {
                            if let Some(field_name) = required_field.as_str() {
                                if !data_obj.contains_key(field_name) {
                                    errors.push(SchemaError {
                                        path: format!("{}.{}", path, field_name),
                                        message: format!(
                                            "Required field '{}' is missing",
                                            field_name
                                        ),
                                        error_type: SchemaErrorType::RequiredFieldMissing,
                                        suggestion: Some(format!(
                                            "Add the required field '{}'",
                                            field_name
                                        )),
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // Validate properties
            if let Some(properties) = schema_obj.get("properties") {
                if let Some(properties_obj) = properties.as_object() {
                    if let Some(data_obj) = data.as_object() {
                        for (prop_name, prop_schema) in properties_obj {
                            let prop_path = if path.is_empty() {
                                prop_name.clone()
                            } else {
                                format!("{}.{}", path, prop_name)
                            };

                            if let Some(prop_value) = data_obj.get(prop_name) {
                                self.validate_value(
                                    prop_value,
                                    prop_schema,
                                    &prop_path,
                                    errors,
                                    warnings,
                                );
                            }
                        }
                    }
                }
            }

            // Validate additional properties
            if let Some(additional_properties) = schema_obj.get("additionalProperties") {
                if let Some(data_obj) = data.as_object() {
                    if let Some(properties) = schema_obj.get("properties") {
                        if let Some(properties_obj) = properties.as_object() {
                            for (key, value) in data_obj {
                                if !properties_obj.contains_key(key) {
                                    if additional_properties.is_boolean()
                                        && !additional_properties.as_bool().unwrap()
                                    {
                                        errors.push(SchemaError {
                                            path: format!("{}.{}", path, key),
                                            message: format!("Additional property '{}' is not allowed", key),
                                            error_type: SchemaErrorType::ObjectConstraintViolation,
                                            suggestion: Some("Remove the additional property or update the schema".to_string()),
                                        });
                                    } else if additional_properties.is_object() {
                                        self.validate_value(
                                            value,
                                            additional_properties,
                                            &format!("{}.{}", path, key),
                                            errors,
                                            warnings,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Validate array items
            if let Some(items) = schema_obj.get("items") {
                if let Some(data_array) = data.as_array() {
                    for (index, item) in data_array.iter().enumerate() {
                        let item_path = format!("{}[{}]", path, index);
                        self.validate_value(item, items, &item_path, errors, warnings);
                    }
                }
            }

            // Validate constraints
            self.validate_constraints(data, schema_obj, path, errors, warnings);
        }
    }

    /// Validate data type
    fn validate_type(
        &self,
        data: &Value,
        expected_type: &str,
        path: &str,
        errors: &mut Vec<SchemaError>,
    ) {
        let actual_type = match data {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };

        if actual_type != expected_type {
            errors.push(SchemaError {
                path: path.to_string(),
                message: format!("Expected type '{}', got '{}'", expected_type, actual_type),
                error_type: SchemaErrorType::InvalidType,
                suggestion: Some(format!("Convert the value to type '{}'", expected_type)),
            });
        }
    }

    /// Validate constraints
    fn validate_constraints(
        &self,
        data: &Value,
        schema_obj: &Map<String, Value>,
        path: &str,
        errors: &mut Vec<SchemaError>,
        warnings: &mut Vec<SchemaError>,
    ) {
        // String constraints
        if let Some(string_value) = data.as_str() {
            if let Some(min_length) = schema_obj.get("minLength") {
                if let Some(min_len) = min_length.as_u64() {
                    if string_value.len() < min_len as usize {
                        errors.push(SchemaError {
                            path: path.to_string(),
                            message: format!(
                                "String length {} is less than minimum {}",
                                string_value.len(),
                                min_len
                            ),
                            error_type: SchemaErrorType::ConstraintViolation,
                            suggestion: Some("Increase the string length".to_string()),
                        });
                    }
                }
            }

            if let Some(max_length) = schema_obj.get("maxLength") {
                if let Some(max_len) = max_length.as_u64() {
                    if string_value.len() > max_len as usize {
                        errors.push(SchemaError {
                            path: path.to_string(),
                            message: format!(
                                "String length {} exceeds maximum {}",
                                string_value.len(),
                                max_len
                            ),
                            error_type: SchemaErrorType::ConstraintViolation,
                            suggestion: Some("Decrease the string length".to_string()),
                        });
                    }
                }
            }

            if let Some(pattern) = schema_obj.get("pattern") {
                if let Some(pattern_str) = pattern.as_str() {
                    if let Ok(regex) = regex::Regex::new(pattern_str) {
                        if !regex.is_match(string_value) {
                            errors.push(SchemaError {
                                path: path.to_string(),
                                message: format!("String does not match pattern: {}", pattern_str),
                                error_type: SchemaErrorType::PatternMismatch,
                                suggestion: Some(
                                    "Update the string to match the required pattern".to_string(),
                                ),
                            });
                        }
                    }
                }
            }
        }

        // Number constraints
        if let Some(number_value) = data.as_f64() {
            if let Some(minimum) = schema_obj.get("minimum") {
                if let Some(min_val) = minimum.as_f64() {
                    if number_value < min_val {
                        errors.push(SchemaError {
                            path: path.to_string(),
                            message: format!(
                                "Value {} is less than minimum {}",
                                number_value, min_val
                            ),
                            error_type: SchemaErrorType::ConstraintViolation,
                            suggestion: Some("Increase the value".to_string()),
                        });
                    }
                }
            }

            if let Some(maximum) = schema_obj.get("maximum") {
                if let Some(max_val) = maximum.as_f64() {
                    if number_value > max_val {
                        errors.push(SchemaError {
                            path: path.to_string(),
                            message: format!("Value {} exceeds maximum {}", number_value, max_val),
                            error_type: SchemaErrorType::ConstraintViolation,
                            suggestion: Some("Decrease the value".to_string()),
                        });
                    }
                }
            }
        }

        // Array constraints
        if let Some(array_value) = data.as_array() {
            if let Some(min_items) = schema_obj.get("minItems") {
                if let Some(min_count) = min_items.as_u64() {
                    if array_value.len() < min_count as usize {
                        errors.push(SchemaError {
                            path: path.to_string(),
                            message: format!(
                                "Array length {} is less than minimum {}",
                                array_value.len(),
                                min_count
                            ),
                            error_type: SchemaErrorType::ArrayConstraintViolation,
                            suggestion: Some("Add more items to the array".to_string()),
                        });
                    }
                }
            }

            if let Some(max_items) = schema_obj.get("maxItems") {
                if let Some(max_count) = max_items.as_u64() {
                    if array_value.len() > max_count as usize {
                        errors.push(SchemaError {
                            path: path.to_string(),
                            message: format!(
                                "Array length {} exceeds maximum {}",
                                array_value.len(),
                                max_count
                            ),
                            error_type: SchemaErrorType::ArrayConstraintViolation,
                            suggestion: Some("Remove items from the array".to_string()),
                        });
                    }
                }
            }
        }
    }

    /// Get all registered schema names
    pub fn get_schema_names(&self) -> Vec<String> {
        self.schemas.keys().cloned().collect()
    }

    /// Check if a schema is registered
    pub fn has_schema(&self, name: &str) -> bool {
        self.schemas.contains_key(name)
    }

    /// Remove a schema
    pub fn remove_schema(&mut self, name: &str) -> Option<Value> {
        self.logger.info(&format!("Removing schema: {}", name));
        self.schemas.remove(name)
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}
