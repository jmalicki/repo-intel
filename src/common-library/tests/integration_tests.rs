//! Integration tests for the common library

use common_library::prelude::*;

#[tokio::test]
async fn test_config_manager_creation() {
    // Test: Configuration manager can be created with default settings
    let config_manager = ConfigManager::new();
    assert!(
        config_manager.is_ok(),
        "ConfigManager should be created successfully"
    );

    let config = config_manager.unwrap();
    let app_config = config.get_app_config();
    assert!(
        app_config.is_ok(),
        "Should be able to get app configuration"
    );
}

#[tokio::test]
async fn test_config_validation() {
    // Test: Configuration validation works correctly
    let config_manager = ConfigManager::new().expect("Failed to create config manager");
    let validation_result = config_manager.validate();
    assert!(
        validation_result.is_ok(),
        "Default configuration should be valid"
    );
}

#[tokio::test]
async fn test_logger_creation() {
    // Test: Logger can be created and used
    let logger = Logger::new("test");
    logger.info("Test info message");
    logger.warn("Test warning message");
    logger.error("Test error message");
    logger.debug("Test debug message");
}

#[tokio::test]
async fn test_utils_functions() {
    // Test: Utility functions work correctly

    // Test date utilities
    let now = date::now();
    assert!(now.timestamp() > 0, "Current timestamp should be positive");

    let timestamp_str = date::format_timestamp(now);
    assert!(
        !timestamp_str.is_empty(),
        "Formatted timestamp should not be empty"
    );

    // Test crypto utilities
    let uuid = crypto::generate_uuid();
    assert!(
        !uuid.to_string().is_empty(),
        "Generated UUID should not be empty"
    );

    let random_string = crypto::generate_random_string(10);
    assert_eq!(
        random_string.len(),
        10,
        "Random string should have correct length"
    );

    // Test string utilities
    let test_string = "Hello World";
    let truncated = string::truncate(test_string, 5);
    assert_eq!(truncated, "He...", "String should be truncated correctly");

    assert!(string::is_blank("   "), "Blank string should be detected");
    assert!(
        !string::is_blank("hello"),
        "Non-blank string should not be detected"
    );

    // Test validation utilities
    assert!(
        validation::is_valid_email("test@example.com"),
        "Valid email should pass validation"
    );
    assert!(
        !validation::is_valid_email("invalid-email"),
        "Invalid email should fail validation"
    );

    assert!(
        validation::is_valid_url("https://example.com"),
        "Valid URL should pass validation"
    );
    assert!(
        !validation::is_valid_url("invalid-url"),
        "Invalid URL should fail validation"
    );
}

#[tokio::test]
async fn test_error_handling() {
    // Test: Error handling works correctly
    let config_error = Error::config("Test configuration error");
    assert!(config_error.to_string().contains("Configuration error"));

    let http_error = Error::http("Test HTTP error");
    assert!(http_error.to_string().contains("HTTP error"));

    let generic_error = Error::generic("Test generic error");
    assert!(generic_error.to_string().contains("Generic error"));
}

#[tokio::test]
async fn test_module_loading() {
    // Test: All modules can be loaded and used
    use common_library::config::ConfigManager;
    use common_library::error::Error;
    use common_library::logging::Logger;
    use common_library::utils::*;

    // Test that we can create instances of all main types
    let _config = ConfigManager::new().expect("Failed to create config manager");
    let _logger = Logger::new("test");
    let _error: Error = Error::generic("test");

    // Test that utility functions are accessible
    let _now = date::now();
    let _uuid = crypto::generate_uuid();
    let _truncated = string::truncate("test", 2);
}
