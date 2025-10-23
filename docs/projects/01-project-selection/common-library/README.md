# Common Library

**Parent:** [Projects Overview](../README.md)
**Related:** [Automation Opportunities](../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) - Shared functionality across all tools

## Overview

The Common Library is a **shared Python library** that provides reusable functionality for all Phase 1 project selection tools. It eliminates code duplication and ensures consistent behavior across all tools.

## Purpose

- **Eliminate code duplication** across all project selection tools
- **Provide shared functionality** for common operations
- **Ensure consistent behavior** across all tools
- **Simplify maintenance** and updates
- **Standardize interfaces** and data formats

## Design Principles

1. **Reusability** - Functions designed for multiple use cases
2. **Consistency** - Standardized interfaces and behavior
3. **Modularity** - Independent, composable components
4. **Maintainability** - Single source of truth for common functionality
5. **Performance** - Optimized for efficiency across all tools

## Shared Components

### 1. HTTP Client Library
**Used by:** GitHub API Collector, Package Manager Collector, Trending Analyzer

**Functionality:**
- **Rate Limiting** - Automatic rate limit compliance
- **Retry Logic** - Exponential backoff for failed requests
- **Authentication** - Token management and rotation
- **Error Handling** - Comprehensive error handling and logging
- **Pagination** - Automatic pagination handling
- **Caching** - Response caching to reduce API calls

**Key Functions:**
```python
class APIClient:
    def __init__(self, base_url, rate_limit, auth_token=None)
    def get(self, endpoint, params=None, headers=None)
    def post(self, endpoint, data=None, headers=None)
    def handle_rate_limit(self, response)
    def retry_request(self, func, max_retries=3)
    def paginate_results(self, endpoint, params=None)
```

### 2. Data Processing Library
**Used by:** All tools that process data

**Functionality:**
- **Data Validation** - Schema validation and data quality checks
- **Data Normalization** - Convert data to common formats
- **Data Transformation** - Transform data between formats
- **Data Cleaning** - Remove duplicates, handle missing values
- **Data Aggregation** - Merge and combine data from multiple sources

**Key Functions:**
```python
class DataProcessor:
    def validate_schema(self, data, schema)
    def normalize_data(self, data, mapping)
    def clean_data(self, data, rules)
    def merge_datasets(self, datasets, merge_keys)
    def deduplicate_data(self, data, key_fields)
```

### 3. Storage Library
**Used by:** All tools that store data

**Functionality:**
- **File I/O** - Read/write JSON, CSV, YAML files
- **Database Operations** - SQLite, PostgreSQL support
- **Data Serialization** - JSON, Pickle, MessagePack
- **Backup & Recovery** - Data backup and restoration
- **Data Compression** - Gzip, LZ4 compression support

**Key Functions:**
```python
class StorageManager:
    def save_json(self, data, filepath)
    def load_json(self, filepath)
    def save_csv(self, data, filepath, headers=None)
    def load_csv(self, filepath)
    def backup_data(self, source, destination)
    def restore_data(self, backup_path, destination)
```

### 4. Configuration Library
**Used by:** All tools that use configuration

**Functionality:**
- **Configuration Management** - YAML, JSON, INI file support
- **Environment Variables** - Environment variable handling
- **Default Values** - Sensible defaults for all settings
- **Validation** - Configuration validation and error reporting
- **Hot Reloading** - Configuration changes without restart

**Key Functions:**
```python
class ConfigManager:
    def __init__(self, config_file=None)
    def load_config(self, filepath)
    def get(self, key, default=None)
    def set(self, key, value)
    def validate_config(self, schema)
    def reload_config(self)
```

### 5. Logging Library
**Used by:** All tools

**Functionality:**
- **Structured Logging** - JSON-formatted logs
- **Log Levels** - DEBUG, INFO, WARNING, ERROR, CRITICAL
- **Log Rotation** - Automatic log file rotation
- **Performance Metrics** - Execution time and resource usage
- **Error Tracking** - Detailed error information and stack traces

**Key Functions:**
```python
class Logger:
    def __init__(self, name, level=INFO)
    def debug(self, message, **kwargs)
    def info(self, message, **kwargs)
    def warning(self, message, **kwargs)
    def error(self, message, **kwargs)
    def critical(self, message, **kwargs)
    def log_performance(self, operation, duration)
```

### 6. Metrics Library
**Used by:** Metrics Calculator, Quality Filter, Report Generator

**Functionality:**
- **Statistical Calculations** - Mean, median, standard deviation
- **Growth Rate Calculations** - Linear and exponential growth
- **Trend Analysis** - Moving averages, trend detection
- **Scoring Algorithms** - Weighted scoring, normalization
- **Benchmarking** - Performance comparison and ranking

