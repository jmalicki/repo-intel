# Common Library - Detailed Design

**Parent:** [Common Library](README.md)
**Related:** [GitHub API Collector](../github-api-collector/DESIGN.md) - Primary consumer

## Overview

The Common Library is a **Rust crate** that provides shared functionality for all Phase 1 project selection tools. It eliminates code duplication, ensures consistent behavior, and provides a solid foundation for HTTP clients, data processing, storage, configuration, logging, metrics, and validation.

## Programming Language: Rust

### Justification for Rust
- **Performance**: Zero-cost abstractions for high-performance data processing
- **Memory Safety**: Prevents data corruption in shared libraries
- **Concurrency**: Excellent async/await support for concurrent operations
- **Error Handling**: Robust error handling with Result types
- **Crate Ecosystem**: Rich ecosystem for HTTP, database, serialization
- **Type Safety**: Compile-time guarantees for API contracts
- **Cross-Platform**: Works on all major platforms

### Key Rust Crates
- `reqwest` - HTTP client with async support
- `tokio` - Async runtime and utilities
- `serde` + `serde_json` - Serialization framework
- `diesel` + `diesel-async` - Type-safe ORM with async support
- `chrono` - Date/time handling
- `tracing` - Structured logging
- `anyhow` + `thiserror` - Error handling
- `clap` - Command-line parsing
- `config` - Configuration management
- `uuid` - UUID generation
- `base64` - Base64 encoding/decoding

## Architecture

### Crate Structure

```
common-library/
├── Cargo.toml                  # Dependencies and metadata
├── src/
│   ├── lib.rs                  # Public API exports
│   ├── http/                   # HTTP client functionality
│   │   ├── mod.rs
│   │   ├── client.rs           # HTTP client wrapper
│   │   ├── rate_limiter.rs     # Rate limiting logic
│   │   ├── retry.rs            # Retry logic
│   │   └── auth.rs             # Authentication
│   ├── storage/                # Data storage
│   │   ├── mod.rs
│   │   ├── database.rs         # Database operations
│   │   ├── files.rs            # File I/O operations
│   │   ├── serialization.rs    # Data serialization
│   │   └── backup.rs           # Backup operations
│   ├── config/                 # Configuration management
│   │   ├── mod.rs
│   │   ├── manager.rs          # Configuration manager
│   │   ├── validation.rs       # Config validation
│   │   └── sources.rs          # Config sources
│   ├── logging/                # Logging functionality
│   │   ├── mod.rs
│   │   ├── logger.rs           # Logger implementation
│   │   ├── formatters.rs       # Log formatters
│   │   └── metrics.rs          # Performance metrics
│   ├── metrics/                # Metrics and calculations
│   │   ├── mod.rs
│   │   ├── calculator.rs       # Statistical calculations
│   │   ├── scoring.rs          # Scoring algorithms
│   │   └── trends.rs           # Trend analysis
│   ├── validation/             # Data validation
│   │   ├── mod.rs
│   │   ├── schemas.rs          # Schema validation
│   │   ├── rules.rs            # Business rules
│   │   └── errors.rs           # Validation errors
│   ├── processing/             # Data processing
│   │   ├── mod.rs
│   │   ├── transformer.rs      # Data transformation
│   │   ├── cleaner.rs          # Data cleaning
│   │   └── aggregator.rs       # Data aggregation
│   └── utils/                  # Utility functions
│       ├── mod.rs
│       ├── date.rs             # Date/time utilities
│       ├── crypto.rs           # Cryptographic utilities
│       └── compression.rs      # Compression utilities
├── tests/                      # Integration tests
├── examples/                   # Usage examples
└── README.md
```

## HTTP Client Library

### Core HTTP Client
**Purpose**: Provides robust HTTP client functionality with rate limiting, retry logic, and authentication

**Key Components**:
- **Request Management**: Async HTTP requests with configurable timeouts
- **Rate Limiting**: Built-in rate limiting to respect API quotas
- **Retry Logic**: Automatic retry with exponential backoff for failed requests
- **Authentication**: Token-based authentication support
- **Error Handling**: Comprehensive error handling and recovery

**API Surface**:
- `APIClient::new()` - Initialize client with configuration
- `APIClient::get()` - Make GET requests with rate limiting
- `APIClient::post()` - Make POST requests with authentication
- `APIClient::set_auth()` - Configure authentication tokens
- `APIClient::set_rate_limit()` - Adjust rate limiting parameters

