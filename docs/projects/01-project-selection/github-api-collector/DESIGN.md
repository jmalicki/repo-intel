# GitHub API Collector - Detailed Design

**Parent:** [GitHub API Collector](README.md)
**Related:** [Common Library](../common-library/DESIGN.md) - Shared functionality dependencies

## Overview

The GitHub API Collector is a **Rust application** that systematically collects repository data from GitHub's API for 160-240 candidate projects across 8 categories. It uses the Common Library for HTTP client functionality, data processing, storage, and configuration management.

## Programming Language: Rust

### Justification for Rust
- **Performance**: Excellent for high-throughput API collection with minimal resource usage
- **Memory Safety**: Prevents data corruption during large-scale data collection
- **Concurrency**: Native async/await for efficient parallel API requests
- **Error Handling**: Robust error handling with Result types
- **HTTP Libraries**: Excellent crates like `reqwest`, `tokio`, `serde`
- **Database Integration**: Strong SQLite support with `rusqlite` and `sqlx`
- **JSON Processing**: Fast JSON parsing with `serde_json`

### Key Rust Crates
- `reqwest` - HTTP client with async support
- `tokio` - Async runtime for concurrent operations
- `serde` + `serde_json` - JSON serialization/deserialization
- `rusqlite` - SQLite database operations
- `chrono` - Date/time handling
- `clap` - Command-line argument parsing
- `tracing` - Structured logging
- `anyhow` + `thiserror` - Error handling

## Architecture

### Core Components

```
GitHubAPICollector
├── src/
│   ├── main.rs                 # Application entry point
│   ├── config/                 # Configuration management
│   │   ├── mod.rs
│   │   ├── settings.rs         # Configuration structs
│   │   └── validation.rs       # Config validation
│   ├── api/                    # GitHub API client
│   │   ├── mod.rs
│   │   ├── client.rs           # HTTP client wrapper
│   │   ├── endpoints.rs        # API endpoint definitions
│   │   └── rate_limiter.rs     # Rate limiting logic
│   ├── collectors/             # Data collection logic
│   │   ├── mod.rs
│   │   ├── repository.rs       # Repository data collection
│   │   ├── search.rs           # Search operations
│   │   └── metrics.rs          # Metrics calculation
│   ├── storage/                # Data storage
│   │   ├── mod.rs
│   │   ├── database.rs         # SQLite operations
│   │   ├── files.rs            # File I/O operations
│   │   └── backup.rs           # Backup operations
│   ├── models/                 # Data models
│   │   ├── mod.rs
│   │   ├── repository.rs       # Repository data structures
│   │   ├── search.rs           # Search result structures
│   │   └── metrics.rs          # Metrics data structures
│   └── utils/                  # Utility functions
│       ├── mod.rs
│       ├── date.rs             # Date/time utilities
│       └── validation.rs       # Data validation
├── tests/                      # Test modules
├── Cargo.toml                  # Dependencies
└── README.md
```

## Data Storage Strategy

### Primary Storage: SQLite Database
**Rationale**: Structured queries, ACID compliance, embedded (no external dependencies)

#### Database Schema

