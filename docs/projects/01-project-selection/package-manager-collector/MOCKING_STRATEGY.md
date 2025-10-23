# Package Manager Collector - Mocking Strategy

**Parent:** [Package Manager Collector Design](DESIGN.md)
**Related:** [Implementation Plan](IMPLEMENTATION_PLAN.md)

## Overview

This document outlines a comprehensive mocking strategy for the Package Manager Collector, which integrates with 8 different package manager APIs (NPM, PyPI, Crates.io, Maven Central, Go Modules, RubyGems, Packagist, NuGet). Effective mocking is crucial for reliable testing, development speed, and avoiding API rate limits during testing.

## Testing Architecture

### Core Testing Components

```
TestingStrategy
├── mockito/                    # HTTP request mocking
│   ├── mockito-server          # Mock HTTP server
│   ├── mockito-client          # HTTP client mocking
│   └── mockito-verification    # Request verification
├── fixtures/                   # Static response data
│   ├── npm-responses.json      # NPM API response samples
│   ├── pypi-responses.json     # PyPI API response samples
│   ├── crates-responses.json   # Crates.io API response samples
│   ├── maven-responses.json    # Maven Central API response samples
│   ├── go-responses.json       # Go Modules API response samples
│   ├── ruby-responses.json     # RubyGems API response samples
│   ├── php-responses.json      # Packagist API response samples
│   └── nuget-responses.json    # NuGet API response samples
├── scenarios/                  # Test scenario definitions
│   ├── success-scenarios.json  # Successful API responses
│   ├── error-scenarios.json    # Error conditions and responses
│   ├── rate-limit-scenarios.json # Rate limiting scenarios
│   └── network-error-scenarios.json # Network failure scenarios
├── builders/                   # Mock response builders
│   ├── response-builder.rs     # Generic response builder
│   ├── error-builder.rs        # Error response builder
│   └── rate-limit-builder.rs   # Rate limit response builder
└── integration/                # Real API integration tests
    ├── manual-tests/           # Manual test suite
    ├── benchmarks/             # Performance benchmarks
    ├── smoke-tests/            # Smoke tests with real APIs
    └── validation-tests/       # Mock validation against real APIs
```

## Mocking Libraries and Tools

### Primary Mocking Library: `mockito`

**Usage**: Mock HTTP requests and responses for all package manager APIs

**Benefits**:
- **Rust-native**: Built specifically for Rust testing
- **HTTP-focused**: Perfect for API mocking
- **Easy setup**: Simple request/response mocking
- **Verification**: Can verify request patterns and counts
- **Isolation**: Each test gets a fresh mock server

**Implementation**:
```rust
use mockito::{mock, server_url, Mock};

// Create mock for NPM API
let npm_mock = mock("GET", "/express")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(include_str!("fixtures/npm-responses.json"))
    .create();

// Create mock for PyPI API
let pypi_mock = mock("GET", "/pypi/requests/json")
    .with_status(200)
    .with_header("content-type", "application/json")
    .with_body(include_str!("fixtures/pypi-responses.json"))
    .create();
```

### Secondary Mocking: `wiremock`

**Usage**: Advanced HTTP mocking with more sophisticated features

**Benefits**:
- **Advanced matching**: Complex request matching patterns
- **Stateful mocks**: Maintain state between requests
- **Custom responses**: Dynamic response generation
- **Request verification**: Detailed request analysis

**Use Cases**:
- Complex API workflows
- Stateful interactions
- Advanced error scenarios
- Performance testing with realistic delays

### Database Mocking: `diesel-mock`

**Usage**: Mock database operations without requiring a real database

**Benefits**:
- **Fast tests**: No database setup/teardown
- **Isolation**: Each test gets clean state
- **Predictable data**: Controlled test data
- **No dependencies**: No external database required

## Mock Data Strategy

### Static Fixture Files

**Location**: `tests/fixtures/`

