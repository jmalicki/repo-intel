# Report Generator - Detailed Design

**Parent:** [Report Generator](README.md)
**Related:** [Common Library](../common-library/DESIGN.md) - Shared functionality dependencies

## Overview

The Report Generator is a **Rust application** that generates comprehensive reports from aggregated data, analysis results, and quality assessments. It creates structured reports for project selection, category representation, and final quality assessments. It uses the Common Library for data processing, storage, validation, and configuration management.

## Programming Language: Rust

### Justification for Rust
- **Performance**: Excellent for high-performance report generation with minimal resource usage
- **Memory Safety**: Prevents data corruption during large-scale report processing
- **Concurrency**: Native async/await for efficient parallel report generation
- **Error Handling**: Robust error handling with Result types
- **Data Processing**: Excellent crates like `serde`, `tokio`, `diesel-async`
- **Database Integration**: Async SQLite support with `diesel-async`
- **JSON Processing**: Fast JSON parsing and serialization with `serde_json`

### Key Rust Crates
- `diesel` + `diesel-async` - Type-safe ORM with async support
- `tokio` - Async runtime for concurrent operations
- `serde` + `serde_json` - JSON serialization/deserialization
- `chrono` - Date/time handling
- `clap` - Command-line argument parsing
- `tracing` - Structured logging
- `anyhow` + `thiserror` - Error handling
- `handlebars` - Template engine for report generation

## Architecture

### Core Components

```
ReportGenerator
├── src/
│   ├── main.rs                 # Application entry point
│   ├── config/                 # Configuration management
│   │   ├── mod.rs
│   │   ├── settings.rs         # Configuration structs
│   │   └── validation.rs       # Config validation
│   ├── generators/             # Report generation logic
│   │   ├── mod.rs
│   │   ├── project_selection.rs # Project selection report generation
│   │   ├── category_representation.rs # Category representation report generation
│   │   ├── final_assessment.rs # Final assessment report generation
│   │   └── summary_report.rs   # Summary report generation
│   ├── processors/             # Data processing logic
│   │   ├── mod.rs
│   │   ├── aggregator.rs       # Data aggregation operations
│   │   ├── normalizer.rs       # Data normalization
│   │   └── validator.rs        # Data validation and quality checks
│   ├── templates/              # Report templates
│   │   ├── mod.rs
│   │   ├── project_selection.hbs # Project selection template
│   │   ├── category_representation.hbs # Category representation template
│   │   ├── final_assessment.hbs # Final assessment template
│   │   └── summary_report.hbs  # Summary report template
│   ├── storage/                # Data storage
│   │   ├── mod.rs
│   │   ├── database.rs         # SQLite operations
│   │   ├── files.rs            # File I/O operations
│   │   └── backup.rs           # Backup operations
│   ├── models/                 # Data models
│   │   ├── mod.rs
│   │   ├── report.rs           # Report data structures
│   │   ├── template.rs         # Template data structures
│   │   ├── data.rs             # Data structures
│   │   └── metadata.rs         # Metadata structures
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
-- Reports table
CREATE TABLE reports (
    id INTEGER PRIMARY KEY,
    report_type TEXT NOT NULL,  -- project_selection, category_representation, final_assessment
    report_name TEXT NOT NULL,
    report_status TEXT NOT NULL,  -- pending, generating, completed, failed
    report_data TEXT NOT NULL,  -- JSON with report data
    report_template TEXT NOT NULL,
    generated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    error_message TEXT
);

-- Report templates table
CREATE TABLE report_templates (
    id INTEGER PRIMARY KEY,
    template_name TEXT UNIQUE NOT NULL,
    template_type TEXT NOT NULL,
    template_content TEXT NOT NULL,
    template_variables TEXT,  -- JSON array of required variables
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Report generation metadata table
CREATE TABLE report_generation_metadata (
    id INTEGER PRIMARY KEY,
    generation_run_id TEXT NOT NULL,
    report_type TEXT NOT NULL,
    reports_generated INTEGER NOT NULL,
    errors_count INTEGER NOT NULL,
    started_at DATETIME NOT NULL,
    completed_at DATETIME NOT NULL,
    duration_seconds REAL NOT NULL
);

-- Report data cache table
CREATE TABLE report_data_cache (
    id INTEGER PRIMARY KEY,
    cache_key TEXT UNIQUE NOT NULL,
    data_type TEXT NOT NULL,
    cached_data TEXT NOT NULL,  -- JSON with cached data
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL
);
```

