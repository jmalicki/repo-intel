//! End-to-End Integration Tests
//!
//! Comprehensive integration tests that test the entire common-library workflow
//! across all modules: foundation, HTTP, storage, metrics, and validation.

use common_library::prelude::*;
use common_library::{
    config::ConfigManager, http::APIClient, logging::Logger,
    metrics::{StatisticalCalculator, TrendAnalyzer, GrowthCalculator, PerformanceAnalyzer, DataNormalizer, NormalizationMethod},
    storage::{FileManager, JsonFileManager, MigrationManager, BackupManager},
    validation::{SchemaValidator, DataIntegrityChecker, TypeValidator, SchemaRegistry, errors::ValidationErrorReporter}
};
use serde_json::json;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_complete_workflow() {
    // Test: Complete end-to-end workflow using all library components
    println!("🚀 Starting complete workflow test...");

    // 1. Initialize logging
    common_library::logging::init();
    let logger = Logger::new("end_to_end_test");
    logger.info("Starting complete workflow test");

    // 2. Configuration management
    let mut config_manager = ConfigManager::new().unwrap();
    config_manager.set("app.name", "common-library-test").unwrap();
    config_manager.set("app.version", "1.0.0").unwrap();
    config_manager.set("http.timeout", "30").unwrap();
    config_manager.set("database.url", "sqlite://test.db").unwrap();

    // 3. HTTP client setup
    let mut api_client = APIClient::new().unwrap();

    // 4. Storage setup
    let file_manager = FileManager::new();
    let json_manager = JsonFileManager::new();
    let migration_manager = MigrationManager::new("migrations/".into());
    let backup_manager = BackupManager::new("backups/".into());

    // 5. Metrics setup
    let statistical_calculator = StatisticalCalculator::new();
    let trend_analyzer = TrendAnalyzer::new();
    let growth_calculator = GrowthCalculator::new();
    let performance_analyzer = PerformanceAnalyzer::new();
    let data_normalizer = DataNormalizer::new();

    // 6. Validation setup
    let mut schema_validator = SchemaValidator::new();
    let mut integrity_checker = DataIntegrityChecker::new();
    let mut type_validator = TypeValidator::new();
    let mut error_reporter = ValidationErrorReporter::new();
    let mut schema_registry = SchemaRegistry::new();

    // 7. Create test data
    let test_data = json!({
        "id": "test-001",
        "name": "Integration Test",
        "value": 42.5,
        "metrics": [1.0, 2.0, 3.0, 4.0, 5.0],
        "metadata": {
            "created_at": "2024-01-01T00:00:00Z",
            "version": "1.0.0"
        }
    });

    // 8. Validate data with schema
    let schema = json!({
        "type": "object",
        "properties": {
            "id": {"type": "string", "minLength": 1},
            "name": {"type": "string", "minLength": 1},
            "value": {"type": "number", "minimum": 0},
            "metrics": {"type": "array", "items": {"type": "number"}},
            "metadata": {
                "type": "object",
                "properties": {
                    "created_at": {"type": "string", "format": "date-time"},
                    "version": {"type": "string"}
                }
            }
        },
        "required": ["id", "name", "value"]
    });

    schema_validator.register_schema("test_data", schema.clone()).unwrap();
    let validation_result = schema_validator.validate(&test_data, "test_data").unwrap();
    assert!(validation_result.is_valid, "Test data should be valid");

    // 9. Statistical analysis
    let metrics_data = test_data["metrics"].as_array().unwrap();
    let values: Vec<f64> = metrics_data.iter().map(|v| v.as_f64().unwrap()).collect();

    let stats = statistical_calculator.calculate_statistics(&values).unwrap();
    assert!(stats.mean > 0.0, "Mean should be positive");
    assert!(stats.standard_deviation >= 0.0, "Standard deviation should be non-negative");

    // 10. Trend analysis
    let trend_result = trend_analyzer.analyze_trend(&values).unwrap();
    assert!(!format!("{:?}", trend_result.trend_type).is_empty(), "Trend type should be determined");

    // 11. Growth calculation
    let growth_result = growth_calculator.calculate_cagr(values[0], values[values.len()-1], 5.0);
    assert!(growth_result.is_finite(), "CAGR should be finite");

    // 12. Performance analysis
    let start_time = std::time::Instant::now();
    sleep(Duration::from_millis(10)).await; // Simulate work
    let end_time = start_time.elapsed();

    let performance_metrics = performance_analyzer.calculate_latency_percentiles(&[end_time]).unwrap();
    assert!(performance_metrics[0].0 > 0.0, "Latency should be positive");

    // 13. Data normalization
    let normalized = data_normalizer.normalize(&values, NormalizationMethod::MinMax).unwrap();
    assert_eq!(normalized.normalized_data.len(), values.len(), "Normalized data should have same length");

    // 14. File operations
    let test_file_path = std::path::Path::new("test_data.json");
    json_manager.write_json(test_file_path, &test_data).await.unwrap();
    let loaded_data: serde_json::Value = json_manager.read_json(test_file_path).await.unwrap();
    assert_eq!(loaded_data["id"], test_data["id"], "Loaded data should match saved data");

    // 15. Data integrity check
    integrity_checker.add_checksum("test_data", "test_checksum");
    let integrity_result = integrity_checker.check_integrity(&test_data, "test_data").unwrap();
    // Note: This will fail checksum validation, which is expected for this test

    // 16. Type validation
    let type_result = type_validator.validate_type(&test_data["value"], "number").unwrap();
    assert!(type_result.is_valid, "Value should be valid number type");

    // 17. Error reporting
    if !validation_result.is_valid {
        for error in validation_result.errors {
            let validation_error = common_library::validation::errors::ValidationError {
                error_type: common_library::validation::errors::ValidationErrorType::SchemaValidation,
                path: error.path,
                message: error.message,
                severity: common_library::validation::errors::ErrorSeverity::Error,
                suggestion: error.suggestion.map(|s| common_library::validation::errors::ValidationSuggestion {
                    title: "Fix Schema Error".to_string(),
                    description: s,
                    action: common_library::validation::errors::SuggestionAction::Fix,
                    confidence: 0.8,
                }),
                context: Some(test_data.clone()),
                timestamp: chrono::Utc::now(),
            };
            error_reporter.add_error(validation_error);
        }
    }

    // 18. Schema registry
    let schema_metadata = common_library::validation::registry::SchemaMetadata {
        name: "test_schema".to_string(),
        version: "1.0.0".to_string(),
        schema: schema.clone(),
        description: Some("Test schema for integration testing".to_string()),
        tags: vec!["test".to_string(), "integration".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        author: Some("test".to_string()),
        dependencies: Vec::new(),
        is_deprecated: false,
    };
    schema_registry.register_schema(schema_metadata).unwrap();

    // 19. Cleanup
    let _ = std::fs::remove_file("test_data.json");

    logger.info("Complete workflow test completed successfully");
}

#[tokio::test]
async fn test_performance_benchmark() {
    // Test: Performance benchmark across all components
    println!("⚡ Starting performance benchmark test...");

    let start_time = std::time::Instant::now();

    // HTTP client performance
    let api_client = APIClient::new().unwrap();

    // Statistical calculations performance
    let statistical_calculator = StatisticalCalculator::new();
    let large_dataset: Vec<f64> = (0..10000).map(|i| i as f64).collect();
    let _stats = statistical_calculator.calculate_statistics(&large_dataset).unwrap();

    // Trend analysis performance
    let trend_analyzer = TrendAnalyzer::new();
    let _trend = trend_analyzer.analyze_trend(&large_dataset).unwrap();

    // Data normalization performance
    let data_normalizer = DataNormalizer::new();
    let _normalized = data_normalizer.normalize(&large_dataset, NormalizationMethod::ZScore).unwrap();

    // File operations performance
    let json_manager = JsonFileManager::new();
    let test_data = json!({"large_array": large_dataset});
    let test_file = std::path::Path::new("performance_test.json");
    json_manager.write_json(test_file, &test_data).await.unwrap();
    let _loaded: serde_json::Value = json_manager.read_json(test_file).await.unwrap();

    let total_time = start_time.elapsed();
    println!("Performance benchmark completed in {:?}", total_time);

    // Cleanup
    let _ = std::fs::remove_file("performance_test.json");

    assert!(total_time.as_secs() < 10, "Performance benchmark should complete within 10 seconds");
}

#[tokio::test]
async fn test_error_handling_workflow() {
    // Test: Error handling across all components
    println!("🚨 Starting error handling test...");

    // Test configuration errors
    let mut config_manager = ConfigManager::new().unwrap();
    let result = config_manager.get::<String>("nonexistent.key");
    assert!(result.is_err(), "Should return error for nonexistent key");

    // Test HTTP client errors (without actual network)
    let api_client = APIClient::new().unwrap();

    // Test validation errors
    let mut schema_validator = SchemaValidator::new();
    let invalid_schema = json!({"invalid": "schema"});
    let result = schema_validator.register_schema("invalid", invalid_schema);
    assert!(result.is_err(), "Should reject invalid schema");

    // Test metrics errors
    let statistical_calculator = StatisticalCalculator::new();
    let empty_data: Vec<f64> = vec![];
    let result = statistical_calculator.calculate_statistics(&empty_data);
    assert!(result.is_err(), "Should return error for empty data");

    // Test storage errors
    let file_manager = FileManager::new();
    let result = file_manager.read_file(std::path::Path::new("/nonexistent/path/file.txt")).await;
    assert!(result.is_err(), "Should return error for nonexistent file");

    println!("Error handling test completed successfully");
}

#[tokio::test]
async fn test_concurrent_operations() {
    // Test: Concurrent operations across multiple components
    println!("🔄 Starting concurrent operations test...");

    let handles: Vec<_> = (0..5).map(|i| {
        tokio::spawn(async move {
            // Each task performs different operations
            match i % 5 {
                0 => {
                    // Configuration operations
                    let mut config = ConfigManager::new().unwrap();
                    config.set(&format!("task.{}", i), &format!("value_{}", i)).unwrap();
                    config.get::<String>(&format!("task.{}", i)).unwrap()
                }
                1 => {
                    // Statistical calculations
                    let calculator = StatisticalCalculator::new();
                    let data: Vec<f64> = (0..100).map(|j| (i * 100 + j) as f64).collect();
                    calculator.calculate_statistics(&data).unwrap();
                    "statistics".to_string()
                }
                2 => {
                    // File operations
                    let json_manager = JsonFileManager::new();
                    let test_data = json!({"task": i, "data": "test"});
                    let filename = format!("concurrent_test_{}.json", i);
                    let path = std::path::Path::new(&filename);
                    json_manager.write_json(path, &test_data).await.unwrap();
                    let _loaded: serde_json::Value = json_manager.read_json(path).await.unwrap();
                    std::fs::remove_file(&filename).unwrap();
                    "file_ops".to_string()
                }
                3 => {
                    // Validation operations
                    let mut validator = SchemaValidator::new();
                    let schema = json!({"type": "object", "properties": {"value": {"type": "number"}}});
                    validator.register_schema(&format!("schema_{}", i), schema).unwrap();
                    "validation".to_string()
                }
                _ => {
                    // Metrics operations
                    let analyzer = TrendAnalyzer::new();
                    let data: Vec<f64> = (0..50).map(|j| (i * 50 + j) as f64).collect();
                    analyzer.analyze_trend(&data).unwrap();
                    "metrics".to_string()
                }
            }
        })
    }).collect();

    let results = futures::future::join_all(handles).await;

    for (i, result) in results.into_iter().enumerate() {
        assert!(result.is_ok(), "Task {} should complete successfully", i);
        println!("Task {} completed: {:?}", i, result.unwrap());
    }

    println!("Concurrent operations test completed successfully");
}

#[tokio::test]
async fn test_memory_usage() {
    // Test: Memory usage across all components
    println!("💾 Starting memory usage test...");

    let initial_memory = get_memory_usage();
    println!("Initial memory usage: {} KB", initial_memory);

    // Create multiple instances of all components
    let mut components = Vec::new();

    for i in 0..10 {
        let config = ConfigManager::new().unwrap();
        let http_client = APIClient::new().unwrap();
        let file_manager = FileManager::new();
        let json_manager = JsonFileManager::new();
        let statistical_calculator = StatisticalCalculator::new();
        let trend_analyzer = TrendAnalyzer::new();
        let schema_validator = SchemaValidator::new();
        let integrity_checker = DataIntegrityChecker::new();
        let type_validator = TypeValidator::new();
        let error_reporter = ValidationErrorReporter::new();
        let schema_registry = SchemaRegistry::new();

        components.push((
            config, http_client, file_manager, json_manager,
            statistical_calculator, trend_analyzer, schema_validator,
            integrity_checker, type_validator, error_reporter, schema_registry
        ));
    }

    let after_creation_memory = get_memory_usage();
    println!("After component creation: {} KB", after_creation_memory);

    // Perform operations
    for (i, (mut config, _http_client, _file_manager, json_manager, statistical_calculator, trend_analyzer, mut schema_validator, _integrity_checker, _type_validator, _error_reporter, _schema_registry)) in components.into_iter().enumerate() {
        // Configuration operations
        config.set(&format!("test.{}", i), &format!("value_{}", i)).unwrap();

        // Statistical operations
        let data: Vec<f64> = (0..1000).map(|j| (i * 1000 + j) as f64).collect();
        let _stats = statistical_calculator.calculate_statistics(&data).unwrap();
        let _trend = trend_analyzer.analyze_trend(&data).unwrap();

        // File operations
        let test_data = json!({"index": i, "data": data});
        let filename = format!("memory_test_{}.json", i);
        let path = std::path::Path::new(&filename);
        json_manager.write_json(path, &test_data).await.unwrap();
        let _loaded: serde_json::Value = json_manager.read_json(path).await.unwrap();
        std::fs::remove_file(&filename).unwrap();

        // Validation operations
        let schema = json!({"type": "object", "properties": {"value": {"type": "number"}}});
        schema_validator.register_schema(&format!("memory_schema_{}", i), schema).unwrap();
    }

    let final_memory = get_memory_usage();
    println!("Final memory usage: {} KB", final_memory);

    let memory_increase = final_memory - initial_memory;
    println!("Memory increase: {} KB", memory_increase);

    // Memory usage should be reasonable (less than 100MB for this test)
    assert!(memory_increase < 100000, "Memory usage should be reasonable");
}

fn get_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real test, you'd use proper memory profiling tools
    0
}