**Purpose**: Store realistic API response data for each package manager

**Structure**:
```json
{
  "npm": {
    "success": {
      "express": { /* NPM package data */ },
      "lodash": { /* NPM package data */ }
    },
    "errors": {
      "not_found": { /* 404 response */ },
      "rate_limited": { /* 429 response */ }
    }
  },
  "pypi": {
    "success": {
      "requests": { /* PyPI package data */ },
      "django": { /* PyPI package data */ }
    },
    "errors": {
      "not_found": { /* 404 response */ },
      "rate_limited": { /* 429 response */ }
    }
  }
}
```

### Dynamic Response Generation

**Purpose**: Generate realistic responses for edge cases and varied scenarios

**Implementation**:
```rust
pub struct MockResponseBuilder {
    package_name: String,
    registry: PackageRegistry,
    scenario: TestScenario,
}

impl MockResponseBuilder {
    pub fn success_response(&self) -> MockResponse {
        // Generate successful API response
    }
    
    pub fn error_response(&self, error_type: ApiError) -> MockResponse {
        // Generate error response
    }
    
    pub fn rate_limit_response(&self) -> MockResponse {
        // Generate rate limit response
    }
}
```

## Testing Scenarios

### 1. Success Scenarios

**Purpose**: Test normal API interactions with successful responses

**Coverage**:
- Package metadata retrieval
- Download statistics
- Dependency information
- Version history
- Search results

**Mock Data**:
- Realistic package data for popular packages
- Varied package sizes and complexity
- Different package states (stable, beta, deprecated)

### 2. Error Scenarios

**Purpose**: Test error handling and recovery mechanisms

**Coverage**:
- **404 Not Found**: Package doesn't exist
- **403 Forbidden**: Access denied
- **500 Internal Server Error**: Server issues
- **Timeout**: Network timeouts
- **Invalid JSON**: Malformed responses
- **Authentication errors**: Invalid API keys

**Mock Implementation**:
```rust
// Test 404 error handling
let not_found_mock = mock("GET", "/nonexistent-package")
    .with_status(404)
    .with_body(r#"{"error": "package not found"}"#)
    .create();

// Test timeout scenario
let timeout_mock = mock("GET", "/slow-package")
    .with_status(200)
    .with_delay(Duration::from_secs(30)) // Simulate timeout
    .create();
```

### 3. Rate Limiting Scenarios

**Purpose**: Test rate limit handling and retry logic

**Coverage**:
- **429 Too Many Requests**: Rate limit exceeded
- **Retry logic**: Exponential backoff
- **Rate limit headers**: Respect API rate limits
- **Circuit breaker**: Stop requests when rate limited

**Mock Implementation**:
```rust
// Test rate limiting
let rate_limit_mock = mock("GET", "/rate-limited")
    .with_status(429)
    .with_header("X-RateLimit-Remaining", "0")
    .with_header("X-RateLimit-Reset", "1640995200")
    .with_body(r#"{"error": "rate limit exceeded"}"#)
    .create();
```

### 4. Network Error Scenarios

**Purpose**: Test network failure handling

**Coverage**:
- **Connection refused**: Server unavailable
- **DNS resolution failure**: Invalid endpoints
- **SSL/TLS errors**: Certificate issues
- **Partial responses**: Incomplete data

### 5. Data Consistency Scenarios

**Purpose**: Test data validation and consistency

**Coverage**:
- **Schema validation**: Invalid JSON structure
- **Field validation**: Missing required fields
- **Type validation**: Wrong data types
- **Range validation**: Invalid numeric ranges

## Mock Configuration Management

### Environment-Based Mocking

**Purpose**: Different mocking strategies for different environments

**Environments**:
- **Unit Tests**: Full mocking with static fixtures
- **Integration Tests**: Partial mocking with real APIs
- **Performance Tests**: Mocked APIs with realistic delays
- **Development**: Optional mocking for faster development

