# Package Manager Collector

**Navigation:** [Projects Overview](../README.md) → [Project Selection](../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) → Package Manager Collector

## Overview

The Package Manager Collector is a **deterministic Python script** that makes HTTP requests to package manager APIs (NPM, PyPI, crates.io), processes JSON responses, and stores data. No LLM calls, no human interaction, no subagents.

## Purpose

- **Collect package download statistics** from NPM, PyPI, and crates.io
- **Extract package metadata** including ratings, dependencies, and versions
- **Track download trends** over time periods
- **Cross-reference packages** with GitHub repositories
- **Generate package ecosystem reports** for analysis

## Design Principles

1. **100% Automation** - No human intervention required
2. **Multi-Source Collection** - Collect from multiple package managers
3. **Rate Limit Compliance** - Respect API rate limits
4. **Data Consistency** - Normalize data across different sources
5. **Error Resilience** - Handle API failures gracefully

## Dependencies

This tool uses the **[Common Library](../common-library/README.md)** for:
- **HTTP Client Library** - Rate limiting, retry logic, authentication
- **Data Processing Library** - Validation, normalization, transformation
- **Storage Library** - File I/O, database operations, serialization
- **Logging Library** - Structured logging, performance metrics
- **Configuration Library** - Settings management, environment variables

## Data Sources

### NPM (Node.js)
- **API Endpoint**: `https://registry.npmjs.org/`
- **Data Collected**: Downloads, versions, dependencies, ratings

### PyPI (Python)
- **API Endpoint**: `https://pypi.org/pypi/`
- **Data Collected**: Downloads, versions, dependencies, ratings

### Crates.io (Rust)
- **API Endpoint**: `https://crates.io/api/v1/`
- **Data Collected**: Downloads, versions, dependencies, ratings

## Data Collection Process

### 1. Package Discovery
- **GitHub Repository Analysis** - Extract package names from repository files
- **Package.json Detection** - Find NPM packages
- **setup.py Detection** - Find Python packages
- **Cargo.toml Detection** - Find Rust packages
- **Cross-Reference Validation** - Validate package existence

### 2. Metadata Collection
- **Package Information** - Name, description, version, license
- **Download Statistics** - Total, monthly, weekly, daily downloads
- **Dependency Data** - Dependencies and dependents count
- **Version History** - Version release dates and patterns
- **Rating Data** - User ratings and reviews

### 3. Trend Analysis
- **Download Trends** - Download patterns over time
- **Version Adoption** - Version adoption rates
- **Dependency Growth** - Dependency growth patterns
- **Popularity Trends** - Popularity changes over time

## Data Schema

**See [API_SCHEMAS.md](API_SCHEMAS.md) for actual API response schemas from NPM, PyPI, and Crates.io.**

The package manager APIs provide different data structures. We collect and normalize:
- **Package metadata** (name, description, version, license)
- **Repository information** (GitHub URLs, homepage)
- **Dependencies** (direct dependencies per version)
- **Version history** (release dates, version numbers)
- **Download statistics** (where available via separate endpoints)

## API Integration

### NPM API Integration
```python
def collect_npm_data(package_name):
    url = f"https://registry.npmjs.org/{package_name}"
    response = requests.get(url)

    if response.status_code == 200:
        data = response.json()
        return {
            "downloads": data.get("downloads", {}),
            "versions": data.get("versions", {}),
            "dependencies": data.get("dependencies", {}),
            "dependents": get_dependents(package_name)
        }
    return None
```

### PyPI API Integration
```python
def collect_pypi_data(package_name):
    url = f"https://pypi.org/pypi/{package_name}/json"
    response = requests.get(url)

    if response.status_code == 200:
        data = response.json()
        return {
            "downloads": data.get("downloads", {}),
            "versions": data.get("releases", {}),
            "dependencies": data.get("info", {}).get("requires_dist", []),
            "dependents": get_dependents(package_name)
        }
    return None
```

### Crates.io API Integration
```python
def collect_crates_data(package_name):
    url = f"https://crates.io/api/v1/crates/{package_name}"
    response = requests.get(url)

    if response.status_code == 200:
        data = response.json()
        return {
            "downloads": data.get("crate", {}).get("downloads", 0),
            "versions": data.get("versions", []),
            "dependencies": data.get("crate", {}).get("dependencies", []),
            "dependents": get_dependents(package_name)
        }
    return None
```

## Data Processing

### Download Statistics Processing
- **Total Downloads** - Cumulative download count
- **Time Series Data** - Downloads over time periods
- **Growth Rate Calculation** - Download growth rates
- **Trend Analysis** - Download trend patterns
- **Seasonality Detection** - Seasonal download patterns

### Dependency Analysis
- **Dependency Count** - Number of dependencies
- **Dependent Count** - Number of packages depending on this package
- **Dependency Tree** - Dependency hierarchy
- **Circular Dependencies** - Detect circular dependency issues
- **Dependency Health** - Assess dependency health and maintenance

### Version Analysis
- **Version History** - Version release patterns
- **Semantic Versioning** - Assess semantic versioning compliance
- **Breaking Changes** - Detect breaking changes
- **Version Adoption** - Version adoption rates
- **Release Frequency** - Release frequency patterns

## Error Handling

Uses **[Common Library HTTP Client](../common-library/README.md#http-client-library)** for:
- **Rate Limiting** - Handle rate limit errors with backoff
- **Authentication** - Handle authentication errors
- **Network Errors** - Handle network connectivity issues
- **Data Validation** - Validate API response data
- **Malformed Data** - Handle malformed API responses
- **Duplicate Data** - Handle duplicate package entries

## Performance Optimization

### Caching Strategy
- **Response Caching** - Cache API responses
- **Data Caching** - Cache processed data
- **Query Caching** - Cache package queries
- **Rate Limit Caching** - Cache rate limit status

### Batch Processing
- **Concurrent Requests** - Process multiple packages concurrently
- **Batch API Calls** - Batch similar API calls
- **Queue Management** - Manage request queues
- **Resource Management** - Control memory and CPU usage

## Output Formats

### JSON Output
```json
{
  "collection_timestamp": "2024-01-15T10:30:00Z",
  "total_packages": 240,
  "registries": {
    "npm": 80,
    "pypi": 80,
    "crates.io": 80
  },
  "packages": [...]
}
```

### CSV Output
```csv
package_name,registry,downloads_total,downloads_monthly,rating,dependencies,dependents
lodash,npm,1000000,50000,4.8,0,1500
requests,pypi,800000,40000,4.7,5,800
serde,crates.io,600000,30000,4.9,2,1200
```

## Configuration

Uses **[Common Library Configuration](../common-library/README.md#configuration-library)** for:
- **Settings Management** - YAML, JSON, INI file support
- **Environment Variables** - API tokens and registry settings
- **Default Values** - Sensible defaults for all settings
- **Validation** - Configuration validation and error reporting

## Dependencies

### Python Packages
- `requests` - HTTP client
- `pandas` - Data manipulation
- `numpy` - Numerical computations
- `sqlite3` - Database operations
- `yaml` - Configuration parsing

### External Services
- **NPM Registry** - Node.js packages
- **PyPI** - Python packages
- **Crates.io** - Rust packages
- **Local Storage** - Data persistence

## Success Metrics

- **Data Completeness** - 95% of packages have complete data
- **API Efficiency** - 90% of API quota utilization
- **Processing Speed** - 100 packages per hour
- **Error Rate** - Less than 5% API errors
- **Data Quality** - 99% data validation success