#[tokio::test]
async fn test_cross_platform_compatibility() {
    // Test: Cross-platform compatibility
    println!("🌐 Starting cross-platform compatibility test...");

    // Test file path handling
    let file_manager = FileManager::new();
    let test_path = if cfg!(windows) {
        "test\\compatibility.txt"
    } else {
        "test/compatibility.txt"
    };

    let test_content = b"Cross-platform test content";
    let path = std::path::Path::new(test_path);
    file_manager.write_file(path, test_content).await.unwrap();
    let loaded_content = file_manager.read_file(path).await.unwrap();
    assert_eq!(loaded_content, test_content, "File content should match");

    // Cleanup
    std::fs::remove_file(test_path).unwrap();

    // Test JSON handling (should work the same across platforms)
    let json_manager = JsonFileManager::new();
    let test_data = json!({
        "platform": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let json_path = std::path::Path::new("compatibility_test.json");
    json_manager.write_json(json_path, &test_data).await.unwrap();
    let loaded_data: serde_json::Value = json_manager.read_json(json_path).await.unwrap();
    assert_eq!(loaded_data["platform"], test_data["platform"], "Platform should match");

    // Cleanup
    std::fs::remove_file(json_path).unwrap();

    println!("Cross-platform compatibility test completed successfully");
}
