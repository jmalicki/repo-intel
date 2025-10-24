//! Foundation Tests
//!
//! Comprehensive test coverage for core foundation components:
//! - Configuration management
//! - Error handling
//! - Logging infrastructure
//! - Utility functions
//! - Module integration

use common_library::prelude::*;
use std::time::SystemTime;
use tempfile::tempdir;

#[tokio::test]
async fn test_foundation_components() {
    // Test: All foundation components can be loaded and used
    let _config_manager = ConfigManager::new().expect("ConfigManager should be created");
    let _logger = Logger::new("foundation_test");
    let _error = Error::generic("test error");

    // Test utility functions
    let uuid = crypto::generate_uuid();
    assert!(!uuid.to_string().is_empty(), "UUID should be generated");

    let now = date::now();
    assert!(now.timestamp() > 0, "Current timestamp should be positive");

    // Test validation
    assert!(
        validation::is_valid_email("test@example.com"),
        "Valid email should pass"
    );
    assert!(
        !validation::is_valid_email("invalid-email"),
        "Invalid email should fail"
    );

    // Test string utilities
    let truncated = string::truncate("Hello World", 5);
    assert_eq!(truncated, "He...", "String should be truncated correctly");
}

#[tokio::test]
async fn test_configuration_management() {
    // Test: Configuration management works correctly
    let config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Test default configuration
    let app_config = config_manager
        .get_app_config()
        .expect("Should get app config");
    assert_eq!(
        app_config.database.max_connections, 10,
        "Default max connections should be 10"
    );
    assert_eq!(
        app_config.http.timeout_seconds, 30,
        "Default timeout should be 30"
    );

    // Test configuration validation
    assert!(
        config_manager.validate().is_ok(),
        "Default config should be valid"
    );

    // Test typed configuration retrieval
    let max_connections: u32 = config_manager
        .get("database.max_connections")
        .expect("Should get max connections");
    assert_eq!(
        max_connections, 10,
        "Should get correct max connections value"
    );
}

#[tokio::test]
async fn test_error_handling_comprehensive() {
    // Test: Error handling works for all error types
    let config_error = Error::config("test config error");
    assert!(matches!(config_error, Error::Config(_)));

    let http_error = Error::http("test http error");
    assert!(matches!(http_error, Error::Http(_)));

    let validation_error = Error::validation("test validation error");
    assert!(matches!(validation_error, Error::Validation(_)));

    // Test error conversion from std::io::Error
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let common_error: Error = io_error.into();
    assert!(matches!(common_error, Error::Io(_)));

    // Test error conversion from serde_json::Error
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let common_error: Error = json_error.into();
    assert!(matches!(common_error, Error::Serialization(_)));

    // Test error display
    let error_msg = config_error.to_string();
    assert!(
        error_msg.contains("Configuration error"),
        "Error message should contain type"
    );
    assert!(
        error_msg.contains("test config error"),
        "Error message should contain details"
    );
}

#[tokio::test]
async fn test_logging_infrastructure() {
    // Test: Logging infrastructure works correctly
    // Initialize logging (this should not panic)
    let init_result = common_library::logging::init();
    assert!(init_result.is_ok(), "Logging initialization should succeed");

    // Test logger creation and usage
    let logger = Logger::new("foundation_logging_test");
    logger.info("This is an info message");
    logger.warn("This is a warning message");
    logger.error("This is an error message");
    logger.debug("This is a debug message");

    // Test performance logging
    let start = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(10));
    logger.log_performance("test_operation", start.elapsed());
}