```sql
-- Categories table
CREATE TABLE categories (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    search_queries TEXT NOT NULL,  -- JSON array of search queries
    min_stars INTEGER NOT NULL,
    max_results INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Repositories table
CREATE TABLE repositories (
    id INTEGER PRIMARY KEY,
    github_id INTEGER UNIQUE NOT NULL,
    name TEXT NOT NULL,
    full_name TEXT NOT NULL,
    owner TEXT NOT NULL,
    description TEXT,
    language TEXT,
    topics TEXT,  -- JSON array
    license TEXT,
    stars INTEGER NOT NULL,
    forks INTEGER NOT NULL,
    watchers INTEGER NOT NULL,
    open_issues INTEGER NOT NULL,
    open_prs INTEGER NOT NULL,
    size INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    pushed_at DATETIME NOT NULL,
    category_id INTEGER NOT NULL,
    collected_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- Repository metrics table
CREATE TABLE repository_metrics (
    id INTEGER PRIMARY KEY,
    repository_id INTEGER NOT NULL,
    commit_count INTEGER,
    contributor_count INTEGER,
    release_count INTEGER,
    avg_issue_response_time REAL,
    avg_pr_merge_time REAL,
    activity_score REAL,
    community_health_score REAL,
    calculated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (repository_id) REFERENCES repositories(id)
);

-- Historical data table
CREATE TABLE historical_data (
    id INTEGER PRIMARY KEY,
    repository_id INTEGER NOT NULL,
    date DATE NOT NULL,
    stars INTEGER NOT NULL,
    forks INTEGER NOT NULL,
    watchers INTEGER NOT NULL,
    commits INTEGER NOT NULL,
    FOREIGN KEY (repository_id) REFERENCES repositories(id),
    UNIQUE(repository_id, date)
);

-- API rate limit tracking
CREATE TABLE rate_limits (
    id INTEGER PRIMARY KEY,
    endpoint TEXT NOT NULL,
    remaining INTEGER NOT NULL,
    reset_time DATETIME NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Secondary Storage: JSON Files
**Rationale**: Human-readable backups, easy data exchange, version control friendly

#### File Structure
```
data/
├── raw/                        # Raw API responses
│   ├── repositories/           # Individual repository data
│   │   ├── rust-libraries/
│   │   │   ├── serde.json
│   │   │   └── tokio.json
│   │   └── chrome-extensions/
│   └── search-results/         # Search API responses
│       ├── rust-libraries.json
│       └── chrome-extensions.json
├── processed/                  # Processed data
│   ├── repositories.json       # All repositories
│   ├── metrics.json           # Calculated metrics
│   └── summaries/             # Category summaries
│       ├── rust-libraries.json
│       └── chrome-extensions.json
└── backups/                   # Automated backups
    ├── 2024-01-15/
    └── 2024-01-16/
```

## Core Data Models

### Repository Model
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub license: Option<String>,
    pub stars: u32,
    pub forks: u32,
    pub watchers: u32,
    pub open_issues: u32,
    pub open_prs: u32,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: DateTime<Utc>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMetrics {
    pub repository_id: u64,
    pub commit_count: u32,
    pub contributor_count: u32,
    pub release_count: u32,
    pub avg_issue_response_time: Option<f64>,
    pub avg_pr_merge_time: Option<f64>,
    pub activity_score: f64,
    pub community_health_score: f64,
    pub calculated_at: DateTime<Utc>,
}
```

### Search Configuration Model
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub category: String,
    pub queries: Vec<String>,
    pub min_stars: u32,
    pub max_results: u32,
    pub sort_by: SortBy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    Stars,
    Activity,
    Relevance,
}
```

## API Client Design

### Rate Limiting Strategy
```rust
pub struct RateLimiter {
    remaining: u32,
    reset_time: DateTime<Utc>,
    client: reqwest::Client,
}

impl RateLimiter {
    pub async fn make_request(&mut self, request: RequestBuilder) -> Result<Response> {
        // Check rate limit before making request
        if self.remaining == 0 {
            self.wait_for_reset().await?;
        }
        
        let response = request.send().await?;
        self.update_rate_limit(&response);
        Ok(response)
    }
    
    async fn wait_for_reset(&self) -> Result<()> {
        let wait_time = self.reset_time - Utc::now();
        tokio::time::sleep(wait_time.to_std()?).await;
        Ok(())
    }
}
```

### Concurrent Collection
```rust
pub struct ConcurrentCollector {
    rate_limiter: RateLimiter,
    semaphore: Semaphore,
    database: Database,
}

impl ConcurrentCollector {
    pub async fn collect_repositories(&self, configs: Vec<SearchConfig>) -> Result<()> {
        let mut tasks = Vec::new();
        
        for config in configs {
            let semaphore = self.semaphore.clone();
            let rate_limiter = self.rate_limiter.clone();
            let database = self.database.clone();
            
            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await?;
                Self::collect_category(rate_limiter, database, config).await
            });
            
            tasks.push(task);
        }
        
        // Wait for all tasks to complete
        for task in tasks {
            task.await??;
        }
        
        Ok(())
    }
}
```

## Configuration Management

### Configuration Structure
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github: GitHubConfig,
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub collection: CollectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub api_url: String,
    pub token: String,
    pub rate_limit: u32,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
    pub backup_enabled: bool,
    pub backup_interval_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub categories: Vec<SearchConfig>,
    pub concurrent_requests: u32,
    pub batch_size: u32,
    pub historical_days: u32,
}
```

