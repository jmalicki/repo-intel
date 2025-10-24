# Common Library

A comprehensive Rust library providing essential utilities for configuration management, HTTP client operations, storage, metrics, and data validation.

## Features

### 🏗️ Foundation
- **Configuration Management**: Flexible configuration system with multiple sources
- **Error Handling**: Comprehensive error types with detailed error information
- **Logging**: Structured logging with multiple output formats
- **Utilities**: Common utility functions for strings, dates, crypto, and file operations

### 🌐 HTTP Client
- **HTTP Client**: Async HTTP client with retry logic and rate limiting
- **Authentication**: Support for Bearer, Basic, API Key, and custom authentication
- **Rate Limiting**: Configurable rate limiting with semaphore-based control
- **Retry Logic**: Exponential backoff with jitter for robust request handling

### 💾 Storage
- **File Operations**: Async file system operations with error handling
- **JSON Management**: Specialized JSON file operations with serialization
- **Database Support**: Async database operations with connection pooling
- **Migrations**: Database migration management with version control
- **Backup & Restore**: Comprehensive backup and restore functionality

### 📊 Metrics
- **Statistical Analysis**: Mean, median, mode, standard deviation, variance, quartiles
- **Trend Analysis**: Linear regression, moving averages, trend detection
- **Growth Calculations**: CAGR, YoY, QoQ growth rate calculations
- **Performance Metrics**: Latency, throughput, efficiency calculations
- **Data Normalization**: Min-Max, Z-score, Decimal scaling normalization

### ✅ Validation
- **JSON Schema Validation**: Complete JSON Schema Draft 7 compliance
- **Data Integrity**: Checksum verification and constraint validation
- **Type Validation**: Custom type definitions with constraint support
- **Error Reporting**: Intelligent error suggestions and categorization
- **Schema Registry**: Schema versioning and metadata management

## Quick Start

Add the common-library to your `Cargo.toml`:

```toml
[dependencies]
common-library = "0.1.0"
```

### Basic Usage

```rust
use common_library::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    common_library::logging::init();
    let logger = Logger::new("my_app");
    logger.info("Starting application");

    // Configuration management
    let mut config = ConfigManager::new();
    config.set("app.name", "my-application")?;
    config.set("app.version", "1.0.0")?;

    let app_name: String = config.get("app.name")?;
    println!("Application: {}", app_name);

    // HTTP client
    let http_config = HttpClientConfig {
        timeout: Duration::from_secs(30),
        max_retries: 3,
        rate_limit: 100,
        ..Default::default()
    };
    let mut api_client = APIClient::new(http_config);

    // Storage operations
    let json_manager = JsonFileManager::new();
    let data = json!({"message": "Hello, World!"});
    json_manager.write_json("data.json", &data).await?;
    let loaded_data = json_manager.read_json("data.json").await?;
    println!("Data: {}", loaded_data["message"]);

    // Metrics calculations
    let statistical_calculator = StatisticalCalculator::new();
    let sample_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let stats = statistical_calculator.calculate_statistics(&sample_data)?;
    println!("Mean: {:.2}", stats.mean);

    // Data validation
    let mut schema_validator = SchemaValidator::new();
    let schema = json!({
        "type": "object",
        "properties": {
            "message": {"type": "string", "minLength": 1}
        },
        "required": ["message"]
    });
    schema_validator.register_schema("message_schema", schema)?;

    let validation_result = schema_validator.validate(&data, "message_schema")?;
    if validation_result.is_valid {
        println!("✅ Data validation passed");
    }

    Ok(())
}
```

## Configuration

### Environment Variables

The library can be configured using environment variables:

```bash
# Logging configuration
RUST_LOG=info
LOG_FORMAT=json
LOG_LEVEL=info

# HTTP client configuration
HTTP_TIMEOUT=30
HTTP_MAX_RETRIES=3
HTTP_RATE_LIMIT=100

# Database configuration
DATABASE_URL=sqlite://app.db
DATABASE_POOL_SIZE=10

# Metrics configuration
METRICS_ENABLED=true
METRICS_INTERVAL=60
```

### Configuration File

You can also use configuration files:

```toml
# config.toml
[app]
name = "my-application"
version = "1.0.0"
environment = "development"

[http]
timeout = 30
max_retries = 3
rate_limit = 100
base_url = "https://api.example.com"

[database]
url = "sqlite://app.db"
pool_size = 10
timeout = 30

[metrics]
enabled = true
interval = 60
retention_days = 30

[validation]
strict_mode = true
schema_cache_size = 100
```

## Examples

### HTTP Client with Authentication