**Key Functions:**
```python
class MetricsCalculator:
    def calculate_growth_rate(self, values, time_periods)
    def calculate_trend(self, values, window_size)
    def normalize_scores(self, scores, method='minmax')
    def weighted_score(self, values, weights)
    def rank_items(self, items, criteria)
```

### 7. Validation Library
**Used by:** All tools that validate data

**Functionality:**
- **Schema Validation** - JSON Schema validation
- **Data Type Validation** - Type checking and conversion
- **Range Validation** - Min/max value validation
- **Format Validation** - Email, URL, date format validation
- **Business Rule Validation** - Custom validation rules

**Key Functions:**
```python
class Validator:
    def validate_schema(self, data, schema)
    def validate_type(self, value, expected_type)
    def validate_range(self, value, min_val, max_val)
    def validate_format(self, value, pattern)
    def validate_business_rules(self, data, rules)
```

## Integration with Existing Tools

### GitHub API Collector
- **Uses:** HTTP Client, Storage, Logging, Configuration
- **Benefits:** Consistent rate limiting, error handling, data storage

### Package Manager Collector
- **Uses:** HTTP Client, Storage, Logging, Configuration
- **Benefits:** Unified API handling across NPM, PyPI, crates.io

### Trending Analyzer
- **Uses:** HTTP Client, Metrics, Storage, Logging
- **Benefits:** Consistent trending calculations, data storage

### Metrics Calculator
- **Uses:** Metrics, Validation, Storage, Logging
- **Benefits:** Standardized calculations, data validation

### Repository Data Aggregator
- **Uses:** Data Processing, Storage, Validation, Logging
- **Benefits:** Consistent data merging, validation, storage

### Quality Filter
- **Uses:** Validation, Metrics, Storage, Logging
- **Benefits:** Standardized filtering, scoring, validation

### Report Generator
- **Uses:** Storage, Metrics, Logging, Configuration
- **Benefits:** Consistent report generation, data formatting

### Pattern Matcher
- **Uses:** HTTP Client, Storage, Logging, Configuration
- **Benefits:** Consistent API handling, data storage

## Implementation Strategy

### Phase 1: Core Infrastructure
1. **HTTP Client Library** - Essential for API-based tools
2. **Storage Library** - Essential for data persistence
3. **Logging Library** - Essential for debugging and monitoring
4. **Configuration Library** - Essential for tool configuration

### Phase 2: Data Processing
1. **Data Processing Library** - For data transformation and validation
2. **Validation Library** - For data quality assurance
3. **Metrics Library** - For calculations and scoring

### Phase 3: Advanced Features
1. **Caching System** - For performance optimization
2. **Monitoring System** - For tool health monitoring
3. **Testing Framework** - For automated testing

## Benefits

### Code Reduction
- **Eliminate Duplication** - Remove ~60% of duplicate code
- **Consistent Interfaces** - Standardized APIs across all tools
- **Simplified Maintenance** - Single source of truth for common functionality

### Quality Improvement
- **Consistent Behavior** - Same error handling, logging, validation
- **Better Testing** - Shared test utilities and frameworks
- **Performance Optimization** - Optimized shared components

### Development Efficiency
- **Faster Development** - Reuse existing components
- **Easier Debugging** - Consistent logging and error handling
- **Simplified Deployment** - Shared dependencies and configuration

## Dependencies

### Core Dependencies
- **requests** - HTTP client library
- **pydantic** - Data validation and settings
- **click** - Command-line interface
- **rich** - Rich text and beautiful formatting

### Optional Dependencies
- **pandas** - Data manipulation and analysis
- **numpy** - Numerical computing
- **sqlalchemy** - Database ORM
- **redis** - Caching and session storage

## Usage Examples

### HTTP Client Usage
```python
from common_library import APIClient

client = APIClient(
    base_url="https://api.github.com",
    rate_limit=5000,
    auth_token="ghp_..."
)

repos = client.paginate_results("/search/repositories", {
    "q": "topic:rust-library",
    "sort": "stars"
})
```

### Data Processing Usage
```python
from common_library import DataProcessor

processor = DataProcessor()
clean_data = processor.clean_data(raw_data, {
    "remove_duplicates": True,
    "fill_missing": "mean"
})
```

### Metrics Calculation Usage
```python
from common_library import MetricsCalculator

calculator = MetricsCalculator()
growth_rate = calculator.calculate_growth_rate(
    star_counts,
    time_periods
)
```

This common library eliminates duplication while providing a solid foundation for all Phase 1 project selection tools.
