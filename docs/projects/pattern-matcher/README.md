# Pattern Matcher

## Overview

The Pattern Matcher is an LLM-powered tool that performs specific, concrete analysis tasks on repository content to identify patterns that automated scripts cannot detect.

## Purpose

- **Detect specific file patterns** that indicate practices (e.g., "has .github/dependabot.yml")
- **Analyze specific content patterns** in configuration files
- **Identify specific practice indicators** in documentation and code
- **Generate structured reports** with specific findings
- **Complement automated detection** with content analysis

## Design Principles

1. **LLM-Powered Analysis** - Use AI for pattern recognition and content analysis
2. **Judgment-Based Discovery** - Identify patterns requiring human-like understanding
3. **Complementary to Automation** - Work alongside automated tools
4. **Focused Analysis** - Target specific areas where judgment is needed
5. **Efficient Token Usage** - Optimize LLM calls for maximum value

## Specific Analysis Tasks

### 1. Security Practice Detection
- **Dependabot Configuration**: Analyze .github/dependabot.yml for security scanning setup
- **Security Policy**: Check for SECURITY.md file and analyze content
- **Vulnerability Disclosure**: Look for vulnerability disclosure processes in documentation
- **Dependency Scanning**: Detect automated dependency vulnerability scanning
- **Security Headers**: Check for security headers in web applications

### 2. Documentation Quality Assessment
- **README Completeness**: Analyze README.md for required sections (installation, usage, examples)
- **API Documentation**: Check for API documentation generation (OpenAPI, JSDoc, etc.)
- **Contributing Guidelines**: Analyze CONTRIBUTING.md for contribution process clarity
- **Code Examples**: Count and assess quality of code examples in documentation
- **Documentation Structure**: Check for organized documentation hierarchy

### 3. CI/CD Practice Detection
- **GitHub Actions**: Analyze .github/workflows/ for CI/CD pipeline sophistication
- **Testing Integration**: Detect automated testing in CI/CD pipelines
- **Deployment Automation**: Check for automated deployment processes
- **Quality Gates**: Look for quality gates in CI/CD (tests, linting, security scans)
- **Multi-Platform Support**: Detect cross-platform testing and deployment

### 4. Code Quality Indicators
- **Linting Configuration**: Check for ESLint, Prettier, or similar configuration files
- **Testing Setup**: Analyze test configuration and setup files
- **Code Coverage**: Look for code coverage reporting in CI/CD
- **Type Safety**: Detect TypeScript usage or similar type safety measures
- **Performance Monitoring**: Check for performance testing and monitoring setup

## Specific LLM Tasks

### 1. File Content Analysis
- **Configuration File Analysis**: Analyze specific config files (.github/dependabot.yml, package.json, Cargo.toml)
- **Documentation Content Analysis**: Analyze README.md, CONTRIBUTING.md, SECURITY.md content
- **CI/CD Workflow Analysis**: Analyze .github/workflows/ files for pipeline sophistication
- **Code Quality Config Analysis**: Analyze linting, testing, and quality configuration files

### 2. Specific Pattern Detection
- **Security Practice Detection**: Look for specific security practices in config files
- **Documentation Quality Assessment**: Check for specific documentation elements
- **CI/CD Sophistication**: Detect specific CI/CD practices and automation
- **Code Quality Practices**: Identify specific code quality measures

### 3. Structured Output Generation
- **Boolean Indicators**: Generate yes/no answers for specific practices
- **Count Metrics**: Count specific elements (e.g., number of test files, documentation sections)
- **Quality Scores**: Generate 0-1 scores for specific quality dimensions
- **Specific Findings**: List specific practices found with evidence

## Specific LLM Prompts

### Security Practice Detection Prompt
```
Analyze the following repository content for specific security practices:

Repository: {repository_name}
Content: {extracted_content}

Answer these specific questions with yes/no and evidence:
1. Does this repository have automated dependency vulnerability scanning? (Look for .github/dependabot.yml)
2. Does this repository have a SECURITY.md file with vulnerability disclosure process?
3. Does this repository use security headers in web applications?
4. Does this repository have automated security testing in CI/CD?
5. Does this repository have dependency update automation?

Provide specific evidence for each finding.
```

### Documentation Quality Assessment Prompt
```
Analyze the following repository content for documentation quality:

Repository: {repository_name}
Content: {extracted_content}

Answer these specific questions with yes/no and evidence:
1. Does the README.md have installation instructions?
2. Does the README.md have usage examples?
3. Does the README.md have API documentation?
4. Does the repository have CONTRIBUTING.md with clear contribution process?
5. Does the repository have organized documentation structure?

Provide specific evidence for each finding.
```

### CI/CD Practice Detection Prompt
```
Analyze the following repository content for CI/CD practices:

Repository: {repository_name}
Content: {extracted_content}

Answer these specific questions with yes/no and evidence:
1. Does this repository have automated testing in CI/CD?
2. Does this repository have automated deployment?
3. Does this repository have quality gates (linting, security scans)?
4. Does this repository have multi-platform testing?
5. Does this repository have automated release processes?

Provide specific evidence for each finding.
```

