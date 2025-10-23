# Metrics Calculator

## Overview

The Metrics Calculator is a **deterministic Python script** that performs mathematical calculations on repository data to generate scores, rankings, and metrics. No LLM calls, no human interaction, no subagents.

## Purpose

- **Calculate quantitative metrics** from raw repository data
- **Generate scoring systems** for project evaluation
- **Apply filtering thresholds** to identify quality projects
- **Rank projects** by various criteria
- **Generate metric reports** for human review

## Design Principles

1. **100% Automation** - No human intervention required
2. **Mathematical Precision** - Accurate calculations and formulas
3. **Configurable Thresholds** - Adjustable quality filters
4. **Scalable Processing** - Handle large datasets efficiently
5. **Reproducible Results** - Consistent calculations across runs

## Input Data

### Repository Data
- **Basic Metrics**: Stars, forks, watchers, size
- **Activity Metrics**: Commits, contributors, releases
- **Community Metrics**: Issues, PRs, discussions
- **Temporal Metrics**: Creation date, last update, activity history

### Category-Specific Data
- **Package Downloads**: NPM, PyPI, crates.io statistics
- **App Store Metrics**: Ratings, reviews, downloads
- **Documentation Metrics**: File counts, update frequency
- **Security Metrics**: Vulnerability counts, security practices

## Metric Categories

### 1. Popularity Metrics
- **Star Growth Rate**: Stars per month over time
- **Fork Ratio**: Forks per star ratio
- **Watcher Engagement**: Watchers per star ratio
- **Download Velocity**: Package download trends

### 2. Activity Metrics
- **Commit Frequency**: Commits per week/month
- **Contributor Activity**: Active contributors per month
- **Release Frequency**: Releases per month
- **Issue Resolution**: Issues closed per week

### 3. Community Health Metrics
- **Response Time**: Average issue/PR response time
- **Resolution Rate**: Issues/PRs closed vs. opened
- **Contributor Diversity**: Unique contributors over time
- **Community Engagement**: Comments, reactions, discussions

### 4. Quality Metrics
- **Documentation Coverage**: Documentation file presence
- **Test Coverage**: Test file presence and coverage
- **Security Practices**: Security file presence
- **Code Quality**: File structure and organization

### 5. Maintenance Metrics
- **Update Frequency**: Repository update patterns
- **Dependency Management**: Dependency update frequency
- **Version Management**: Semantic versioning practices
- **Breaking Changes**: Breaking change frequency

## Calculation Formulas

### Star Growth Rate
```
star_growth_rate = (current_stars - stars_6_months_ago) / 6
```

### Activity Score
```
activity_score = (commits_last_month * 0.4) + 
                 (contributors_last_month * 0.3) + 
                 (releases_last_month * 0.3)
```

### Community Health Score
```
community_health = (response_time_score * 0.3) + 
                   (resolution_rate_score * 0.4) + 
                   (contributor_diversity_score * 0.3)
```

### Quality Score
```
quality_score = (documentation_score * 0.3) + 
                (test_coverage_score * 0.3) + 
                (security_score * 0.2) + 
                (code_quality_score * 0.2)
```

### Overall Score
```
overall_score = (popularity_score * 0.3) + 
                (activity_score * 0.25) + 
                (community_health * 0.25) + 
                (quality_score * 0.2)
```

## Scoring Systems

### Normalized Scoring (0-1)
- **Min-Max Normalization**: Scale values to 0-1 range
- **Percentile Ranking**: Rank-based scoring
- **Z-Score Normalization**: Statistical normalization
- **Logarithmic Scaling**: Handle exponential distributions

### Category-Specific Scoring
- **Rust Libraries**: Performance benchmarks, memory safety
- **Chrome Extensions**: User ratings, security practices
- **Full-Stack Systems**: Architecture complexity, scalability
- **Data Science**: Reproducibility, documentation quality

## Filtering Thresholds

### Minimum Requirements
- **Stars**: Minimum star count by category
- **Activity**: Minimum commit frequency
- **Community**: Minimum contributor count
- **Documentation**: Minimum documentation coverage
- **Security**: Minimum security practices

