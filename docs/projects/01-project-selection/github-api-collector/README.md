# GitHub API Collector

**Parent:** [Projects Overview](../README.md)
**Related:** [Automation Opportunities](../../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) - GitHub API Data Collection task

## Overview

The GitHub API Collector is a **deterministic Python script** that makes HTTP requests to GitHub's API, processes JSON responses, and stores data in structured formats. No LLM calls, no human interaction, no subagents.

## Purpose

- **Automate GitHub API data collection** for 160-240 candidate projects
- **Extract repository metadata** (stars, forks, commits, issues, PRs, releases)
- **Calculate derived metrics** (activity rates, community health, growth patterns)
- **Handle API rate limiting** and pagination automatically
- **Store structured data** for downstream analysis

## Design Principles

1. **100% Automation** - No human intervention required
2. **Rate Limit Compliance** - Respect GitHub API limits
3. **Data Completeness** - Collect all relevant repository data
4. **Error Handling** - Robust error handling and retry logic
5. **Scalability** - Handle large numbers of repositories efficiently

## Input Sources

### Category-Specific Searches
- **Chrome Extensions**: `topic:chrome-extension`, `topic:browser-extension`
- **MCP Servers**: `topic:mcp-server`, `topic:model-context-protocol`
- **Rust Libraries**: `language:rust`, `topic:rust-library`
- **Full-Stack Systems**: `topic:fullstack`, `topic:web-application`
- **Data Science & ML**: `topic:machine-learning`, `topic:data-science`
- **CLI Tools**: `topic:cli`, `topic:command-line`
- **Mobile Apps**: `topic:mobile`, `topic:react-native`
- **Documentation Sites**: `topic:documentation`, `topic:docs`

### Search Parameters
- **Star Count**: Minimum thresholds by category
- **Activity**: Recent commit activity
- **Language**: Category-specific language filters
- **Topics**: Category-specific topic tags
- **Sort Order**: Stars, activity, relevance

## Data Collection

### Repository Metadata
- **Basic Info**: Name, description, language, topics, license
- **Statistics**: Stars, forks, watchers, open issues, open PRs
- **Activity**: Commit count, contributor count, release count
- **Timestamps**: Created, updated, pushed dates
- **Size**: Repository size, code size, documentation size

### Historical Data
- **Star History**: Star count over time
- **Fork History**: Fork count over time
- **Activity History**: Commit activity over time
- **Release History**: Release frequency and patterns
- **Issue/PR History**: Issue and PR activity over time

### Community Data
- **Contributors**: Contributor count and activity
- **Collaborators**: Collaborator count and permissions
- **Forks**: Fork count and active forks
- **Issues**: Issue count, open/closed ratio, response time
- **PRs**: PR count, open/closed ratio, merge rate

## API Endpoints Used

### Search Endpoints
- `/search/repositories` - Repository search
- `/search/code` - Code search
- `/search/commits` - Commit search
- `/search/issues` - Issue search

### Repository Endpoints
- `/repos/{owner}/{repo}` - Repository details
- `/repos/{owner}/{repo}/stats/contributors` - Contributor stats
- `/repos/{owner}/{repo}/stats/participation` - Participation stats
- `/repos/{owner}/{repo}/stats/code_frequency` - Code frequency
- `/repos/{owner}/{repo}/stats/commit_activity` - Commit activity

### Rate Limiting
- **Authenticated**: 5,000 requests/hour
- **Unauthenticated**: 60 requests/hour
- **Retry Logic**: Exponential backoff with jitter
- **Queue Management**: Request queuing and prioritization

## Data Storage

### Format
- **Primary**: JSON files per category
- **Secondary**: CSV files for analysis
- **Database**: SQLite for complex queries
- **Backup**: Compressed archives

### Structure
```json
{
  "category": "rust-libraries",
  "timestamp": "2024-01-15T10:30:00Z",
  "repositories": [
    {
      "id": 12345,
      "name": "tokio",
      "full_name": "tokio-rs/tokio",
      "description": "A runtime for writing reliable asynchronous applications",
      "language": "Rust",
      "stars": 25000,
      "forks": 2500,
      "watchers": 500,
      "open_issues": 150,
      "open_prs": 75,
      "created_at": "2016-01-01T00:00:00Z",
      "updated_at": "2024-01-15T10:00:00Z",
      "pushed_at": "2024-01-15T09:30:00Z",
      "size": 50000,
      "license": "MIT",
      "topics": ["async", "runtime", "tokio"],
      "contributors": 150,
      "releases": 50,
      "activity_score": 0.85,
      "community_health": 0.92
    }
  ]
}
```

## Error Handling

### API Errors
- **Rate Limiting**: Automatic retry with backoff
- **Authentication**: Token refresh and retry
- **Network**: Connection retry and timeout handling
- **Data Validation**: Schema validation and error reporting

### Data Quality
- **Missing Data**: Flag incomplete records
- **Inconsistent Data**: Validate data consistency
- **Duplicate Data**: Detect and handle duplicates
- **Data Freshness**: Track data age and staleness

## Performance Optimization

### Caching
- **Response Caching**: Cache API responses
- **Data Caching**: Cache processed data
- **Query Caching**: Cache search results
- **Rate Limit Caching**: Cache rate limit status

### Parallel Processing
- **Concurrent Requests**: Parallel API calls
- **Batch Processing**: Batch similar requests
- **Queue Management**: Prioritize important requests
- **Resource Management**: Control memory and CPU usage

## Output

### Files Generated
- `github_data_{category}_{timestamp}.json` - Raw API data
- `github_metrics_{category}_{timestamp}.csv` - Processed metrics
- `github_summary_{category}_{timestamp}.json` - Summary statistics
- `github_errors_{timestamp}.log` - Error log

### Data Quality Report
- **Total Repositories**: Count by category
- **Data Completeness**: Percentage of complete records
- **API Errors**: Error count and types
- **Processing Time**: Collection duration
- **Rate Limit Usage**: API quota consumption

## Configuration

### API Configuration
```yaml
github:
  api_url: "https://api.github.com"
  token: "${GITHUB_TOKEN}"
  rate_limit: 5000
  timeout: 30
  retry_count: 3
  retry_delay: 1
```

### Search Configuration
```yaml
categories:
  rust-libraries:
    queries:
      - "language:rust stars:>1000"
      - "topic:rust-library stars:>500"
    min_stars: 1000
    min_activity: 0.1
    max_results: 30
```

## Dependencies

### Python Packages
- `requests` - HTTP client
- `python-dateutil` - Date parsing
- `pandas` - Data manipulation
- `sqlite3` - Database operations
- `yaml` - Configuration parsing

### External Services
- **GitHub API** - Primary data source
- **GitHub Token** - Authentication
- **Local Storage** - Data persistence

## Success Metrics

- **Data Completeness** - 95% of repositories have complete data
- **API Efficiency** - 90% of API quota utilization
- **Processing Speed** - 100 repositories per hour
- **Error Rate** - Less than 5% API errors
- **Data Quality** - 99% data validation success