**Configuration**:
```rust
#[derive(Debug, Clone)]
pub enum MockMode {
    Full,      // Mock all external APIs
    Partial,   // Mock some APIs, use real for others
    None,      // Use real APIs (for integration tests)
}

pub struct MockConfig {
    pub mode: MockMode,
    pub npm_mock: bool,
    pub pypi_mock: bool,
    pub crates_mock: bool,
    // ... other registries
}
```

### Mock Server Management

**Purpose**: Centralized mock server lifecycle management

**Implementation**:
```rust
pub struct MockServerManager {
    servers: HashMap<String, MockServer>,
    config: MockConfig,
}

impl MockServerManager {
    pub fn start_all(&mut self) -> Result<()> {
        // Start all required mock servers
    }
    
    pub fn stop_all(&mut self) {
        // Stop all mock servers
    }
    
    pub fn reset_all(&mut self) {
        // Reset all mock servers to clean state
    }
}
```

## Testing Strategy Integration

### Unit Testing with Mocks

**Approach**: Mock all external dependencies

**Benefits**:
- **Fast execution**: No network calls
- **Reliable**: Consistent test data
- **Isolated**: No external dependencies
- **Comprehensive**: Can test all scenarios

**Implementation**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_npm_package_collection() {
        let npm_mock = mock("GET", "/express")
            .with_status(200)
            .with_body(include_str!("fixtures/npm-express.json"))
            .create();

        let collector = NpmCollector::new();
        let result = collector.collect_package("express").await;
        
        assert!(result.is_ok());
        npm_mock.assert();
    }
}
```

### Integration Testing with Partial Mocks

**Approach**: Mock some APIs, use real APIs for others

**Benefits**:
- **Realistic testing**: Some real API interactions
- **Controlled environment**: Mock problematic APIs
- **Cost effective**: Reduce API usage costs
- **Reliable**: Avoid flaky tests from external APIs

### End-to-End Testing with Full Mocks

**Approach**: Mock all external APIs for complete workflow testing

**Benefits**:
- **Complete workflows**: Test full application flows
- **Predictable**: Consistent test results
- **Fast**: No external API delays
- **Comprehensive**: Test all error scenarios

## Mock Data Maintenance

### Fixture Data Updates

**Strategy**: Regular updates of fixture data to match real API changes

**Process**:
1. **Quarterly review**: Check for API changes
2. **Automated collection**: Scripts to collect fresh fixture data
3. **Validation**: Ensure fixture data matches current API responses
4. **Versioning**: Track fixture data versions

### API Change Detection

**Strategy**: Detect when real APIs change and update mocks accordingly

**Implementation**:
- **API monitoring**: Regular checks of real API responses
- **Change detection**: Compare current responses with fixtures
- **Automated updates**: Update fixtures when changes detected
- **Test validation**: Ensure tests still pass with updated data

## Performance Considerations

### Mock Server Performance

**Optimization**:
- **In-memory servers**: Fast mock server implementations
- **Connection pooling**: Reuse mock server connections
- **Response caching**: Cache frequently used responses
- **Parallel execution**: Run multiple mock servers concurrently

### Test Execution Performance

**Optimization**:
- **Parallel tests**: Run tests in parallel where possible
- **Shared fixtures**: Reuse fixture data across tests
- **Minimal setup**: Reduce test setup overhead
- **Selective mocking**: Only mock what's necessary

## Error Injection Testing

### Purpose

Test error handling and recovery mechanisms by injecting various types of errors

### Error Types

1. **Network Errors**:
   - Connection timeouts
   - DNS resolution failures
   - SSL/TLS errors
   - Connection refused

2. **HTTP Errors**:
   - 4xx client errors
   - 5xx server errors
   - Malformed responses
   - Empty responses

3. **Data Errors**:
   - Invalid JSON
   - Missing fields
   - Wrong data types
   - Encoding issues

### Implementation

```rust
pub struct ErrorInjector {
    error_rate: f32,
    error_types: Vec<ErrorType>,
}