### Quality Thresholds
- **Response Time**: Maximum issue response time
- **Resolution Rate**: Minimum issue resolution rate
- **Update Frequency**: Minimum update frequency
- **Test Coverage**: Minimum test coverage

## Ranking Systems

### Multi-Criteria Ranking
- **Weighted Sum**: Weighted combination of metrics
- **Pareto Ranking**: Non-dominated solutions
- **TOPSIS**: Technique for Order Preference by Similarity
- **AHP**: Analytic Hierarchy Process

### Category-Specific Rankings
- **Popularity Ranking**: Star count, download count
- **Activity Ranking**: Commit frequency, contributor activity
- **Quality Ranking**: Documentation, testing, security
- **Community Ranking**: Response time, resolution rate

## Data Processing

### Data Cleaning
- **Missing Values**: Handle missing data appropriately
- **Outliers**: Detect and handle outliers
- **Data Validation**: Validate data consistency
- **Data Transformation**: Normalize and scale data

### Feature Engineering
- **Derived Metrics**: Calculate composite metrics
- **Temporal Features**: Time-based feature extraction
- **Categorical Features**: Encode categorical variables
- **Interaction Features**: Feature interactions

## Output Formats

### JSON Output
```json
{
  "category": "rust-libraries",
  "timestamp": "2024-01-15T10:30:00Z",
  "metrics": {
    "popularity_score": 0.85,
    "activity_score": 0.92,
    "community_health": 0.78,
    "quality_score": 0.88,
    "overall_score": 0.86
  },
  "rankings": {
    "popularity_rank": 5,
    "activity_rank": 3,
    "community_rank": 8,
    "quality_rank": 2,
    "overall_rank": 4
  },
  "thresholds": {
    "meets_minimum": true,
    "meets_quality": true,
    "meets_community": false
  }
}
```

### CSV Output
```csv
repository,stars,forks,activity_score,community_health,quality_score,overall_score,rank
tokio,25000,2500,0.92,0.78,0.88,0.86,4
serde,15000,1200,0.85,0.82,0.91,0.87,3
```

## Configuration

### Metric Weights
```yaml
weights:
  popularity: 0.3
  activity: 0.25
  community_health: 0.25
  quality: 0.2
```

### Thresholds
```yaml
thresholds:
  min_stars: 1000
  min_activity: 0.1
  min_community_health: 0.5
  min_quality: 0.6
```

### Category-Specific Settings
```yaml
categories:
  rust-libraries:
    min_stars: 1000
    min_activity: 0.2
    quality_weights:
      documentation: 0.3
      testing: 0.3
      security: 0.2
      performance: 0.2
```

## Performance Optimization

### Parallel Processing
- **Multi-threading**: Parallel metric calculations
- **Vectorization**: NumPy vectorized operations
- **Caching**: Cache intermediate results
- **Memory Management**: Efficient memory usage

### Data Structures
- **Pandas DataFrames**: Efficient data manipulation
- **NumPy Arrays**: Fast numerical computations
- **SQLite**: Efficient data storage and querying
- **JSON**: Lightweight data exchange

## Error Handling

### Data Validation
- **Schema Validation**: Validate input data structure
- **Range Validation**: Check metric value ranges
- **Consistency Checks**: Validate data consistency
- **Error Reporting**: Comprehensive error logging

### Calculation Errors
- **Division by Zero**: Handle zero denominators
- **Invalid Values**: Handle NaN and infinity
- **Overflow/Underflow**: Handle numerical overflow
- **Precision Issues**: Handle floating-point precision

## Dependencies

### Python Packages
- `pandas` - Data manipulation
- `numpy` - Numerical computations
- `scipy` - Statistical functions
- `scikit-learn` - Machine learning utilities
- `sqlite3` - Database operations

### External Libraries
- **Statistical Libraries**: Advanced statistical functions
- **Optimization Libraries**: Numerical optimization
- **Visualization Libraries**: Data visualization (optional)

## Success Metrics

- **Calculation Accuracy** - 99.9% calculation accuracy
- **Processing Speed** - 1000 repositories per minute
- **Memory Usage** - Less than 1GB for 1000 repositories
- **Error Rate** - Less than 0.1% calculation errors
- **Reproducibility** - 100% reproducible results
