//! Validation Tests
//!
//! Comprehensive test coverage for validation components:
//! - JSON schema validation
//! - Data integrity checks
//! - Type validation and constraints
//! - Error reporting and suggestions
//! - Schema registry management

use common_library::validation::errors::{
    ErrorSeverity, ValidationError, ValidationErrorReporter, ValidationErrorType,
};
use common_library::validation::integrity::{
    ConstraintType as IntegrityConstraintType, IntegrityConstraint, IntegrityViolationType,
    ViolationSeverity,
};
use common_library::validation::types::{ConstraintSeverity, ConstraintType, TypeConstraint};
use common_library::validation::{
    DataIntegrityChecker, SchemaRegistry, SchemaValidator, TypeValidator,
};
use serde_json::json;

#[tokio::test]
async fn test_schema_validation() {
    // Test: Schema validation works correctly
    let mut validator = SchemaValidator::new();

    // Test schema registration
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "age": {"type": "number", "minimum": 0},
            "email": {"type": "string", "pattern": "^[\\w\\.-]+@[\\w\\.-]+\\.[a-zA-Z]{2,}$"}
        },
        "required": ["name", "age"]
    });

    validator
        .register_schema("user", schema)
        .expect("Should register schema");
    assert!(validator.has_schema("user"), "Schema should be registered");

    // Test valid data
    let valid_data = json!({
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com"
    });

    let result = validator
        .validate(&valid_data, "user")
        .expect("Should validate data");
    assert!(result.is_valid, "Valid data should pass validation");
    assert!(result.errors.is_empty(), "Valid data should have no errors");

    // Test invalid data
    let invalid_data = json!({
        "name": "",
        "age": -5,
        "email": "invalid-email"
    });

    let result = validator
        .validate(&invalid_data, "user")
        .expect("Should validate data");
    assert!(!result.is_valid, "Invalid data should fail validation");
    assert!(!result.errors.is_empty(), "Invalid data should have errors");

    // Test missing required field
    let incomplete_data = json!({
        "name": "John Doe"
    });

    let result = validator
        .validate(&incomplete_data, "user")
        .expect("Should validate data");
    assert!(!result.is_valid, "Incomplete data should fail validation");
    assert!(
        result.errors.iter().any(|e| e.error_type
            == common_library::validation::schema::SchemaErrorType::RequiredFieldMissing),
        "Should have required field error"
    );
}

#[tokio::test]
async fn test_data_integrity_checks() {
    // Test: Data integrity checks work correctly
    let mut checker = DataIntegrityChecker::new();

    // Test checksum validation
    let data = json!({"name": "John", "age": 30});
    let checksum = "abc123";
    checker.add_checksum("test_data", checksum);

    let result = checker
        .check_integrity(&data, "test_data")
        .expect("Should check integrity");
    assert!(!result.checksum_valid, "Checksum should not match");

    // Test constraint validation
    let constraint = IntegrityConstraint {
        name: "age_constraint".to_string(),
        constraint_type: IntegrityConstraintType::Range,
        path: "age".to_string(),
        value: Some(json!({"min": 0, "max": 120})),
        severity: ViolationSeverity::High,
    };
    checker.add_constraint(constraint);

    let result = checker
        .check_integrity(&data, "test_data")
        .expect("Should check integrity");
    // Note: The integrity checker doesn't have a registered checksum, so it will pass
    assert!(result.is_valid, "Data should pass integrity check");

    // Test invalid data
    let invalid_data = json!({"name": "John", "age": 150});
    let result = checker
        .check_integrity(&invalid_data, "test_data")
        .expect("Should check integrity");
    assert!(!result.is_valid, "Invalid data should fail integrity check");
    assert!(
        !result.violations.is_empty(),
        "Should have integrity violations"
    );
}

#[tokio::test]
async fn test_type_validation() {
    // Test: Type validation works correctly
    let mut validator = TypeValidator::new();

    // Test basic type validation
    let string_data = json!("hello");
    let result = validator
        .validate_type(&string_data, "string")
        .expect("Should validate type");
    assert!(
        result.is_valid,
        "String data should be valid for string type"
    );
    assert_eq!(result.actual_type, "string");
    assert_eq!(result.expected_type, "string");

    // Test type mismatch
    let number_data = json!(42);
    let result = validator
        .validate_type(&number_data, "string")
        .expect("Should validate type");
    assert!(
        !result.is_valid,
        "Number data should be invalid for string type"
    );
    assert!(!result.errors.is_empty(), "Should have type mismatch error");

    // Test type conversion
    let string_number = json!("42");
    let converted = validator
        .convert_type(&string_number, "number")
        .expect("Should convert type");
    assert!(converted.is_number(), "Should convert to number");

    // Test constraint validation
    let constraint = TypeConstraint {
        name: "length_constraint".to_string(),
        constraint_type: ConstraintType::MinLength,
        value: Some(json!(3)),
        severity: ConstraintSeverity::Error,
    };
    validator.add_constraint(constraint);

    let short_string = json!("hi");
    let result = validator
        .validate_type(&short_string, "string")
        .expect("Should validate type");
    assert!(
        !result.is_valid,
        "Short string should fail length constraint"
    );
}