impl ErrorInjector {
    pub fn should_inject_error(&self) -> Option<ErrorType> {
        // Determine if error should be injected
    }
    
    pub fn inject_network_error(&mut self) -> NetworkError {
        // Inject network error
    }
    
    pub fn inject_http_error(&mut self) -> HttpError {
        // Inject HTTP error
    }
}
```

## Mock Validation Strategy

### Purpose

Validate that our mock data accurately represents real API responses to ensure test reliability and catch API changes.

### Validation Test Categories

#### 1. Manual Mock Validation Tests

**Purpose**: Manually run tests that compare mock responses with real API responses

**Implementation**:
```rust
// Manual validation tests (run with: cargo test --features manual-validation)
#[cfg(feature = "manual-validation")]
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn validate_npm_express_mock() {
        // Get real API response
        let real_response = fetch_real_npm_package("express").await?;
        
        // Load mock response
        let mock_response = load_npm_mock("express")?;
        
        // Compare key fields
        assert_eq!(real_response.name, mock_response.name);
        assert_eq!(real_response.version, mock_response.version);
        assert_eq!(real_response.description, mock_response.description);
        
        // Validate structure matches
        validate_response_structure(&real_response, &mock_response);
    }
    
    #[tokio::test]
    async fn validate_pypi_requests_mock() {
        // Similar validation for PyPI
    }
}
```

**Usage**:
```bash
# Run manual validation tests
cargo test --features manual-validation

# Run validation for specific package manager
cargo test --features manual-validation validate_npm

# Run validation with verbose output
cargo test --features manual-validation -- --nocapture
```

#### 2. Mock Accuracy Validation

**Purpose**: Validate that mock responses contain realistic and accurate data

**Validation Criteria**:
- **Field Completeness**: All expected fields are present
- **Data Types**: Fields have correct data types
- **Value Ranges**: Numeric values are within realistic ranges
- **Format Consistency**: Dates, URLs, and other formats are correct
- **Relationship Integrity**: Related fields are consistent

**Implementation**:
```rust
pub struct MockValidator {
    package_registry: PackageRegistry,
    validation_rules: ValidationRules,
}

impl MockValidator {
    pub fn validate_mock_response(&self, mock: &MockResponse) -> ValidationResult {
        let mut errors = Vec::new();
        
        // Validate required fields
        if mock.name.is_empty() {
            errors.push("Package name is empty");
        }
        
        // Validate version format
        if !self.is_valid_version(&mock.version) {
            errors.push("Invalid version format");
        }
        
        // Validate numeric ranges
        if mock.downloads < 0 {
            errors.push("Downloads count cannot be negative");
        }
        
        // Validate URL formats
        if let Some(url) = &mock.homepage {
            if !self.is_valid_url(url) {
                errors.push("Invalid homepage URL");
            }
        }
        
        ValidationResult { errors }
    }
}
```

#### 3. API Change Detection

**Purpose**: Detect when real APIs change and alert developers

**Detection Methods**:
- **Schema Changes**: Detect new/removed fields
- **Response Format Changes**: Detect changes in data structure
- **Endpoint Changes**: Detect new/removed endpoints
- **Authentication Changes**: Detect changes in auth requirements

**Implementation**:
```rust
pub struct ApiChangeDetector {
    baseline_responses: HashMap<String, ApiResponse>,
    current_responses: HashMap<String, ApiResponse>,
}

