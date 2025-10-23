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
```rust
use reqwest::{Client, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct APIClient {
    client: Client,
    base_url: String,
    rate_limiter: RateLimiter,
    retry_config: RetryConfig,
    auth: Option<AuthToken>,
}

impl APIClient {
    pub fn new(base_url: String, config: ClientConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap();
            
        Self {
            client,
            base_url,
            rate_limiter: RateLimiter::new(config.rate_limit),
            retry_config: config.retry_config,
            auth: config.auth_token,
        }
    }
    
    pub async fn get(&self, endpoint: &str) -> Result<Response> {
        self.make_request(Method::GET, endpoint, None).await
    }
    
    pub async fn post(&self, endpoint: &str, data: &impl Serialize) -> Result<Response> {
        self.make_request(Method::POST, endpoint, Some(data)).await
    }
    
    async fn make_request(
        &self,
        method: Method,
        endpoint: &str,
        data: Option<&impl Serialize>,
    ) -> Result<Response> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
        let mut request = self.client.request(method, &url);
        
        if let Some(auth) = &self.auth {
            request = request.bearer_auth(auth.token());
        }
        
        if let Some(data) = data {
            request = request.json(data);
        }
        
        self.rate_limiter.execute_with_retry(request, &self.retry_config).await
    }
}
```

### Rate Limiting
```rust
pub struct RateLimiter {
    remaining: AtomicU32,
    reset_time: AtomicI64,
    semaphore: Semaphore,
}

impl RateLimiter {
    pub async fn execute_with_retry(
        &self,
        request: RequestBuilder,
        retry_config: &RetryConfig,
    ) -> Result<Response> {
        let permit = self.semaphore.acquire().await?;
        let _permit_guard = permit;
        
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
    }
}
```

## Storage Library

### Database Operations
```rust
use diesel_async::{AsyncSqliteConnection, AsyncConnection};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Database {
    connection: AsyncSqliteConnection,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let connection = AsyncSqliteConnection::establish(database_url).await?;
        Ok(Self { connection })
    }
    
    pub async fn insert<T>(&mut self, data: &T) -> Result<()>
    where
        T: Serialize + diesel::Insertable<dyn diesel::Table>,
    {
        diesel::insert_into(table)
            .values(data)
            .execute(&mut self.connection)
            .await?;
        Ok(())
    }
    
    pub async fn select<T>(&mut self, query: diesel::dsl::SelectStatement) -> Result<Vec<T>>
    where
        T: diesel::Queryable<dyn diesel::Table, diesel::sqlite::Sqlite> + Send,
    {
        let results = query.load(&mut self.connection).await?;
        Ok(results)
    }
}
```

### File Operations
```rust
use std::path::Path;
use tokio::fs;

pub struct FileManager {
    base_path: PathBuf,
}

impl FileManager {
    pub async fn save_json<T>(&self, data: &T, path: &Path) -> Result<()>
    where
        T: Serialize,
    {
        let json = serde_json::to_string_pretty(data)?;
        let full_path = self.base_path.join(path);
        
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::write(full_path, json).await?;
        Ok(())
    }
    
    pub async fn load_json<T>(&self, path: &Path) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let full_path = self.base_path.join(path);
        let content = fs::read_to_string(full_path).await?;
        let data = serde_json::from_str(&content)?;
        Ok(data)
    }
    
    pub async fn backup(&self, source: &Path, destination: &Path) -> Result<()> {
        let source_path = self.base_path.join(source);
        let dest_path = self.base_path.join(destination);
        
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::copy(source_path, dest_path).await?;
        Ok(())
    }
}
```

## Configuration Library

### Configuration Manager
```rust
use config::{Config, ConfigError, File, Environment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub http: HttpConfig,
    pub logging: LoggingConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub rate_limit: u32,
}

pub struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;
            
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
        
        Ok(())
    }
}
```

## Logging Library

### Structured Logger
```rust
use tracing::{info, warn, error, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Logger {
    level: Level,
}

impl Logger {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
    
    pub fn init(&self) -> Result<()> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| format!("{}", self.level).into()),
            )
            .with(tracing_subscriber::fmt::layer().json())
            .init();
        Ok(())
    }
    
    pub fn log_performance(&self, operation: &str, duration: Duration) {
        info!(
            operation = operation,
            duration_ms = duration.as_millis(),
            "Performance metric"
        );
    }
    
    pub fn log_api_call(&self, endpoint: &str, status: u16, duration: Duration) {
        info!(
            endpoint = endpoint,
            status = status,
            duration_ms = duration.as_millis(),
            "API call completed"
        );
    }
}
```

## Metrics Library