#[tokio::test]
async fn test_utility_functions_comprehensive() {
    // Test: All utility functions work correctly

    // Date utilities
    let now = date::now();
    let timestamp_str = date::format_timestamp(now);
    assert!(
        !timestamp_str.is_empty(),
        "Formatted timestamp should not be empty"
    );

    // Test RFC3339 functionality
    let rfc3339_time = date::format_rfc3339(now);
    assert!(
        !rfc3339_time.is_empty(),
        "RFC3339 timestamp should not be empty"
    );
    assert!(rfc3339_time.contains('T'), "RFC3339 should contain 'T'");

    // Test RFC3339 parsing
    let parsed_time = date::parse_rfc3339(&rfc3339_time);
    assert!(
        parsed_time.is_ok(),
        "Should be able to parse RFC3339 timestamp"
    );

    // Test SystemTime conversion with nanosecond precision
    let system_time = SystemTime::now();
    let converted_time = date::from_system_time(system_time);
    assert!(converted_time.is_ok(), "SystemTime conversion should work");

    // Crypto utilities
    let uuid1 = crypto::generate_uuid();
    let uuid2 = crypto::generate_uuid();
    assert_ne!(uuid1, uuid2, "Generated UUIDs should be unique");

    let random_string = crypto::generate_random_string(10);
    assert_eq!(
        random_string.len(),
        10,
        "Random string should have correct length"
    );

    // Base64 roundtrip test
    let test_data = b"Hello, World!";
    let encoded = crypto::encode_base64(test_data);
    let decoded = crypto::decode_base64(&encoded).expect("Base64 decode should work");
    assert_eq!(decoded, test_data, "Base64 roundtrip should preserve data");

    // String utilities
    let test_string = "Hello World";
    let truncated = string::truncate(test_string, 5);
    assert_eq!(truncated, "He...", "String should be truncated correctly");

    assert!(string::is_blank("   "), "Blank string should be detected");
    assert!(
        !string::is_blank("hello"),
        "Non-blank string should not be detected"
    );

    // Test case conversion
    let snake_case = string::to_snake_case("HelloWorld");
    assert_eq!(snake_case, "hello_world", "Should convert to snake_case");

    let camel_case = string::to_camel_case("hello_world");
    assert_eq!(camel_case, "helloWorld", "Should convert to camelCase");

    // Validation utilities
    assert!(
        validation::is_valid_email("test@example.com"),
        "Valid email should pass"
    );
    assert!(
        !validation::is_valid_email("invalid-email"),
        "Invalid email should fail"
    );

    assert!(
        validation::is_valid_url("https://example.com"),
        "Valid URL should pass"
    );
    assert!(
        !validation::is_valid_url("invalid-url"),
        "Invalid URL should fail"
    );

    assert!(
        validation::is_not_empty("hello"),
        "Non-empty string should pass"
    );
    assert!(!validation::is_not_empty(""), "Empty string should fail");

    assert!(
        validation::is_in_range(5.0, 0.0, 10.0),
        "Value in range should pass"
    );
    assert!(
        !validation::is_in_range(15.0, 0.0, 10.0),
        "Value out of range should fail"
    );

    // File system utilities
    let temp_dir = tempdir().expect("Should create temp directory");
    let test_path = temp_dir.path().join("test_subdir");

    assert!(
        fs::ensure_dir(&test_path).is_ok(),
        "Should be able to create directory"
    );
    assert!(test_path.is_dir(), "Directory should exist");

    let test_file = test_path.join("test_file.txt");
    std::fs::write(&test_file, "hello").expect("Should write test file");
    assert!(fs::is_file(&test_file), "Should detect file");
    assert!(
        !fs::is_dir(&test_file),
        "File should not be detected as directory"
    );

    // Test file size
    let file_size = fs::file_size(&test_file).expect("Should get file size");
    assert!(file_size > 0, "File should have content");
}

#[tokio::test]
async fn test_module_integration() {
    // Test: All modules work together correctly
    let config_manager = ConfigManager::new().expect("ConfigManager should be created");
    let _app_config = config_manager
        .get_app_config()
        .expect("Should get app config");

    // Note: Logging is already initialized by previous tests

    // Create logger and test logging
    let logger = Logger::new("integration_test");
    logger.info("Integration test started");

    // Test error handling with logging
    let error = Error::config("Integration test error");
    logger.error(&format!("Error occurred: {}", error));

    // Test utility functions with logging
    let uuid = crypto::generate_uuid();
    logger.info(&format!("Generated UUID: {}", uuid));

    let now = date::now();
    let rfc3339_time = date::format_rfc3339(now);
    logger.info(&format!("Current time: {}", rfc3339_time));

    // Test validation with logging
    let email = "test@example.com";
    if validation::is_valid_email(email) {
        logger.info(&format!("Email {} is valid", email));
    } else {
        logger.warn(&format!("Email {} is invalid", email));
    }

    logger.info("Integration test completed");
}