#[tokio::test]
async fn test_error_reporting() {
    // Test: Error reporting works correctly
    let mut reporter = ValidationErrorReporter::new();

    // Test adding errors
    let error = ValidationError {
        error_type: ValidationErrorType::SchemaValidation,
        path: "name".to_string(),
        message: "Required field missing".to_string(),
        severity: ErrorSeverity::Error,
        suggestion: None,
        context: Some(json!("test")),
        timestamp: chrono::Utc::now(),
    };

    reporter.add_error(error);
    assert_eq!(reporter.get_errors().len(), 1, "Should have one error");

    // Test error filtering
    let errors_by_type = reporter.get_errors_by_type(&ValidationErrorType::SchemaValidation);
    assert_eq!(errors_by_type.len(), 1, "Should find error by type");

    let errors_by_severity = reporter.get_errors_by_severity(&ErrorSeverity::Error);
    assert_eq!(errors_by_severity.len(), 1, "Should find error by severity");

    // Test error summary
    let summary = reporter.generate_summary();
    assert_eq!(summary.total_errors, 1, "Should have one total error");
    assert!(summary.has_errors, "Should have errors");

    // Test suggestions
    let suggestions = reporter.generate_suggestions();
    assert!(!suggestions.is_empty(), "Should generate suggestions");
}

#[tokio::test]
async fn test_schema_registry() {
    // Test: Schema registry works correctly
    let mut registry = SchemaRegistry::new();

    // Test schema registration
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"}
        }
    });

    let metadata = common_library::validation::registry::SchemaMetadata {
        name: "user".to_string(),
        version: "1.0".to_string(),
        schema,
        description: Some("User schema".to_string()),
        tags: vec!["user".to_string(), "profile".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        author: Some("test".to_string()),
        dependencies: Vec::new(),
        is_deprecated: false,
    };

    registry
        .register_schema(metadata)
        .expect("Should register schema");

    // Test schema retrieval
    let retrieved = registry
        .get_schema("user", "1.0")
        .expect("Should get schema");
    assert_eq!(retrieved.name, "user");
    assert_eq!(retrieved.version, "1.0");

    // Test schema listing
    let schemas = registry.list_schemas();
    assert!(
        schemas.contains(&"user".to_string()),
        "Should list user schema"
    );

    // Test schema search
    let search_results = registry.search_schemas("user");
    assert!(!search_results.is_empty(), "Should find user schema");

    // Test schema deprecation
    registry
        .deprecate_schema("user", "1.0")
        .expect("Should deprecate schema");
    let deprecated = registry
        .get_schema("user", "1.0")
        .expect("Should get deprecated schema");
    assert!(deprecated.is_deprecated, "Schema should be deprecated");

    // Test registry statistics
    let stats = registry.get_statistics();
    assert_eq!(stats.total_schemas, 1, "Should have one schema");
    assert_eq!(
        stats.deprecated_count, 1,
        "Should have one deprecated schema"
    );
}

