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
- **Dependabot Check**: Does .github/dependabot.yml exist and have security updates enabled?
- **Security Policy Check**: Does SECURITY.md exist and contain vulnerability disclosure process?
- **Dependency Scanning Check**: Are there security scanning tools in CI/CD (Snyk, OWASP, etc.)?
- **Security Headers Check**: Do web apps have security headers (CSP, HSTS, etc.)?
- **Dependency Update Check**: Are there automated dependency update workflows?

### 2. Documentation Quality Assessment
- **README Installation Check**: Does README.md have installation instructions?
- **README Usage Check**: Does README.md have usage examples with code?
- **API Docs Check**: Are there API docs (OpenAPI spec, JSDoc comments, etc.)?
- **Contributing Check**: Does CONTRIBUTING.md exist and have clear contribution process?
- **Docs Structure Check**: Is there a docs/ directory with organized documentation?

### 3. CI/CD Practice Detection
- **GitHub Actions Check**: Are there .github/workflows/ files with CI/CD pipelines?
- **Testing Integration Check**: Do CI/CD pipelines run automated tests?
- **Deployment Check**: Are there automated deployment workflows?
- **Quality Gates Check**: Do CI/CD pipelines have quality gates (linting, security scans)?
- **Multi-Platform Check**: Are there cross-platform testing workflows (Linux, Windows, macOS)?

### 4. Code Quality Indicators
- **Linting Check**: Are there linting config files (.eslintrc, .prettierrc, etc.)?
- **Testing Setup Check**: Are there test config files (jest.config.js, pytest.ini, etc.)?
- **Coverage Check**: Is there code coverage reporting in CI/CD?
- **Type Safety Check**: Is TypeScript or similar type safety used?
- **Performance Check**: Are there performance testing or monitoring tools?

## Specific LLM Tasks

### 1. File Content Analysis
- **Check .github/dependabot.yml**: Does it exist and have security updates enabled?
- **Check SECURITY.md**: Does it exist and contain vulnerability disclosure process?
- **Check README.md**: Does it have installation instructions and usage examples?
- **Check CONTRIBUTING.md**: Does it exist and have clear contribution process?
- **Check .github/workflows/**: Are there CI/CD pipeline files?

### 2. Specific Pattern Detection
- **Count test files**: How many test files are there (test/, tests/, __tests__/)?
- **Count documentation files**: How many documentation files are there?
- **Check for linting configs**: Are there .eslintrc, .prettierrc, etc.?
- **Check for type safety**: Is TypeScript or similar used?
- **Check for coverage reporting**: Is code coverage reported in CI/CD?

### 3. Structured Output Generation
- **Boolean answers**: Yes/No for each specific check
- **Count metrics**: Number of test files, docs files, etc.
- **Quality scores**: 0-1 scores for each quality dimension
- **Evidence**: Specific file names and content snippets as evidence

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