impl ApiChangeDetector {
    pub async fn detect_changes(&mut self) -> Vec<ApiChange> {
        let mut changes = Vec::new();
        
        for (package, current) in &self.current_responses {
            if let Some(baseline) = self.baseline_responses.get(package) {
                // Compare responses
                let diff = self.compare_responses(baseline, current);
                if !diff.is_empty() {
                    changes.push(ApiChange {
                        package: package.clone(),
                        changes: diff,
                        severity: self.assess_change_severity(&diff),
                    });
                }
            }
        }
        
        changes
    }
}
```

#### 4. Mock Data Freshness Validation

**Purpose**: Ensure mock data is not stale and represents current API behavior

**Validation Process**:
1. **Timestamp Comparison**: Compare mock data age with real API data
2. **Version Validation**: Check if package versions in mocks are current
3. **Download Count Validation**: Verify download counts are realistic
4. **Dependency Validation**: Check if dependencies are current

**Implementation**:
```rust
pub struct FreshnessValidator {
    max_age_days: u32,
    current_packages: HashSet<String>,
}

impl FreshnessValidator {
    pub fn validate_freshness(&self, mock: &MockResponse) -> FreshnessResult {
        let mut issues = Vec::new();
        
        // Check data age
        if mock.last_updated < chrono::Utc::now() - Duration::days(self.max_age_days as i64) {
            issues.push("Mock data is stale");
        }
        
        // Check if package versions are current
        if !self.is_current_version(&mock.name, &mock.version) {
            issues.push("Package version is not current");
        }
        
        FreshnessResult { issues }
    }
}
```

### Manual Test Execution Strategy

#### 1. Pre-Release Validation

**Purpose**: Validate all mocks before major releases

**Process**:
```bash
# Run comprehensive validation
cargo test --features manual-validation -- --test-threads=1

# Generate validation report
cargo test --features manual-validation -- --nocapture > validation-report.txt

# Check for API changes
cargo test --features api-change-detection
```

#### 2. Periodic Validation

**Purpose**: Regular validation to catch API changes

**Schedule**:
- **Weekly**: Quick validation of critical packages
- **Monthly**: Comprehensive validation of all packages
- **Quarterly**: Full mock data refresh

**Implementation**:
```bash
# Weekly validation script
#!/bin/bash
echo "Running weekly mock validation..."

# Validate critical packages
cargo test --features manual-validation validate_critical_packages

# Check for API changes
cargo test --features api-change-detection

# Generate report
cargo test --features manual-validation -- --nocapture > weekly-validation-$(date +%Y%m%d).txt
```

#### 3. Development Validation

**Purpose**: Validate mocks during development

**Usage**:
```bash
# Validate specific package manager
cargo test --features manual-validation validate_npm

# Validate specific package
cargo test --features manual-validation validate_npm_express

# Quick validation check
cargo test --features quick-validation
```

### Validation Reporting

#### 1. Validation Reports

**Purpose**: Generate detailed reports of mock validation results

**Report Contents**:
- **Validation Summary**: Overall validation status
- **Package-by-Package Results**: Detailed results for each package
- **API Changes Detected**: List of detected API changes
- **Recommendations**: Suggestions for mock updates

**Implementation**:
```rust
pub struct ValidationReporter {
    results: Vec<ValidationResult>,
    changes: Vec<ApiChange>,
}

impl ValidationReporter {
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Mock Validation Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", chrono::Utc::now()));
        
        // Summary
        report.push_str("## Summary\n");
        report.push_str(&format!("Total packages validated: {}\n", self.results.len()));
        report.push_str(&format!("API changes detected: {}\n", self.changes.len()));
        
        // Detailed results
        for result in &self.results {
            report.push_str(&format!("### {}\n", result.package));
            report.push_str(&format!("Status: {}\n", result.status));
            if !result.issues.is_empty() {
                report.push_str("Issues:\n");
                for issue in &result.issues {
                    report.push_str(&format!("- {}\n", issue));
                }
            }
        }
        
        report
    }
}
```

#### 2. Change Notifications

**Purpose**: Notify developers when API changes are detected

**Notification Methods**:
- **Console Output**: Immediate feedback during validation
- **Log Files**: Detailed logs for analysis
- **Email Alerts**: Notifications for critical changes
- **GitHub Issues**: Automatic issue creation for API changes

### Mock Update Strategy

#### 1. Automated Updates

**Purpose**: Automatically update mocks when minor changes are detected

**Update Criteria**:
- **Non-breaking changes**: New optional fields
- **Version updates**: New package versions
- **Download count updates**: Updated download statistics

**Implementation**:
```rust
pub struct MockUpdater {
    validation_results: ValidationResults,
    update_rules: UpdateRules,
}