#[tokio::test]
async fn test_prelude_functionality() {
    // Test: Prelude module provides convenient access to all components
    use common_library::prelude::*;

    // Test that we can use all the re-exported types
    let _config_manager = ConfigManager::new().expect("ConfigManager should be available");
    let _logger = Logger::new("prelude_test");
    let _error = Error::generic("test error");

    // Test utility functions are available
    let uuid = crypto::generate_uuid();
    assert!(
        !uuid.to_string().is_empty(),
        "Crypto utilities should be available"
    );

    let now = date::now();
    assert!(now.timestamp() > 0, "Date utilities should be available");

    let truncated = string::truncate("test", 2);
    assert_eq!(truncated, "...", "String utilities should be available");

    let is_valid = validation::is_valid_email("test@example.com");
    assert!(is_valid, "Validation utilities should be available");
}

#[tokio::test]
async fn test_error_recovery_patterns() {
    // Test: Error recovery patterns work correctly
    let config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Test graceful error handling
    let result: Result<u32> = config_manager.get("nonexistent.key");
    match result {
        Ok(_) => panic!("Should not find nonexistent key"),
        Err(Error::ConfigParse(_)) => {
            // Expected error type for missing config keys
        }
        Err(e) => panic!("Unexpected error type: {}", e),
    }

    // Test error propagation
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let common_error: Error = io_error.into();

    match common_error {
        Error::Io(msg) => {
            assert!(
                msg.contains("access denied"),
                "Error message should be preserved"
            );
        }
        _ => panic!("Should be an Io error"),
    }
}

#[tokio::test]
async fn test_runtime_configuration_changes() {
    // Test: Runtime configuration changes work correctly
    let mut config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Test setting runtime configuration values
    config_manager.set("app.name", "runtime-test").expect("Should set app.name");
    config_manager.set("app.version", "2.0.0").expect("Should set app.version");
    config_manager.set("http.timeout_seconds", 60u64).expect("Should set http.timeout_seconds");
    config_manager.set("database.max_connections", 20u32).expect("Should set database.max_connections");

    // Test retrieving runtime configuration values
    let app_name: String = config_manager.get("app.name").expect("Should get app.name");
    let app_version: String = config_manager.get("app.version").expect("Should get app.version");
    let http_timeout: u64 = config_manager.get("http.timeout_seconds").expect("Should get http.timeout_seconds");
    let max_connections: u32 = config_manager.get("database.max_connections").expect("Should get database.max_connections");

    assert_eq!(app_name, "runtime-test", "Runtime app.name should match");
    assert_eq!(app_version, "2.0.0", "Runtime app.version should match");
    assert_eq!(http_timeout, 60, "Runtime http.timeout_seconds should match");
    assert_eq!(max_connections, 20, "Runtime database.max_connections should match");

    // Test that runtime overrides take precedence over default values
    let default_timeout: u64 = config_manager.get("http.timeout_seconds").expect("Should get runtime override");
    assert_eq!(default_timeout, 60, "Runtime override should take precedence");

    // Test getting override keys
    let override_keys = config_manager.get_override_keys();
    assert_eq!(override_keys.len(), 4, "Should have 4 runtime overrides");
    assert!(override_keys.contains(&"app.name".to_string()), "Should contain app.name override");
    assert!(override_keys.contains(&"app.version".to_string()), "Should contain app.version override");
    assert!(override_keys.contains(&"http.timeout_seconds".to_string()), "Should contain http.timeout_seconds override");
    assert!(override_keys.contains(&"database.max_connections".to_string()), "Should contain database.max_connections override");
}

#[tokio::test]
async fn test_runtime_configuration_override_behavior() {
    // Test: Runtime overrides behave correctly with different types
    let mut config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Test string overrides
    config_manager.set("app.name", "override-test").expect("Should set string override");
    let app_name: String = config_manager.get("app.name").expect("Should get string override");
    assert_eq!(app_name, "override-test", "String override should work");

    // Test numeric overrides
    config_manager.set("http.timeout_seconds", 120u64).expect("Should set numeric override");
    let timeout: u64 = config_manager.get("http.timeout_seconds").expect("Should get numeric override");
    assert_eq!(timeout, 120, "Numeric override should work");

    // Test boolean overrides
    config_manager.set("storage.backup_enabled", true).expect("Should set boolean override");
    let backup_enabled: bool = config_manager.get("storage.backup_enabled").expect("Should get boolean override");
    assert_eq!(backup_enabled, true, "Boolean override should work");

    // Test that overrides can be changed
    config_manager.set("app.name", "changed-test").expect("Should change string override");
    let changed_name: String = config_manager.get("app.name").expect("Should get changed override");
    assert_eq!(changed_name, "changed-test", "Changed override should work");
}