#[tokio::test]
async fn test_validation_integration() {
    // Test: All validation components work together
    let mut schema_validator = SchemaValidator::new();
    let mut integrity_checker = DataIntegrityChecker::new();
    let mut type_validator = TypeValidator::new();
    let mut error_reporter = ValidationErrorReporter::new();
    let mut schema_registry = SchemaRegistry::new();

    // Register schema
    let schema = json!({
        "type": "object",
        "properties": {
            "id": {"type": "string", "pattern": "^[0-9]+$"},
            "name": {"type": "string", "minLength": 1},
            "age": {"type": "number", "minimum": 0, "maximum": 120}
        },
        "required": ["id", "name", "age"]
    });

    schema_validator
        .register_schema("person", schema)
        .expect("Should register schema");

    // Add integrity constraints
    let constraint = IntegrityConstraint {
        name: "age_range".to_string(),
        constraint_type: IntegrityConstraintType::Range,
        path: "age".to_string(),
        value: Some(json!({"min": 0, "max": 120})),
        severity: ViolationSeverity::High,
    };
    integrity_checker.add_constraint(constraint);

    // Add type constraints
    let type_constraint = TypeConstraint {
        name: "name_length".to_string(),
        constraint_type: ConstraintType::MinLength,
        value: Some(json!(1)),
        severity: ConstraintSeverity::Error,
    };
    type_validator.add_constraint(type_constraint);

    // Test valid data
    let valid_data = json!({
        "id": "123",
        "name": "John Doe",
        "age": 30
    });

    // Schema validation
    let schema_result = schema_validator
        .validate(&valid_data, "person")
        .expect("Should validate schema");
    assert!(
        schema_result.is_valid,
        "Valid data should pass schema validation"
    );

    // Integrity check
    let integrity_result = integrity_checker
        .check_integrity(&valid_data, "test")
        .expect("Should check integrity");
    assert!(
        integrity_result.is_valid,
        "Valid data should pass integrity check"
    );

    // Type validation
    let type_result = type_validator
        .validate_type(&valid_data, "object")
        .expect("Should validate type");
    assert!(
        type_result.is_valid,
        "Valid data should pass type validation"
    );

    // Test invalid data
    let invalid_data = json!({
        "id": "abc",
        "name": "",
        "age": 150
    });

    // Schema validation
    let schema_result = schema_validator
        .validate(&invalid_data, "person")
        .expect("Should validate schema");
    assert!(
        !schema_result.is_valid,
        "Invalid data should fail schema validation"
    );

    // Report errors
    for error in schema_result.errors {
        let validation_error = ValidationError {
            error_type: ValidationErrorType::SchemaValidation,
            path: error.path,
            message: error.message,
            severity: ErrorSeverity::Error,
            suggestion: error.suggestion.map(|s| {
                common_library::validation::errors::ValidationSuggestion {
                    title: "Fix Schema Error".to_string(),
                    description: s,
                    action: common_library::validation::errors::SuggestionAction::Fix,
                    confidence: 0.8,
                }
            }),
            context: Some(invalid_data.clone()),
            timestamp: chrono::Utc::now(),
        };
        error_reporter.add_error(validation_error);
    }

    // Generate error summary
    let summary = error_reporter.generate_summary();
    assert!(summary.has_errors, "Should have errors");
    assert!(summary.total_errors > 0, "Should have total errors");

    // Test error export
    let exported = error_reporter
        .export_errors()
        .expect("Should export errors");
    assert!(
        exported.is_object(),
        "Exported errors should be JSON object"
    );
}

#[tokio::test]
async fn test_edge_cases() {
    // Test: Edge cases are handled correctly
    let validator = SchemaValidator::new();
    let integrity_checker = DataIntegrityChecker::new();
    let type_validator = TypeValidator::new();

    // Test empty data
    let empty_data = json!({});
    assert!(
        validator
            .validate_against_schema(&empty_data, &json!({"type": "object"}))
            .is_ok(),
        "Should handle empty data"
    );

    // Test null data
    let null_data = json!(null);
    assert!(
        validator
            .validate_against_schema(&null_data, &json!({"type": "null"}))
            .is_ok(),
        "Should handle null data"
    );

    // Test array data
    let array_data = json!([1, 2, 3]);
    assert!(
        validator
            .validate_against_schema(&array_data, &json!({"type": "array"}))
            .is_ok(),
        "Should handle array data"
    );

    // Test nested object data
    let nested_data = json!({
        "user": {
            "profile": {
                "name": "John"
            }
        }
    });
    let nested_schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "profile": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"}
                        }
                    }
                }
            }
        }
    });
    assert!(
        validator
            .validate_against_schema(&nested_data, &nested_schema)
            .is_ok(),
        "Should handle nested objects"
    );

    // Test type conversion edge cases
    assert!(
        type_validator
            .convert_type(&json!("true"), "boolean")
            .is_ok(),
        "Should convert string to boolean"
    );
    assert!(
        type_validator.convert_type(&json!("42"), "number").is_ok(),
        "Should convert string to number"
    );
    assert!(
        type_validator.convert_type(&json!("42"), "integer").is_ok(),
        "Should convert string to integer"
    );

    // Test integrity check with no constraints
    let result = integrity_checker
        .check_integrity(&json!({"test": "data"}), "test")
        .expect("Should check integrity");
    assert!(result.is_valid, "Data should be valid with no constraints");
    assert_eq!(
        result.consistency_score, 100.0,
        "Consistency score should be 100%"
    );
}