impl MockUpdater {
    pub fn should_update_mock(&self, change: &ApiChange) -> bool {
        match change.severity {
            ChangeSeverity::Minor => true,
            ChangeSeverity::Moderate => self.update_rules.auto_update_moderate,
            ChangeSeverity::Major => false, // Require manual review
        }
    }
    
    pub async fn update_mock(&self, package: &str, new_data: &ApiResponse) -> Result<()> {
        // Update mock data file
        let mock_file = format!("tests/fixtures/{}.json", package);
        let updated_mock = self.transform_to_mock(new_data);
        fs::write(mock_file, serde_json::to_string_pretty(&updated_mock)?)?;
        
        // Update validation baseline
        self.update_baseline(package, new_data).await?;
        
        Ok(())
    }
}
```

#### 2. Manual Review Process

**Purpose**: Manual review for significant API changes

**Review Criteria**:
- **Breaking changes**: Removed fields or changed data types
- **New required fields**: Fields that are now mandatory
- **Authentication changes**: Changes in auth requirements
- **Rate limit changes**: Changes in rate limiting

**Process**:
1. **Detect significant changes**: Automated detection of major changes
2. **Generate review report**: Detailed report of changes
3. **Manual review**: Developer review of changes
4. **Update mocks**: Manual update of mock data
5. **Validate updates**: Re-run validation tests

## Continuous Integration Integration

### Mock Server in CI/CD

**Strategy**: Use mock servers in CI/CD pipelines for reliable testing

**Benefits**:
- **Reliable builds**: No dependency on external APIs
- **Fast execution**: No network delays
- **Cost effective**: No API usage costs
- **Consistent results**: Same test results every time

**Implementation**:
```yaml
# GitHub Actions example
- name: Start Mock Servers
  run: |
    cargo test --features mock-servers -- --test-threads=1
```

### Mock Data Validation

**Strategy**: Validate mock data against real APIs in CI/CD

**Process**:
1. **Daily validation**: Check fixture data against real APIs
2. **Change detection**: Alert when APIs change
3. **Automated updates**: Update fixtures when needed
4. **Test validation**: Ensure tests pass with updated data

## Best Practices

### Mock Design Principles

1. **Realistic Data**: Use real API response data as fixtures
2. **Comprehensive Coverage**: Mock all API endpoints used
3. **Error Scenarios**: Include comprehensive error scenarios
4. **Maintainable**: Easy to update and maintain
5. **Fast**: Minimize test execution time

### Mock Maintenance

1. **Regular Updates**: Keep fixture data current
2. **Version Control**: Track fixture data versions
3. **Documentation**: Document mock scenarios and purposes
4. **Validation**: Regularly validate mock data
5. **Cleanup**: Remove obsolete mock data

### Testing Guidelines

1. **Isolation**: Each test should be independent
2. **Deterministic**: Tests should produce consistent results
3. **Fast**: Minimize test execution time
4. **Comprehensive**: Test all scenarios and edge cases
5. **Maintainable**: Easy to understand and modify

## Conclusion

A comprehensive mocking strategy is essential for the Package Manager Collector due to its integration with 8 different package manager APIs. This strategy provides:

- **Reliable Testing**: Consistent test results without external dependencies
- **Fast Development**: Quick test execution and feedback
- **Cost Effective**: Reduced API usage costs during testing
- **Comprehensive Coverage**: Test all scenarios including error conditions
- **Maintainable**: Easy to update and maintain mock data

The mocking strategy should be integrated into the implementation plan and design documents to ensure proper testing coverage throughout the development process.