### Configuration File (YAML)
```yaml
github:
  api_url: "https://api.github.com"
  token: "${GITHUB_TOKEN}"
  rate_limit: 5000
  timeout_seconds: 30
  retry_attempts: 3

database:
  path: "data/github_collector.db"
  backup_enabled: true
  backup_interval_hours: 24

storage:
  raw_data_path: "data/raw"
  processed_data_path: "data/processed"
  backup_path: "data/backups"

collection:
  concurrent_requests: 5
  batch_size: 10
  historical_days: 30
  categories:
    - name: "rust-libraries"
      queries:
        - "language:rust stars:>1000"
        - "topic:rust-library stars:>500"
      min_stars: 1000
      max_results: 30
```

## Error Handling Strategy

### Error Types
```rust
#[derive(thiserror::Error, Debug)]
pub enum CollectorError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Rate limit exceeded: reset at {0}")]
    RateLimitExceeded(DateTime<Utc>),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Data validation error: {0}")]
    ValidationError(String),
}
```

### Retry Logic
```rust
pub async fn with_retry<F, T>(operation: F, max_retries: u32) -> Result<T>
where
    F: Fn() -> BoxFuture<'static, Result<T>>,
{
    let mut attempt = 0;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(CollectorError::RateLimitExceeded(reset_time)) => {
                let wait_time = reset_time - Utc::now();
                tokio::time::sleep(wait_time.to_std()?).await;
                continue;
            }
            Err(e) if attempt < max_retries => {
                attempt += 1;
                let backoff = Duration::from_secs(2_u64.pow(attempt));
                tokio::time::sleep(backoff).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(5000, Utc::now());
        // Test rate limit handling
    }
    
    #[tokio::test]
    async fn test_repository_parsing() {
        let json = r#"{"id": 1, "name": "test"}"#;
        let repo: Repository = serde_json::from_str(json).unwrap();
        assert_eq!(repo.name, "test");
    }
    
    #[test]
    fn test_config_validation() {
        let config = Config::from_file("test_config.yaml").unwrap();
        assert!(config.github.rate_limit > 0);
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_collection_workflow() {
        // Test complete collection workflow
        let config = Config::test_config();
        let collector = GitHubAPICollector::new(config).await.unwrap();
        
        let results = collector.collect_all().await.unwrap();
        assert!(!results.is_empty());
    }
    
    #[tokio::test]
    async fn test_database_operations() {
        // Test database operations
        let db = Database::in_memory().unwrap();
        let repo = create_test_repository();
        
        db.insert_repository(&repo).await.unwrap();
        let retrieved = db.get_repository(repo.id).await.unwrap();
        assert_eq!(repo.name, retrieved.name);
    }
}
```

### Performance Tests
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_concurrent_collection() {
        let start = Instant::now();
        let collector = GitHubAPICollector::new(config).await.unwrap();
        
        let results = collector.collect_all().await.unwrap();
        let duration = start.elapsed();
        
        // Should collect 240 repositories in under 10 minutes
        assert!(duration.as_secs() < 600);
        assert!(results.len() >= 240);
    }
}
```

## Performance Considerations

### Memory Management
- **Streaming JSON parsing** for large responses
- **Batch database operations** to reduce memory usage
- **Lazy loading** of historical data
- **Connection pooling** for database operations

### Concurrency
- **Semaphore-based limiting** to control concurrent requests
- **Async/await** for non-blocking I/O operations
- **Connection pooling** for HTTP requests
- **Parallel processing** of different categories

### Data Persistence
- **Incremental updates** to avoid re-collecting existing data
- **Compression** for historical data storage
- **Backup strategies** for data recovery
- **Data validation** before storage

## Integration with Common Library

### HTTP Client Integration
```rust
use common_library::http::APIClient;
use common_library::storage::StorageManager;

pub struct GitHubAPICollector {
    api_client: APIClient,
    storage: StorageManager,
    config: Config,
}
```

### Shared Functionality
- **Rate limiting** via Common Library HTTP client
- **Data validation** via Common Library validation
- **Storage operations** via Common Library storage
- **Logging** via Common Library logging
- **Configuration** via Common Library configuration

This design provides a robust, performant, and maintainable GitHub API collector that integrates seamlessly with the Common Library while meeting all requirements for collecting repository data across 8 categories.