### Secondary Storage: JSON Files
**Rationale**: Human-readable backups, easy data exchange, version control friendly

#### File Organization
```
data/
├── reports/
│   ├── generated/
│   │   ├── 2024-01-16/
│   │   │   ├── project-selection-report.json
│   │   │   ├── category-representation-report.json
│   │   │   ├── final-assessment-report.json
│   │   │   └── summary-report.json
│   │   └── latest/
│   ├── templates/
│   │   ├── project-selection.hbs
│   │   ├── category-representation.hbs
│   │   ├── final-assessment.hbs
│   │   └── summary-report.hbs
│   └── metadata/
│       ├── generation-runs.json
│       └── template-versions.json
└── cache/
    ├── report-cache/
    └── template-cache/
```

## Core Data Models

### Data Models

**Report Model**:
- **Report Information**: Report type, name, status, data
- **Template Information**: Template used for report generation
- **Generation Metadata**: Generation timestamps, completion status
- **Error Handling**: Error messages and status tracking

**Report Template Model**:
- **Template Information**: Template name, type, content
- **Template Variables**: Required variables for template rendering
- **Template Management**: Active status, creation and update timestamps
- **Template Configuration**: Template configuration and settings

**Report Generation Metadata Model**:
- **Generation Tracking**: Generation run ID and execution metadata
- **Processing Statistics**: Reports generated, errors encountered
- **Performance Metrics**: Execution time and efficiency tracking

**Report Data Cache Model**:
- **Cache Management**: Cache key, data type, cached data
- **Cache Expiration**: Cache expiration and cleanup management
- **Performance Optimization**: Cached data for faster report generation

## Report Generation Design

### Project Selection Report Generator
**Purpose**: Generates comprehensive project selection reports

**Key Components**:
- **Project Analysis**: Analysis of candidate projects and their metrics
- **Quality Assessment**: Quality assessment and scoring of projects
- **Selection Criteria**: Application of selection criteria and thresholds
- **Ranking and Prioritization**: Ranking and prioritization of projects

**API Surface**:
- `ProjectSelectionGenerator::generate_report()` - Generate project selection report
- `ProjectSelectionGenerator::analyze_projects()` - Analyze candidate projects
- `ProjectSelectionGenerator::assess_quality()` - Assess project quality
- `ProjectSelectionGenerator::rank_projects()` - Rank and prioritize projects

### Category Representation Report Generator
**Purpose**: Generates category representation and diversity reports

**Key Components**:
- **Category Analysis**: Analysis of project categories and representation
- **Diversity Assessment**: Assessment of diversity across categories
- **Representation Metrics**: Calculation of representation metrics
- **Diversity Recommendations**: Recommendations for improving diversity

**API Surface**:
- `CategoryRepresentationGenerator::generate_report()` - Generate category representation report
- `CategoryRepresentationGenerator::analyze_categories()` - Analyze project categories
- `CategoryRepresentationGenerator::assess_diversity()` - Assess category diversity
- `CategoryRepresentationGenerator::recommend_improvements()` - Recommend diversity improvements

### Final Assessment Report Generator
**Purpose**: Generates final assessment and quality reports

**Key Components**:
- **Final Assessment**: Final assessment of selected projects
- **Quality Validation**: Validation of quality assessments
- **Excellence Identification**: Identification of excellence examples
- **Final Recommendations**: Final recommendations and conclusions

**API Surface**:
- `FinalAssessmentGenerator::generate_report()` - Generate final assessment report
- `FinalAssessmentGenerator::validate_assessments()` - Validate quality assessments
- `FinalAssessmentGenerator::identify_excellence()` - Identify excellence examples
- `FinalAssessmentGenerator::generate_recommendations()` - Generate final recommendations

### Summary Report Generator
**Purpose**: Generates summary reports and executive summaries

