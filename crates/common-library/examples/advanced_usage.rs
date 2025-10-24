//! Advanced Usage Examples
//!
//! This example demonstrates advanced usage patterns of the common-library crate,
//! including complex workflows, error handling, performance optimization, and
//! integration between multiple modules.

use common_library::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use tokio;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Common Library - Advanced Usage Example");
    println!("==========================================");

    // Initialize logging with custom configuration
    common_library::logging::init();
    let logger = Logger::new("advanced_usage");
    logger.info("Starting advanced usage example");

    // 1. Advanced Configuration Management
    println!("\n📋 Advanced Configuration");
    let mut config = ConfigManager::new();

    // Load configuration from multiple sources
    config.set("app.name", "advanced-example")?;
    config.set("app.version", "2.0.0")?;
    config.set("app.environment", "development")?;

    // HTTP configuration
    config.set("http.timeout", "60")?;
    config.set("http.max_retries", "5")?;
    config.set("http.rate_limit", "1000")?;
    config.set("http.base_url", "https://api.example.com")?;

    // Database configuration
    config.set("database.url", "sqlite://advanced.db")?;
    config.set("database.pool_size", "10")?;
    config.set("database.timeout", "30")?;

    // Metrics configuration
    config.set("metrics.enabled", "true")?;
    config.set("metrics.interval", "60")?;
    config.set("metrics.retention_days", "30")?;

    // Validation configuration
    config.set("validation.strict_mode", "true")?;
    config.set("validation.schema_cache_size", "100")?;

    println!("Configuration loaded with {} keys", config.keys()?.len());

    // 2. Advanced HTTP Client with Authentication
    println!("\n🌐 Advanced HTTP Client");
    let http_config = HttpClientConfig {
        timeout: Duration::from_secs(60),
        max_retries: 5,
        rate_limit: 1000,
        ..Default::default()
    };
    let mut api_client = APIClient::new(http_config);

    // Configure authentication
    let auth_config = AuthConfig {
        auth_type: AuthType::Bearer,
        token: Some("your-api-token".to_string()),
        username: None,
        password: None,
        api_key: None,
        custom_headers: None,
    };
    api_client.set_auth(auth_config);

    println!("HTTP client configured with authentication");

    // 3. Advanced Storage with Migration Management
    println!("\n💾 Advanced Storage Operations");
    let file_manager = FileManager::new();
    let json_manager = JsonFileManager::new();
    let migration_manager = MigrationManager::new("migrations/");
    let backup_manager = BackupManager::new("backups/");

    // Create complex data structure
    let complex_data = json!({
        "application": {
            "name": "Advanced Example",
            "version": "2.0.0",
            "environment": "development"
        },
        "users": [
            {
                "id": 1,
                "name": "Alice",
                "email": "alice@example.com",
                "metrics": {
                    "login_count": 42,
                    "last_login": "2024-01-01T10:00:00Z",
                    "performance_score": 95.5
                }
            },
            {
                "id": 2,
                "name": "Bob",
                "email": "bob@example.com",
                "metrics": {
                    "login_count": 28,
                    "last_login": "2024-01-02T14:30:00Z",
                    "performance_score": 87.2
                }
            }
        ],
        "system_metrics": {
            "cpu_usage": [45.2, 47.8, 52.1, 48.9, 46.3],
            "memory_usage": [1024, 1089, 1156, 1123, 1098],
            "disk_usage": [85.2, 85.8, 86.1, 85.9, 85.4]
        },
        "metadata": {
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T15:30:00Z",
            "schema_version": "2.0.0"
        }
    });

    // Write complex data
    let data_file = "advanced_data.json";
    json_manager.write_json(data_file, &complex_data).await?;
    println!("Complex data written to {}", data_file);

    // Create migration
    let migration_sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            login_count INTEGER DEFAULT 0,
            last_login TEXT,
            performance_score REAL
        );

        CREATE TABLE IF NOT EXISTS system_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            cpu_usage REAL,
            memory_usage INTEGER,
            disk_usage REAL
        );
    "#;

    migration_manager.create_migration("create_advanced_tables", migration_sql).await?;
    println!("Database migration created");

    // 4. Advanced Metrics and Analytics
    println!("\n📊 Advanced Metrics and Analytics");
    let statistical_calculator = StatisticalCalculator::new();
    let trend_analyzer = TrendAnalyzer::new();
    let growth_calculator = GrowthCalculator::new();
    let performance_analyzer = PerformanceAnalyzer::new();
    let data_normalizer = DataNormalizer::new();

    // Extract metrics from complex data
    let cpu_usage: Vec<f64> = complex_data["system_metrics"]["cpu_usage"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect();

    let memory_usage: Vec<f64> = complex_data["system_metrics"]["memory_usage"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect();

    // Statistical analysis
    let cpu_stats = statistical_calculator.calculate_statistics(&cpu_usage)?;
    let memory_stats = statistical_calculator.calculate_statistics(&memory_usage)?;

    println!("CPU Usage Statistics:");
    println!("  Mean: {:.2}%", cpu_stats.mean);
    println!("  Std Dev: {:.2}%", cpu_stats.standard_deviation);
    println!("  Min: {:.2}%", cpu_stats.min);
    println!("  Max: {:.2}%", cpu_stats.max);

    println!("Memory Usage Statistics:");
    println!("  Mean: {:.2} MB", memory_stats.mean);
    println!("  Std Dev: {:.2} MB", memory_stats.standard_deviation);

    // Trend analysis
    let cpu_trend = trend_analyzer.analyze_trend(&cpu_usage)?;
    let memory_trend = trend_analyzer.analyze_trend(&memory_usage)?;

    println!("CPU Trend: {:?} ({:?})", cpu_trend.trend_type, cpu_trend.strength);
    println!("Memory Trend: {:?} ({:?})", memory_trend.trend_type, memory_trend.strength);

    // Growth analysis
    let cpu_growth = growth_calculator.calculate_cagr(&cpu_usage, 5.0)?;
    let memory_growth = growth_calculator.calculate_cagr(&memory_usage, 5.0)?;

    println!("CPU Growth Rate: {:.2}%", cpu_growth.cagr * 100.0);
    println!("Memory Growth Rate: {:.2}%", memory_growth.cagr * 100.0);

    // Performance analysis
    let start_time = std::time::Instant::now();
    sleep(Duration::from_millis(200)).await;
    let processing_time = start_time.elapsed();

    let perf_metrics = performance_analyzer.calculate_latency(processing_time)?;
    println!("Processing Time: {:.2} ms", perf_metrics.latency_ms);

    // Data normalization
    let normalized_cpu = data_normalizer.normalize(&cpu_usage, NormalizationMethod::ZScore)?;
    let normalized_memory = data_normalizer.normalize(&memory_usage, NormalizationMethod::MinMax)?;

    println!("Normalized CPU (first 3): {:?}", &normalized_cpu[..3]);
    println!("Normalized Memory (first 3): {:?}", &normalized_memory[..3]);

    // 5. Advanced Data Validation
    println!("\n✅ Advanced Data Validation");
    let mut schema_validator = SchemaValidator::new();
    let mut integrity_checker = DataIntegrityChecker::new();
    let mut type_validator = TypeValidator::new();
    let mut error_reporter = ValidationErrorReporter::new();
    let mut schema_registry = SchemaRegistry::new();

    // Define comprehensive schema
    let advanced_schema = json!({
        "type": "object",
        "properties": {
            "application": {
                "type": "object",
                "properties": {
                    "name": {"type": "string, "minLength": 1},
                    "version": {"type": "string", "pattern": "^\\d+\\.\\d+\\.\\d+$"},
                    "environment": {"type": "string", "enum": ["development", "staging", "production"]}
                },
                "required": ["name", "version", "environment"]
            },
            "users": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "integer", "minimum": 1},
                        "name": {"type": "string", "minLength": 1},
                        "email": {"type": "string", "format": "email"},
                        "metrics": {
                            "type": "object",
                            "properties": {
                                "login_count": {"type": "integer", "minimum": 0},
                                "last_login": {"type": "string", "format": "date-time"},
                                "performance_score": {"type": "number", "minimum": 0, "maximum": 100}
                            },
                            "required": ["login_count", "performance_score"]
                        }
                    },
                    "required": ["id", "name", "email", "metrics"]
                },
                "minItems": 1
            },
            "system_metrics": {
                "type": "object",
                "properties": {
                    "cpu_usage": {"type": "array", "items": {"type": "number", "minimum": 0, "maximum": 100}},
                    "memory_usage": {"type": "array", "items": {"type": "number", "minimum": 0}},
                    "disk_usage": {"type": "array", "items": {"type": "number", "minimum": 0, "maximum": 100}}
                },
                "required": ["cpu_usage", "memory_usage", "disk_usage"]
            },
            "metadata": {
                "type": "object",
                "properties": {
                    "created_at": {"type": "string", "format": "date-time"},
                    "updated_at": {"type": "string", "format": "date-time"},
                    "schema_version": {"type": "string", "pattern": "^\\d+\\.\\d+\\.\\d+$"}
                },
                "required": ["created_at", "schema_version"]
            }
        },
        "required": ["application", "users", "system_metrics", "metadata"]
    });

    // Register schema in registry
    let schema_metadata = SchemaMetadata {
        name: "advanced_schema".to_string(),
        version: "2.0.0".to_string(),
        schema: advanced_schema.clone(),
        description: Some("Advanced schema for complex data validation".to_string()),
        tags: vec!["advanced".to_string(), "complex".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        author: Some("advanced-example".to_string()),
        dependencies: Vec::new(),
        is_deprecated: false,
    };
    schema_registry.register_schema(schema_metadata)?;

    // Register schema for validation
    schema_validator.register_schema("advanced_schema", advanced_schema)?;

    // Validate complex data
    let validation_result = schema_validator.validate(&complex_data, "advanced_schema")?;
    if validation_result.is_valid {
        println!("✅ Advanced data validation passed");
    } else {
        println!("❌ Advanced data validation failed with {} errors", validation_result.errors.len());
        for error in validation_result.errors {
            println!("  - {}: {}", error.path, error.message);
        }
    }

    // Add integrity constraints
    let cpu_constraint = IntegrityConstraint {
        name: "cpu_usage_range".to_string(),
        constraint_type: ConstraintType::Range,
        path: "system_metrics.cpu_usage",
        value: Some(json!({"min": 0, "max": 100})),
        severity: ViolationSeverity::High,
    };
    integrity_checker.add_constraint(cpu_constraint);

    let integrity_result = integrity_checker.check_integrity(&complex_data, "advanced_data")?;
    println!("Data integrity score: {:.1}%", integrity_result.consistency_score);

    // Type validation for specific fields
    let user_id_type = type_validator.validate_type(&complex_data["users"][0]["id"], "integer")?;
    let user_name_type = type_validator.validate_type(&complex_data["users"][0]["name"], "string")?;

    println!("User ID type validation: {}", if user_id_type.is_valid { "✅" } else { "❌" });
    println!("User name type validation: {}", if user_name_type.is_valid { "✅" } else { "❌" });

    // 6. Error Handling and Recovery
    println!("\n🚨 Advanced Error Handling");

    // Test various error scenarios
    let error_scenarios = vec![
        ("Invalid configuration key", || config.get::<String>("nonexistent.key")),
        ("Invalid JSON schema", || {
            let invalid_schema = json!({"invalid": "schema"});
            schema_validator.register_schema("invalid", invalid_schema)
        }),
        ("Empty data statistics", || {
            let empty_data: Vec<f64> = vec![];
            statistical_calculator.calculate_statistics(&empty_data)
        }),
        ("Invalid file path", || {
            file_manager.read_file("/nonexistent/path/file.txt")
        }),
    ];

    for (scenario_name, error_func) in error_scenarios {
        match error_func() {
            Ok(_) => println!("  {}: Unexpected success", scenario_name),
            Err(e) => println!("  {}: ✅ Expected error - {}", scenario_name, e),
        }
    }

    // 7. Performance Benchmarking
    println!("\n⚡ Performance Benchmarking");
    let benchmark_data: Vec<f64> = (0..10000).map(|i| i as f64).collect();

    let start_time = std::time::Instant::now();
    let _stats = statistical_calculator.calculate_statistics(&benchmark_data)?;
    let stats_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    let _trend = trend_analyzer.analyze_trend(&benchmark_data)?;
    let trend_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    let _normalized = data_normalizer.normalize(&benchmark_data, NormalizationMethod::ZScore)?;
    let norm_time = start_time.elapsed();

    println!("Statistics calculation: {:?}", stats_time);
    println!("Trend analysis: {:?}", trend_time);
    println!("Data normalization: {:?}", norm_time);

    // 8. Backup and Recovery
    println!("\n💾 Backup and Recovery");
    let backup_result = backup_manager.create_full_backup(
        "advanced_backup",
        data_file,
        BackupStrategy::Full,
        BackupCompression::Gzip
    ).await?;
    println!("Backup created: {}", backup_result.backup_id);

    // List available backups
    let backups = backup_manager.list_backups().await?;
    println!("Available backups: {}", backups.len());

    // 9. Cleanup
    println!("\n🧹 Cleanup");
    std::fs::remove_file(data_file)?;
    println!("Temporary files cleaned up");

    logger.info("Advanced usage example completed successfully");
    println!("\n🎉 Advanced example completed successfully!");

    Ok(())
}
