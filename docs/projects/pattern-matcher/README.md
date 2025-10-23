# Pattern Matcher

## Overview

The Pattern Matcher is a **Python script** that uses LLM analysis to discover novel patterns and practices in repositories. It's designed to find unexpected approaches, not just verify predetermined patterns.

**PURPOSE**: Discover novel practices, innovative approaches, and unexpected patterns that automated scripts cannot detect. This is essential for learning about new ways of doing things.

## What It Actually Does

1. **Takes a repository path** as input
2. **Script scans repository structure** to discover files and patterns
3. **Script makes LLM API calls** to analyze discovered content for novel practices
4. **Script identifies unexpected patterns** and innovative approaches
5. **Returns structured JSON** with discovered patterns and innovations

**The script discovers patterns** - it finds novel approaches, not just verifies predetermined ones.

**This enables discovery of**:
- Novel security practices and tools
- Innovative documentation approaches
- Custom CI/CD implementations
- Unexpected organizational patterns
- New best practices we haven't seen before

## Design Principles

1. **LLM-Powered Analysis** - Use AI for pattern recognition and content analysis
2. **Judgment-Based Discovery** - Identify patterns requiring human-like understanding
3. **Complementary to Automation** - Work alongside automated tools
4. **Focused Analysis** - Target specific areas where judgment is needed
5. **Efficient Token Usage** - Optimize LLM calls for maximum value

## Actual Implementation

### Input
- **Repository path** (e.g., "/path/to/repo" or "owner/repo")
- **Specific files to analyze** (predefined list)

### Process
1. **Read files**: Script reads specific files from repository
2. **LLM API call**: Send file content + prompt to LLM API
3. **Parse response**: Extract structured data from LLM response
4. **Return JSON**: Return structured analysis results

### Output
```json
{
  "repository": "owner/repo",
  "has_dependabot": true,
  "has_security_md": false,
  "readme_has_installation": true,
  "readme_has_examples": true,
  "test_file_count": 15,
  "docs_file_count": 8
}
```

## Specific File Checks

### Security Files
- **Check .github/dependabot.yml**: Read file, send to LLM with prompt "Does this file enable security updates?"
- **Check SECURITY.md**: Read file, send to LLM with prompt "Does this file contain vulnerability disclosure process?"

### Documentation Files  
- **Check README.md**: Read file, send to LLM with prompt "Does this README have installation instructions and usage examples?"
- **Check CONTRIBUTING.md**: Read file, send to LLM with prompt "Does this file have clear contribution process?"

### CI/CD Files
- **Check .github/workflows/**: List directory, send to LLM with prompt "Are there CI/CD pipeline files here?"
- **Check package.json**: Read file, send to LLM with prompt "Does this have test scripts and CI/CD configuration?"

## Script Implementation

### Smart Repository Discovery
```python
def discover_repository_structure(repo_path):
    # Smart scanning - look for patterns, not every file
    all_files = scan_directory(repo_path)
    
    # Use heuristics to identify relevant files
    security_files = find_files_by_pattern(all_files, [
        "security", "vulnerability", "dependabot", "snyk", "owasp"
    ])
    docs_files = find_files_by_pattern(all_files, [
        "readme", "contributing", "docs", "guide", "tutorial"
    ])
    ci_files = find_files_by_pattern(all_files, [
        "workflow", "action", "ci", "cd", "deploy", "test"
    ])
    config_files = find_files_by_pattern(all_files, [
        "config", "package", "cargo", "pom", "requirements"
    ])
    
    return {
        "security": security_files,
        "documentation": docs_files,
        "ci_cd": ci_files,
        "config": config_files
    }
```

### Efficient Pattern Discovery
```python
def analyze_repository(repo_path):
    # Smart discovery - only analyze relevant files
    structure = discover_repository_structure(repo_path)
    
    # Limit file size and count for efficiency
    results = {}
    for category, files in structure.items():
        if files:
            # Limit to reasonable number of files
            relevant_files = files[:5]  # Max 5 files per category
            content = read_files_smart(relevant_files)  # Limit content size
            
            if content:  # Only if we have content
                prompt = f"Analyze these {category} files for novel practices, innovative approaches, and unexpected patterns. What new or interesting things do you see?"
                results[category] = call_llm_api(content, prompt)
    
    return results

def read_files_smart(files):
    # Limit total content size
    max_size = 10000  # characters
    content = ""
    for file in files:
        if len(content) > max_size:
            break
        file_content = read_file(file)
        content += f"\n--- {file} ---\n{file_content[:2000]}"  # Limit per file
    return content
```

### LLM API Call
```python
def call_llm_api(file_content, prompt):
    response = openai.chat.completions.create(
        model="gpt-4",
        messages=[
            {"role": "system", "content": "You are a code analysis tool. Answer only yes/no."},
            {"role": "user", "content": f"{prompt}\n\nFile content:\n{file_content}"}
        ],
        temperature=0
    )
    return response.choices[0].message.content.strip().lower() == "yes"
```

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
