//! Basic Usage Examples
//!
//! This example demonstrates the basic usage of the common-library crate
//! across all its modules: configuration, logging, HTTP client, storage,
//! metrics, and validation.

use common_library::prelude::*;
use serde_json::json;
use tokio;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Common Library - Basic Usage Example");
    println!("=====================================");

    // 1. Initialize logging
    common_library::logging::init();
    let logger = Logger::new("basic_usage");
    logger.info("Starting basic usage example");

    // 2. Configuration Management
    println!("\n📋 Configuration Management");
    let mut config = ConfigManager::new()?;

    // Set configuration values
    config.set("app.name", "common-library-example")?;
    config.set("app.version", "1.0.0")?;
    config.set("http.timeout_seconds", 30u64)?;
    config.set("database.url", "sqlite://example.db")?;

    // Retrieve configuration values
    let app_name: String = config.get("app.name")?;
    let app_version: String = config.get("app.version")?;
    let http_timeout: u64 = config.get("http.timeout_seconds")?;

    println!("App Name: {}", app_name);
    println!("App Version: {}", app_version);
    println!("HTTP Timeout: {} seconds", http_timeout);

    // 3. HTTP Client Usage
    println!("\n🌐 HTTP Client");
    let mut api_client = APIClient::new()?;

    // Note: This is a mock example - in real usage, you'd make actual HTTP requests
    println!("HTTP client configured successfully");

    // 4. Storage Operations
    println!("\n💾 Storage Operations");
    let file_manager = FileManager::new();
    let json_manager = JsonFileManager::new();

    // Create test data
    let test_data = json!({
        "id": "example-001",
        "name": "Basic Usage Example",
        "value": 42.5,
        "metrics": [1.0, 2.0, 3.0, 4.0, 5.0],
        "metadata": {
            "created_at": "2024-01-01T00:00:00Z",
            "version": "1.0.0"
        }
    });

    // Write JSON data
    let test_file = std::path::Path::new("example_data.json");
    json_manager.write_json(test_file, &test_data).await?;
    println!("Data written to example_data.json");

    // Read JSON data
    let loaded_data: serde_json::Value = json_manager.read_json(test_file).await?;
    println!("Data loaded: {}", loaded_data["name"]);

    // Cleanup
    std::fs::remove_file("example_data.json")?;

    // 5. Metrics and Statistical Analysis
    println!("\n📊 Metrics and Statistical Analysis");
    let statistical_calculator = StatisticalCalculator::new();
    let trend_analyzer = TrendAnalyzer::new();
    let growth_calculator = GrowthCalculator::new();
    let performance_analyzer = PerformanceAnalyzer::new();
    let data_normalizer = DataNormalizer::new();

    // Sample data for analysis
    let sample_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

    // Statistical calculations
    let stats = statistical_calculator.calculate_statistics(&sample_data)?;
    println!("Mean: {:.2}", stats.mean);
    println!("Standard Deviation: {:.2}", stats.standard_deviation);
    println!("Median: {:.2}", stats.median);

    // Trend analysis
    let trend = trend_analyzer.analyze_trend(&sample_data)?;
    println!("Trend Type: {:?}", trend.trend_type);
    println!("Trend R-squared: {:.2}", trend.r_squared);

    // Growth calculation
    let growth = growth_calculator.calculate_cagr(sample_data[0], sample_data[sample_data.len()-1], 5.0);
    println!("CAGR: {:.2}%", growth * 100.0);

    // Performance analysis
    let start_time = std::time::Instant::now();
    tokio::time::sleep(Duration::from_millis(100)).await;
    let latency = start_time.elapsed();
    let perf_metrics = performance_analyzer.calculate_latency_percentiles(&[latency])?;
    println!("Latency: {:.2} ms", perf_metrics[0].0 * 1000.0);

    // Data normalization
    let normalized = data_normalizer.normalize(&sample_data, NormalizationMethod::MinMax)?;
    println!("Normalized data (first 3 values): {:?}", &normalized.normalized_data[..3]);

    // 6. Data Validation
    println!("\n✅ Data Validation");
    let mut schema_validator = SchemaValidator::new();
    let mut integrity_checker = DataIntegrityChecker::new();
    let mut type_validator = TypeValidator::new();
    let mut error_reporter = common_library::validation::errors::ValidationErrorReporter::new();

    // Define schema
    let schema = json!({
        "type": "object",
        "properties": {
            "id": {"type": "string", "minLength": 1},
            "name": {"type": "string", "minLength": 1},
            "value": {"type": "number", "minimum": 0},
            "metrics": {"type": "array", "items": {"type": "number"}}
        },
        "required": ["id", "name", "value"]
    });

    // Register schema
    schema_validator.register_schema("example_schema", schema)?;

    // Validate data
    let validation_result = schema_validator.validate(&test_data, "example_schema")?;
    if validation_result.is_valid {
        println!("✅ Data validation passed");
    } else {
        println!("❌ Data validation failed with {} errors", validation_result.errors.len());
        for error in validation_result.errors {
            println!("  - {}: {}", error.path, error.message);
        }
    }

    // Type validation
    let type_result = type_validator.validate_type(&test_data["value"], "number")?;
    if type_result.is_valid {
        println!("✅ Type validation passed");
    } else {
        println!("❌ Type validation failed");
    }

    // Data integrity check
    integrity_checker.add_checksum("example_data", "example_checksum");
    let integrity_result = integrity_checker.check_integrity(&test_data, "example_data")?;
    println!("Data integrity score: {:.1}%", integrity_result.consistency_score);

    // 7. Error Handling Example
    println!("\n🚨 Error Handling");
    match config.get::<String>("nonexistent.key") {
        Ok(value) => println!("Unexpected success: {}", value),
        Err(Error::ConfigParse(msg)) => println!("Expected config error: {}", msg),
        Err(e) => println!("Other error: {}", e),
    }

    // 8. Migration Example (if database feature is enabled)
    #[cfg(feature = "database")]
    {
        println!("\n🗄️ Database Migration");
        let migration_manager = MigrationManager::new("migrations/".into());

        // Create a simple migration
        let migration_sql = "CREATE TABLE IF NOT EXISTS example_table (id INTEGER PRIMARY KEY, name TEXT)";
        migration_manager.create_migration("create_example_table", migration_sql).await?;
        println!("Migration created successfully");
    }

    // 9. Backup Example
    println!("\n💾 Backup Operations");
    let backup_manager = BackupManager::new("backups/".into());

    // Create a backup of the test data
    let backup_result = backup_manager.create_full_backup(
        &[std::path::PathBuf::from("example_data.json")],
        "example_backup",
        common_library::storage::backup::BackupCompression::None
    ).await?;
    println!("Backup created: {}", backup_result.id);

    logger.info("Basic usage example completed successfully");
    println!("\n🎉 Example completed successfully!");

    Ok(())
}