**Key Components**:
- **Executive Summary**: Executive summary generation
- **Key Findings**: Key findings and insights extraction
- **Recommendations Summary**: Summary of recommendations
- **Next Steps**: Next steps and action items

**API Surface**:
- `SummaryReportGenerator::generate_summary()` - Generate executive summary
- `SummaryReportGenerator::extract_findings()` - Extract key findings
- `SummaryReportGenerator::summarize_recommendations()` - Summarize recommendations
- `SummaryReportGenerator::identify_next_steps()` - Identify next steps

## Data Processing Design

### Data Aggregation Strategy
**Purpose**: Aggregates data from multiple sources for report generation

**Key Components**:
- **Data Collection**: Collection of data from multiple sources
- **Data Integration**: Integration of data from different sources
- **Data Validation**: Validation of aggregated data quality
- **Data Transformation**: Transformation of data for report generation

**API Surface**:
- `DataAggregator::collect_data()` - Collect data from multiple sources
- `DataAggregator::integrate_data()` - Integrate data from different sources
- `DataAggregator::validate_data()` - Validate aggregated data
- `DataAggregator::transform_data()` - Transform data for reports

### Data Normalization Strategy
**Purpose**: Normalizes data for consistent report generation

**Key Components**:
- **Data Standardization**: Standardization of data formats and values
- **Data Scaling**: Scaling of data for comparison
- **Data Validation**: Validation of normalized data
- **Data Consistency**: Consistency checking across data sources

**API Surface**:
- `DataNormalizer::standardize_data()` - Standardize data formats
- `DataNormalizer::scale_data()` - Scale data for comparison
- `DataNormalizer::validate_normalized_data()` - Validate normalized data
- `DataNormalizer::check_consistency()` - Check data consistency

### Data Validation Strategy
**Purpose**: Validates data quality and integrity for report generation

**Key Components**:
- **Data Quality Assessment**: Assessment of data quality and completeness
- **Data Integrity Checking**: Checking of data integrity and consistency
- **Data Validation Rules**: Application of data validation rules
- **Data Error Reporting**: Reporting of data validation errors

**API Surface**:
- `DataValidator::assess_quality()` - Assess data quality
- `DataValidator::check_integrity()` - Check data integrity
- `DataValidator::apply_validation_rules()` - Apply validation rules
- `DataValidator::report_errors()` - Report validation errors

## Template Engine Design

### Template Management Strategy
**Purpose**: Manages report templates and template rendering

**Key Components**:
- **Template Loading**: Loading and compilation of report templates
- **Template Variables**: Management of template variables and substitution
- **Template Rendering**: Rendering of templates with data
- **Template Caching**: Caching of compiled templates for performance

**API Surface**:
- `TemplateManager::load_template()` - Load and compile report templates
- `TemplateManager::manage_variables()` - Manage template variables
- `TemplateManager::render_template()` - Render templates with data
- `TemplateManager::cache_template()` - Cache compiled templates

### Template Processing Strategy
**Purpose**: Processes templates for report generation

**Key Components**:
- **Template Compilation**: Compilation of templates for rendering
- **Variable Substitution**: Substitution of variables in templates
- **Template Validation**: Validation of template syntax and variables
- **Template Optimization**: Optimization of templates for better performance

**API Surface**:
- `TemplateProcessor::compile_template()` - Compile templates for rendering
- `TemplateProcessor::substitute_variables()` - Substitute variables in templates
- `TemplateProcessor::validate_template()` - Validate template syntax
- `TemplateProcessor::optimize_template()` - Optimize templates for performance

## Storage Library Integration

### Database Operations
**Purpose**: Leverages Common Library database operations for report data persistence

**Key Components**:
- **Connection Management**: Async SQLite connection establishment and pooling
- **Type-Safe Queries**: Compile-time checked SQL operations using diesel
- **Transaction Support**: ACID-compliant database transactions
- **Migration Management**: Database schema versioning and updates
- **Connection Pooling**: Efficient connection reuse for high-throughput operations