```rust
use common_library::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_config = HttpClientConfig {
        timeout: Duration::from_secs(30),
        max_retries: 3,
        rate_limit: 100,
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

    // Make HTTP requests
    let response = api_client.get("https://api.example.com/data").await?;
    println!("Response: {}", response);

    Ok(())
}
```

### Statistical Analysis

```rust
use common_library::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calculator = StatisticalCalculator::new();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

    let stats = calculator.calculate_statistics(&data)?;
    println!("Mean: {:.2}", stats.mean);
    println!("Standard Deviation: {:.2}", stats.standard_deviation);
    println!("Median: {:.2}", stats.median);
    println!("Mode: {:.2}", stats.mode);

    // Trend analysis
    let trend_analyzer = TrendAnalyzer::new();
    let trend = trend_analyzer.analyze_trend(&data)?;
    println!("Trend: {:?}", trend.trend_type);

    // Growth calculation
    let growth_calculator = GrowthCalculator::new();
    let growth = growth_calculator.calculate_cagr(&data, 5.0)?;
    println!("CAGR: {:.2}%", growth.cagr * 100.0);

    Ok(())
}
```

### Data Validation

```rust
use common_library::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut schema_validator = SchemaValidator::new();

    // Define schema
    let schema = json!({
        "type": "object",
        "properties": {
            "id": {"type": "string", "minLength": 1},
            "name": {"type": "string", "minLength": 1},
            "age": {"type": "number", "minimum": 0, "maximum": 120},
            "email": {"type": "string", "format": "email"}
        },
        "required": ["id", "name", "age"]
    });

    // Register schema
    schema_validator.register_schema("user_schema", schema)?;

    // Validate data
    let user_data = json!({
        "id": "user-001",
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com"
    });

    let result = schema_validator.validate(&user_data, "user_schema")?;
    if result.is_valid {
        println!("✅ User data is valid");
    } else {
        println!("❌ Validation failed:");
        for error in result.errors {
            println!("  - {}: {}", error.path, error.message);
        }
    }

    Ok(())
}
```

### Database Operations

```rust
use common_library::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database configuration
    let db_config = DatabaseConfig {
        url: "sqlite://app.db".to_string(),
        pool_size: 10,
        timeout: Duration::from_secs(30),
        ..Default::default()
    };

    let db_manager = DatabaseManager::new(db_config).await?;

    // Create migration
    let migration_manager = MigrationManager::new("migrations/");
    let migration_sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL
        );
    "#;

    migration_manager.create_migration("create_users_table", migration_sql).await?;

    // Run migrations
    migration_manager.run_migrations().await?;

    Ok(())
}
```

## Performance

The library is designed for high performance with:

- **Async/Await**: Non-blocking operations throughout
- **Connection Pooling**: Efficient database connection management
- **Rate Limiting**: Built-in rate limiting for HTTP operations
- **Memory Efficient**: Optimized data structures and algorithms
- **Concurrent Safe**: Thread-safe operations for concurrent usage

### Benchmarking

Run the performance benchmarks:

```bash
cargo bench
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test foundation_tests
cargo test --test http_integration_tests
cargo test --test storage_tests
cargo test --test metrics_tests
cargo test --test validation_tests
cargo test --test end_to_end_tests

# Run with documentation
cargo test --doc
```

## Documentation

Generate and view the documentation:

```bash
# Generate documentation
cargo doc --open

# Generate documentation for all features
cargo doc --features "http,database,compression,cli" --open
```

## Features

The library supports optional features:

- `http` - HTTP client functionality (default)
- `database` - Database operations with diesel-async
- `compression` - Compression support for backups
- `cli` - Command-line interface utilities

Enable features in your `Cargo.toml`:

```toml
[dependencies]
common-library = { version = "0.1.0", features = ["http", "database", "compression"] }
```

## Error Handling

The library provides comprehensive error handling:

```rust
use common_library::error::{Error, Result};

fn example_function() -> Result<String> {
    // Operations that can fail
    let config = ConfigManager::new();
    let value: String = config.get("some.key")?;
    Ok(value)
}

// Handle specific error types
match example_function() {
    Ok(value) => println!("Success: {}", value),
    Err(Error::ConfigParse(msg)) => println!("Configuration error: {}", msg),
    Err(Error::Io(msg)) => println!("IO error: {}", msg),
    Err(e) => println!("Other error: {}", e),
}
```

## Contributing

Contributions are welcome! Please see our contributing guidelines for details.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

See the CHANGELOG.md file for a list of changes and version history.

## Support

For support and questions:

- Create an issue on GitHub
- Check the documentation
- Review the examples

## Roadmap

- [ ] Additional database backends
- [ ] More validation formats
- [ ] Advanced metrics algorithms
- [ ] WebSocket support
- [ ] GraphQL integration
- [ ] Machine learning utilities