### Statistical Calculations
```rust
pub struct MetricsCalculator;

impl MetricsCalculator {
    pub fn calculate_growth_rate(&self, values: &[f64], time_periods: &[DateTime<Utc>]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let first_value = values[0];
        let last_value = values[values.len() - 1];
        let time_span = time_periods[time_periods.len() - 1] - time_periods[0];
        let years = time_span.num_days() as f64 / 365.25;
        
        if years > 0.0 && first_value > 0.0 {
            (last_value / first_value).powf(1.0 / years) - 1.0
        } else {
            0.0
        }
    }
    
    pub fn calculate_trend(&self, values: &[f64], window_size: usize) -> Trend {
        if values.len() < window_size {
            return Trend::InsufficientData;
        }
        
        let recent_values = &values[values.len() - window_size..];
        let slope = self.calculate_slope(recent_values);
        
        match slope {
            s if s > 0.1 => Trend::Increasing,
            s if s < -0.1 => Trend::Decreasing,
            _ => Trend::Stable,
        }
    }
    
    pub fn normalize_scores(&self, scores: &[f64], method: NormalizationMethod) -> Vec<f64> {
        match method {
            NormalizationMethod::MinMax => {
                let min = scores.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = scores.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let range = max - min;
                
                if range == 0.0 {
                    scores.iter().map(|_| 0.5).collect()
                } else {
                    scores.iter().map(|&score| (score - min) / range).collect()
                }
            }
            NormalizationMethod::ZScore => {
                let mean = scores.iter().sum::<f64>() / scores.len() as f64;
                let variance = scores.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / scores.len() as f64;
                let std_dev = variance.sqrt();
                
                if std_dev == 0.0 {
                    scores.iter().map(|_| 0.0).collect()
                } else {
                    scores.iter().map(|&score| (score - mean) / std_dev).collect()
                }
            }
        }
    }
}
```

## Validation Library

### Schema Validation
```rust
use serde_json::Value;
use std::collections::HashMap;

pub struct SchemaValidator {
    schemas: HashMap<String, JsonSchema>,
}

impl SchemaValidator {
    pub fn validate_schema(&self, data: &Value, schema_name: &str) -> Result<()> {
        let schema = self.schemas.get(schema_name)
            .ok_or_else(|| ValidationError::SchemaNotFound(schema_name.to_string()))?;
            
        self.validate_value(data, schema)
    }
    
    fn validate_value(&self, value: &Value, schema: &JsonSchema) -> Result<()> {
        match schema {
            JsonSchema::Object { properties, required } => {
                if let Value::Object(obj) = value {
                    // Check required fields
                    for field in required {
                        if !obj.contains_key(field) {
                            return Err(ValidationError::MissingRequiredField(field.clone()));
                        }
                    }
                    
                    // Validate each property
                    for (key, value) in obj {
                        if let Some(prop_schema) = properties.get(key) {
                            self.validate_value(value, prop_schema)?;
                        }
                    }
                } else {
                    return Err(ValidationError::TypeMismatch("object".to_string()));
                }
            }
            JsonSchema::String { min_length, max_length } => {
                if let Value::String(s) = value {
                    if let Some(min) = min_length {
                        if s.len() < *min {
                            return Err(ValidationError::StringTooShort(*min));
                        }
                    }
                    if let Some(max) = max_length {
                        if s.len() > *max {
                            return Err(ValidationError::StringTooLong(*max));
                        }
                    }
                } else {
                    return Err(ValidationError::TypeMismatch("string".to_string()));
                }
            }
            JsonSchema::Number { min, max } => {
                if let Value::Number(n) = value {
                    if let Some(min_val) = min {
                        if n.as_f64().unwrap() < *min_val {
                            return Err(ValidationError::NumberTooSmall(*min_val));
                        }
                    }
                    if let Some(max_val) = max {
                        if n.as_f64().unwrap() > *max_val {
                            return Err(ValidationError::NumberTooLarge(*max_val));
                        }
                    }
                } else {
                    return Err(ValidationError::TypeMismatch("number".to_string()));
                }
            }
            _ => {} // Handle other schema types
        }
        Ok(())
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_calculation() {
        let calculator = MetricsCalculator;
        let values = vec![100.0, 110.0, 121.0, 133.1];
        let growth_rate = calculator.calculate_growth_rate(&values, &[]);
        assert!((growth_rate - 0.1).abs() < 0.01);
    }
    
    #[test]
    fn test_config_validation() {
        let config = AppConfig {
            database: DatabaseConfig {
                url: "sqlite:test.db".to_string(),
                max_connections: 10,
                timeout_seconds: 30,
            },
            // ... other fields
        };
        
        let manager = ConfigManager::new();
        assert!(manager.validate().is_ok());
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_http_client_with_rate_limiting() {
        let client = APIClient::new("https://api.github.com".to_string(), config);
        let response = client.get("repos/octocat/Hello-World").await.unwrap();
        assert!(response.status().is_success());
    }
    
    #[tokio::test]
    async fn test_database_operations() {
        let db = Database::in_memory().unwrap();
        let data = serde_json::json!({"name": "test", "value": 42});
        
        db.insert("test_table", &data).await.unwrap();
        let retrieved: Value = db.select("SELECT data FROM test_table").await.unwrap();
        assert_eq!(retrieved["name"], "test");
    }
}
```

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