**API Surface**:
- `Database::new()` - Establish database connection
- `Database::save_report()` - Save generated reports
- `Database::save_template()` - Save report templates
- `Database::cache_data()` - Cache report data
- `Database::transaction()` - Transaction management
- `Database::migrate()` - Schema migration execution

### File Operations
**Purpose**: Leverages Common Library file operations for report backup and exchange

**Key Components**:
- **JSON Serialization**: Automatic serialization/deserialization of structured data
- **Directory Management**: Automatic directory creation and path resolution
- **Async I/O**: Non-blocking file operations using tokio
- **Error Handling**: Comprehensive error handling for file system operations
- **Path Safety**: Secure path handling and validation

**API Surface**:
- `FileManager::new()` - Initialize with base directory
- `FileManager::save_report()` - Save reports as JSON
- `FileManager::save_template()` - Save report templates
- `FileManager::backup_data()` - Create data backups
- `FileManager::cleanup_old_data()` - Clean up old data files

## Configuration Library Integration

### Configuration Manager
**Purpose**: Leverages Common Library configuration management for application settings

**Key Components**:
- **Multi-Source Loading**: File-based configs with environment variable overrides
- **Type Safety**: Strongly-typed configuration structures
- **Validation**: Runtime configuration validation and error reporting
- **Environment Support**: Development, staging, production environment handling
- **Hot Reloading**: Optional configuration reloading without restart

**API Surface**:
- `ConfigManager::new()` - Load configuration from multiple sources
- `ConfigManager::get_report_config()` - Get report-specific configuration
- `ConfigManager::get_template_config()` - Get template configurations
- `ConfigManager::validate()` - Validate configuration completeness
- `ConfigManager::reload()` - Reload configuration from sources

## Logging Library Integration

### Structured Logger
**Purpose**: Leverages Common Library logging for comprehensive application logging

**Key Components**:
- **Log Levels**: Configurable logging levels (DEBUG, INFO, WARN, ERROR)
- **Structured Output**: JSON-formatted logs with consistent fields
- **Environment Integration**: Environment variable configuration support
- **Performance**: Minimal overhead logging for high-throughput applications

**API Surface**:
- `Logger::new()` - Initialize logger with specified level
- `Logger::init()` - Initialize global logging configuration
- `Logger::log_report_generation()` - Log report generation operations
- `Logger::log_template_processing()` - Log template processing
- `Logger::log_errors()` - Log errors and exceptions

## Testing Strategy

### Unit Tests
**Purpose**: Comprehensive testing of all report generation components with high coverage

**Key Components**:
- **Component Testing**: Individual component functionality testing
- **Integration Testing**: Cross-component interaction testing
- **Performance Testing**: Benchmarking and performance regression testing
- **Error Testing**: Error condition and edge case testing

**Test Categories**:
- **Report Generation Tests**: Report generation, template processing, data aggregation
- **Database Tests**: CRUD operations, transaction handling, connection pooling
- **File Tests**: JSON serialization, backup operations, data cleanup
- **Configuration Tests**: Config loading, validation, environment variable handling
- **Validation Tests**: Data validation, template validation, error handling

### Integration Tests
**Purpose**: End-to-end testing of component interactions and real-world scenarios

**Key Components**:
- **Report Generation Integration**: Full report generation workflow testing
- **Database Integration**: Complete database operation testing with real data
- **End-to-End Workflows**: Full application workflow testing
- **Performance Integration**: Load testing and performance validation

**Test Scenarios**:
- **Report Generation Workflows**: Complete report generation workflows
- **Error Recovery**: Generation errors, data corruption, and retry scenarios
- **Data Consistency**: Cross-component data integrity validation
- **Performance Benchmarks**: Throughput and latency testing

## Performance Considerations

### Memory Management
- **Zero-copy deserialization** where possible
- **Streaming JSON parsing** for large datasets
- **Connection pooling** for database operations
- **Report data caching** for frequently generated reports

### Concurrency
- **Async/await** throughout the API
- **Parallel report generation** for multiple reports
- **Concurrent database operations** with connection pooling
- **Background tasks** for data cleanup and maintenance

### Optimization
- **Batch processing** for database operations
- **Caching** for frequently generated reports
- **Template compilation caching** for better performance
- **Data indexing** for faster report generation