#[tokio::test]
async fn test_runtime_configuration_clear_overrides() {
    // Test: Clearing runtime overrides works correctly
    let mut config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Set some runtime overrides
    config_manager.set("app.name", "test-app").expect("Should set override");
    config_manager.set("http.timeout_seconds", 90u64).expect("Should set override");

    // Verify overrides are set
    let override_keys = config_manager.get_override_keys();
    assert_eq!(override_keys.len(), 2, "Should have 2 overrides");

    // Clear overrides
    config_manager.clear_overrides();

    // Verify overrides are cleared
    let cleared_keys = config_manager.get_override_keys();
    assert_eq!(cleared_keys.len(), 0, "Should have no overrides after clearing");

    // Verify that original configuration values are returned for existing keys
    let timeout: u64 = config_manager.get("http.timeout_seconds").expect("Should get original value");
    assert_ne!(timeout, 90, "Should not return override value after clearing");

    // Verify that non-existent keys return errors after clearing
    let result: common_library::Result<String> = config_manager.get("app.name");
    assert!(result.is_err(), "Should return error for non-existent key after clearing");
}

#[tokio::test]
async fn test_runtime_configuration_reload() {
    // Test: Reloading configuration clears runtime overrides
    let mut config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Set runtime overrides
    config_manager.set("app.name", "reload-test").expect("Should set override");
    config_manager.set("http.timeout_seconds", 45u64).expect("Should set override");

    // Verify overrides are set
    let override_keys = config_manager.get_override_keys();
    assert_eq!(override_keys.len(), 2, "Should have 2 overrides");

    // Reload configuration
    config_manager.reload().expect("Should reload configuration");

    // Verify overrides are cleared after reload
    let cleared_keys = config_manager.get_override_keys();
    assert_eq!(cleared_keys.len(), 0, "Should have no overrides after reload");
}

#[tokio::test]
async fn test_runtime_configuration_type_safety() {
    // Test: Runtime configuration maintains type safety
    let mut config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Test correct type setting and getting
    config_manager.set("http.timeout_seconds", 30u64).expect("Should set u64");
    let timeout: u64 = config_manager.get("http.timeout_seconds").expect("Should get u64");
    assert_eq!(timeout, 30, "u64 should work correctly");

    // Test type mismatch error
    config_manager.set("http.timeout_seconds", "invalid").expect("Should set string");
    let result: common_library::Result<u64> = config_manager.get("http.timeout_seconds");
    assert!(result.is_err(), "Should fail when trying to get u64 from string value");
}

#[tokio::test]
async fn test_runtime_configuration_serialization() {
    // Test: Runtime configuration values are properly serialized/deserialized
    let mut config_manager = ConfigManager::new().expect("ConfigManager should be created");

    // Test complex data types
    let complex_data = serde_json::json!({
        "nested": {
            "value": 42,
            "enabled": true
        },
        "array": [1, 2, 3]
    });

    config_manager.set("complex.config", complex_data.clone()).expect("Should set complex data");
    let retrieved: serde_json::Value = config_manager.get("complex.config").expect("Should get complex data");
    assert_eq!(retrieved, complex_data, "Complex data should be preserved");

    // Test that the override is stored correctly
    let override_keys = config_manager.get_override_keys();
    assert!(override_keys.contains(&"complex.config".to_string()), "Should contain complex config override");
}

#[tokio::test]
async fn test_logging_performance_metrics() {
    // Test: Performance logging works correctly
    let logger = Logger::new("performance_test");

    // Test performance logging with different durations
    let short_duration = std::time::Duration::from_millis(10);
    logger.log_performance("short_operation", short_duration);

    let long_duration = std::time::Duration::from_millis(100);
    logger.log_performance("long_operation", long_duration);

    // Test that performance logging doesn't panic
    let zero_duration = std::time::Duration::from_nanos(0);
    logger.log_performance("zero_duration", zero_duration);
}