## Specific Content Extraction

### File Selection (Specific Files)
- **Security Files**: `.github/dependabot.yml`, `SECURITY.md`, `.github/security.yml`
- **Documentation Files**: `README.md`, `CONTRIBUTING.md`, `docs/` directory
- **CI/CD Files**: `.github/workflows/` directory, `package.json`, `Cargo.toml`
- **Quality Files**: `.eslintrc`, `.prettierrc`, `jest.config.js`, `pytest.ini`

### Content Processing (Specific Tasks)
- **Extract specific file contents** for analysis
- **Count specific elements** (test files, documentation sections)
- **Check for specific patterns** in configuration files
- **Validate specific requirements** (installation instructions, examples)
- **Measure specific metrics** (documentation completeness, test coverage)

## Specific Output Format

### Structured Analysis Report
```json
{
  "repository": "tokio-rs/tokio",
  "category": "rust-libraries",
  "analysis_timestamp": "2024-01-15T10:30:00Z",
  "security_practices": {
    "has_dependabot": true,
    "has_security_md": true,
    "has_security_headers": false,
    "has_security_testing": true,
    "has_dependency_automation": true,
    "security_score": 0.8
  },
  "documentation_quality": {
    "has_installation_instructions": true,
    "has_usage_examples": true,
    "has_api_docs": true,
    "has_contributing_guidelines": true,
    "has_organized_structure": true,
    "documentation_score": 0.9
  },
  "cicd_practices": {
    "has_automated_testing": true,
    "has_automated_deployment": true,
    "has_quality_gates": true,
    "has_multi_platform": true,
    "has_automated_releases": true,
    "cicd_score": 0.85
  },
  "code_quality": {
    "has_linting": true,
    "has_testing_setup": true,
    "has_coverage_reporting": true,
    "has_type_safety": true,
    "has_performance_monitoring": false,
    "code_quality_score": 0.75
  },
  "overall_score": 0.825
}
```

## Token Optimization

### Content Filtering
- **Relevance Scoring**: Score content relevance before analysis
- **Size Limits**: Limit content size for LLM processing
- **Priority Selection**: Select highest-priority content for analysis
- **Batch Processing**: Process multiple repositories in batches

### Prompt Optimization
- **Structured Prompts**: Use structured prompts for consistent output
- **Context Optimization**: Optimize context for maximum relevance
- **Output Formatting**: Use structured output formats
- **Error Handling**: Handle LLM errors and retries

## Performance Considerations

### LLM Call Optimization
- **Batch Processing**: Process multiple repositories together
- **Content Chunking**: Split large content into manageable chunks
- **Caching**: Cache LLM responses for similar content
- **Rate Limiting**: Respect LLM API rate limits

### Cost Management
- **Token Counting**: Track and optimize token usage
- **Content Selection**: Select only relevant content for analysis
- **Prompt Efficiency**: Optimize prompts for maximum information
- **Response Caching**: Cache responses to avoid duplicate calls

## Error Handling

### LLM Errors
- **API Errors**: Handle LLM API errors and retries
- **Rate Limiting**: Handle rate limiting and backoff
- **Content Errors**: Handle content extraction errors
- **Parsing Errors**: Handle response parsing errors

### Data Quality
- **Content Validation**: Validate extracted content
- **Response Validation**: Validate LLM responses
- **Error Reporting**: Comprehensive error logging
- **Fallback Handling**: Handle analysis failures gracefully

## Configuration

### LLM Settings
```yaml
llm:
  model: "gpt-4"
  temperature: 0.1
  max_tokens: 2000
  timeout: 30
  retry_count: 3
```

### Analysis Settings
```yaml
analysis:
  content_limit: 10000
  batch_size: 5
  priority_threshold: 0.7
  cache_responses: true
```

### Category-Specific Settings
```yaml
categories:
  rust-libraries:
    focus_areas: ["security", "performance", "documentation"]
    content_priority: ["Cargo.toml", "README.md", "SECURITY.md"]
  chrome-extensions:
    focus_areas: ["security", "user-experience", "documentation"]
    content_priority: ["manifest.json", "README.md", "SECURITY.md"]
```

## Dependencies

### Python Packages
- `openai` - OpenAI API client
- `anthropic` - Anthropic API client
- `requests` - HTTP client
- `pandas` - Data manipulation
- `json` - JSON processing

### External Services
- **OpenAI API** - Primary LLM service
- **Anthropic API** - Alternative LLM service
- **Local Storage** - Content and response caching

## Success Metrics

- **Pattern Discovery Rate** - 80% of repositories have discoverable patterns
- **Innovation Identification** - 20% of repositories have novel practices
- **Token Efficiency** - 50% reduction in token usage through optimization
- **Analysis Quality** - 90% of analyses provide actionable insights
- **Processing Speed** - 10 repositories per hour