### Rate Limiting
**Purpose**: Manages API rate limits with semaphore-based concurrency control and automatic retry logic

**Key Components**:
- **Semaphore Control**: Limit concurrent requests to prevent overwhelming APIs
- **Rate Limit Tracking**: Monitor remaining requests and reset times
- **Automatic Retry**: Exponential backoff for rate limit exceeded errors
- **Request Queuing**: Queue requests during rate limit periods

**API Surface**:
- `RateLimiter::execute_with_retry()` - Execute requests with rate limiting and retry
- `RateLimiter::update_limits()` - Update rate limit information from responses
- `RateLimiter::remaining()` - Check remaining request quota
- `RateLimiter::wait_for_reset()` - Wait until rate limit resets
        
        let mut attempt = 0;
        loop {
            match self.check_rate_limit().await {
                Ok(()) => {
                    match request.try_clone().unwrap().send().await {
                        Ok(response) => {
                            self.update_rate_limit(&response).await;
                            return Ok(response);
                        }
                        Err(e) if attempt < retry_config.max_retries => {
                            attempt += 1;
                            let backoff = retry_config.calculate_backoff(attempt);
                            tokio::time::sleep(backoff).await;
                            continue;
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                Err(RateLimitError::Exceeded(reset_time)) => {
                    let wait_time = reset_time - Utc::now();
                    tokio::time::sleep(wait_time.to_std()?).await;
                    continue;
                }
            }
        }
## Storage Library

### Database Operations
**Purpose**: Provides async database operations with type safety and connection pooling

**Key Components**:
- **Connection Management**: Async SQLite connection establishment and pooling
- **Type-Safe Queries**: Compile-time checked SQL operations using diesel
- **Transaction Support**: ACID-compliant database transactions
- **Migration Management**: Database schema versioning and updates
- **Connection Pooling**: Efficient connection reuse for high-throughput operations

**API Surface**:
- `Database::new()` - Establish database connection
- `Database::insert()` - Type-safe record insertion
- `Database::select()` - Type-safe query execution
- `Database::update()` - Type-safe record updates
- `Database::delete()` - Type-safe record deletion
- `Database::transaction()` - Transaction management
- `Database::migrate()` - Schema migration execution

### File Operations
**Purpose**: Provides async file system operations for JSON data persistence and backup

**Key Components**:
- **JSON Serialization**: Automatic serialization/deserialization of structured data
- **Directory Management**: Automatic directory creation and path resolution
- **Async I/O**: Non-blocking file operations using tokio
- **Error Handling**: Comprehensive error handling for file system operations
- **Path Safety**: Secure path handling and validation

**API Surface**:
- `FileManager::new()` - Initialize with base directory
- `FileManager::save_json()` - Save structured data as JSON
- `FileManager::load_json()` - Load and deserialize JSON data
- `FileManager::exists()` - Check file existence
- `FileManager::delete()` - Remove files safely
- `FileManager::list_files()` - Directory listing with filtering

## Configuration Library

### Configuration Manager
**Purpose**: Centralized configuration management with environment variable support and validation

**Key Components**:
- **Multi-Source Loading**: File-based configs with environment variable overrides
- **Type Safety**: Strongly-typed configuration structures
- **Validation**: Runtime configuration validation and error reporting
- **Environment Support**: Development, staging, production environment handling
- **Hot Reloading**: Optional configuration reloading without restart

**API Surface**:
- `ConfigManager::new()` - Load configuration from multiple sources
- `ConfigManager::get()` - Retrieve typed configuration values
- `ConfigManager::validate()` - Validate configuration completeness
- `ConfigManager::reload()` - Reload configuration from sources
- `ConfigManager::export()` - Export current configuration
            
        let app_config: AppConfig = config.try_deserialize()?;
        Ok(Self { config: app_config })
    }
    
    pub fn get<T>(&self, key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Implementation for getting nested config values
        todo!()
    }
    
    pub fn validate(&self) -> Result<()> {
        // Validate configuration values
        if self.config.database.max_connections == 0 {
            return Err(ConfigError::InvalidValue("max_connections must be > 0".to_string()));
        }
        
        if self.config.http.timeout_seconds == 0 {
            return Err(ConfigError::InvalidValue("timeout_seconds must be > 0".to_string()));
        }
        
## Logging Library

### Structured Logger
**Purpose**: Provides structured logging with configurable levels and output formats

**Key Components**:
- **Log Levels**: Configurable logging levels (DEBUG, INFO, WARN, ERROR)
- **Structured Output**: JSON-formatted logs with consistent fields
- **Environment Integration**: Environment variable configuration support
- **Performance**: Minimal overhead logging for high-throughput applications

**API Surface**:
- `Logger::new()` - Initialize logger with specified level
- `Logger::init()` - Initialize global logging configuration
- `Logger::info()` - Log informational messages
- `Logger::warn()` - Log warning messages
- `Logger::error()` - Log error messages
- `Logger::debug()` - Log debug messages

## Metrics Library

### Statistical Calculations
**Purpose**: Provides statistical analysis and metrics calculation for repository data

**Key Components**:
- **Growth Rate Calculation**: Compound annual growth rate calculations
- **Trend Analysis**: Statistical trend detection and classification
- **Performance Metrics**: Activity scores and community health indicators
- **Time Series Analysis**: Temporal data analysis and forecasting

**API Surface**:
- `MetricsCalculator::calculate_growth_rate()` - Calculate compound growth rates
- `MetricsCalculator::calculate_trend()` - Analyze data trends over time
- `MetricsCalculator::activity_score()` - Calculate repository activity metrics
- `MetricsCalculator::community_health()` - Assess community engagement
- `MetricsCalculator::performance_indicators()` - Generate performance metrics
    
## Validation Library

### Schema Validation
**Purpose**: Provides JSON schema validation for data integrity and type safety

**Key Components**:
- **Schema Registry**: Centralized schema management and versioning
- **Type Validation**: Runtime type checking and constraint validation
- **Error Reporting**: Detailed validation error messages and suggestions
- **Performance**: Fast validation with minimal overhead

**API Surface**:
- `SchemaValidator::validate_schema()` - Validate data against named schema
- `SchemaValidator::register_schema()` - Register new validation schemas
- `SchemaValidator::validate_batch()` - Validate multiple records efficiently
- `SchemaValidator::get_errors()` - Retrieve detailed validation errors

## Testing Strategy

### Unit Tests
**Purpose**: Comprehensive testing of all library components with high coverage

**Key Components**:
- **Component Testing**: Individual component functionality testing
- **Integration Testing**: Cross-component interaction testing
- **Performance Testing**: Benchmarking and performance regression testing
- **Error Testing**: Error condition and edge case testing

**Test Categories**:
- **HTTP Client Tests**: Request/response handling, rate limiting, retry logic
- **Database Tests**: CRUD operations, transaction handling, connection pooling
- **Metrics Tests**: Statistical calculations, trend analysis, performance indicators
- **Validation Tests**: Schema validation, data integrity, error handling
- **Configuration Tests**: Config loading, validation, environment variable handling

### Integration Tests
**Purpose**: End-to-end testing of component interactions and real-world scenarios

**Key Components**:
- **HTTP Integration**: Full HTTP client testing with real API endpoints
- **Database Integration**: Complete database operation testing with real data
- **End-to-End Workflows**: Full application workflow testing
- **Performance Integration**: Load testing and performance validation

**Test Scenarios**:
- **API Collection Workflows**: Complete data collection and storage workflows
- **Error Recovery**: Network failures, rate limiting, and retry scenarios
- **Data Consistency**: Cross-component data integrity validation
- **Performance Benchmarks**: Throughput and latency testing
## Performance Considerations

### Memory Management
- **Zero-copy deserialization** where possible
- **Streaming JSON parsing** for large datasets
- **Connection pooling** for database operations
- **Lazy loading** for configuration values

### Concurrency
- **Async/await** throughout the API
- **Lock-free data structures** where possible
- **Efficient synchronization** primitives
- **Backpressure handling** for rate limiting

### Caching
- **Response caching** for HTTP clients
- **Configuration caching** for performance
- **Schema caching** for validation
- **Metrics caching** for calculations

This design provides a comprehensive, performant, and maintainable common library that serves as the foundation for all Phase 1 project selection tools while maintaining Rust's safety and performance guarantees.
