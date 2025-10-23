# LLM Orchestrator

**Parent:** [Projects Overview](../README.md)
**Related:** [Automation Opportunities](../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) - Human judgment task orchestration

## ⚠️ **NOT READY FOR IMPLEMENTATION** ⚠️

**🚨 CRITICAL DECISION PENDING 🚨**

This component requires **significant strategic decisions** before implementation can begin:

- **LLM Provider Strategy**: Pure Rust vs. PyO3 + Python vs. Pure Python vs. Microservice approach
- **Library Integration**: Which LLM abstraction libraries to use (LiteLLM, OpenRouter, custom implementation)
- **Performance Requirements**: How critical is performance vs. development speed
- **Ecosystem Access**: Importance of Python LLM ecosystem vs. native Rust performance
- **Deployment Strategy**: Single binary vs. multiple services vs. Python environment

**See [LLM Abstraction Strategy](LLM_ABSTRACTION_STRATEGY.md) for detailed analysis and decision framework.**

**DO NOT START IMPLEMENTATION** until these strategic decisions are made and documented.

## Overview

The LLM Orchestrator is a **hybrid Python script** that uses LLM API calls to execute human judgment tasks by templating and chaining prompts from the human prompts directory. It automates the execution of human judgment workflows while maintaining the quality and structure of human decision-making.

## Purpose

- **Execute human judgment tasks** using structured prompts and LLM API calls
- **Template prompts** with project-specific data and variables
- **Chain prompt execution** in logical sequences for complete workflows
- **Generate structured outputs** following the format specified in prompts
- **Maintain decision quality** through comprehensive prompt guidance

## Design Principles

1. **Prompt-Driven** - All logic driven by structured prompts
2. **Template-Based** - Dynamic content generation using Jinja templating
3. **Chain Execution** - Sequential prompt execution with data flow
4. **Quality Preservation** - Maintain human-level decision quality
5. **Audit Trail** - Complete documentation of decisions and rationale

## Dependencies

This tool uses the **[Common Library](../common-library/README.md)** for:
- **HTTP Client Library** - LLM API calls, rate limiting, retry logic
- **Storage Library** - File I/O, database operations, serialization
- **Logging Library** - Structured logging, performance metrics
- **Configuration Library** - Settings management, environment variables

## LLM Provider Strategy

**Strategy Document**: See [LLM Abstraction Strategy](LLM_ABSTRACTION_STRATEGY.md) for detailed analysis of LLM provider integration options, trade-offs, and recommended implementation approach.

**Quick Summary**:
- **Custom Rust Implementation**: Core orchestration with trait-based abstraction
- **OpenRouter Integration**: Unified access to 100+ models with cost optimization
- **LiteLLM Microservice**: Optional advanced features and extended provider support
- **Hybrid Approach**: Combine custom implementation with existing libraries

## Human Prompt Integration

### Prompt Sources
- **[Human Prompts](../../data/prompts/01-project-selection/)** - All human judgment task prompts
- **Template Variables** - Jinja-like templating with project data
- **Output Specifications** - Structured report generation
- **Quality Assurance** - Built-in validation and cross-checking

### Supported Prompt Categories

#### 1. Project Selection and Ranking
- **[Quality Assessment](../../data/prompts/01-project-selection/project-selection-ranking/quality-assessment.md)** - Multi-dimensional quality evaluation
- **[Ranking Methodology](../../data/prompts/01-project-selection/project-selection-ranking/ranking-methodology.md)** - Quantitative and qualitative ranking
- **[Selection Criteria](../../data/prompts/01-project-selection/project-selection-ranking/selection-criteria.md)** - Final selection decisions

#### 2. Category Representation
- **[Scale Diversity](../../data/prompts/01-project-selection/category-representation/scale-diversity.md)** - Scale representation across categories
- **[Approach Diversity](../../data/prompts/01-project-selection/category-representation/approach-diversity.md)** - Approach representation across categories
- **[Community Diversity](../../data/prompts/01-project-selection/category-representation/community-diversity.md)** - Community representation across categories

#### 3. Final Quality Assessment
- **[Quality Standards](../../data/prompts/01-project-selection/final-quality-assessment/quality-standards.md)** - Quality threshold application
- **[Excellence Identification](../../data/prompts/01-project-selection/final-quality-assessment/excellence-identification.md)** - Excellence example identification
- **[Final Validation](../../data/prompts/01-project-selection/final-quality-assessment/final-validation.md)** - Final validation and handoff

#### 4. Process Support Prompts
- **[Pre-Assessment Checklist](../../data/prompts/01-project-selection/pre-assessment-checklist.md)** - Preparation validation
- **[Quality Assurance](../../data/prompts/01-project-selection/quality-assurance.md)** - Decision quality assurance
- **[Documentation Requirements](../../data/prompts/01-project-selection/documentation-requirements.md)** - Documentation standards

## Orchestration Workflows

### 1. Project Selection Workflow
**Purpose:** Complete project selection and ranking process

**Sequence:**
1. **Pre-Assessment** - Run pre-assessment checklist
2. **Quality Assessment** - Evaluate project quality across dimensions
3. **Ranking Methodology** - Rank projects by quality and diversity
4. **Selection Criteria** - Apply final selection criteria
5. **Quality Assurance** - Cross-validate decisions
6. **Documentation** - Generate comprehensive reports

**Input:** Project data from automated tools
**Output:** Ranked project shortlist with rationale

### 2. Category Representation Workflow
**Purpose:** Ensure appropriate category coverage and diversity

**Sequence:**
1. **Pre-Assessment** - Validate data completeness
2. **Scale Diversity** - Assess scale representation
3. **Approach Diversity** - Assess approach representation
4. **Community Diversity** - Assess community representation
5. **Quality Assurance** - Validate diversity decisions
6. **Documentation** - Document diversity rationale

**Input:** Project shortlist and category requirements
**Output:** Diversity assessment and recommendations

### 3. Final Quality Assessment Workflow
**Purpose:** Final quality decisions and threshold application

**Sequence:**
1. **Pre-Assessment** - Validate all inputs
2. **Quality Standards** - Apply quality thresholds
3. **Excellence Identification** - Identify excellence examples
4. **Final Validation** - Final validation and handoff
5. **Quality Assurance** - Cross-validate all decisions
6. **Documentation** - Generate final documentation

**Input:** All project data and previous assessments
**Output:** Final project selection with quality rationale

## Template System

### Jinja Template Variables
All prompts use standardized template variables:

#### Project Variables
- `{{ project_name }}` - Project name
- `{{ project_url }}` - GitHub URL
- `{{ project_category }}` - Project category
- `{{ project_scale }}` - Project scale (Small/Medium/Large)

#### Data Variables
- `{{ quality_scores }}` - Quality assessment scores
- `{{ automated_metrics }}` - Automated metrics data
- `{{ community_health }}` - Community health metrics
- `{{ diversity_metrics }}` - Diversity metrics

#### Output Variables
- `{{ output_report }}` - Output file path
- `{{ assessment_date }}` - Assessment date
- `{{ reviewer_name }}` - Reviewer name
- `{{ review_team }}` - Review team

### Template Processing
```python
# Example template processing
template_vars = {
    "project_name": "rust-analyzer",
    "project_url": "https://github.com/rust-lang/rust-analyzer",
    "project_category": "Rust Libraries",
    "quality_scores": {"code_quality": 8.5, "documentation": 9.2},
    "output_report": "reports/rust-analyzer-assessment.md"
}

# Template the prompt
templated_prompt = template_prompt(prompt_content, template_vars)
```

## LLM Integration

### API Configuration
- **OpenAI GPT-4** - Primary LLM for complex reasoning
- **Anthropic Claude** - Alternative for different reasoning styles
- **Rate Limiting** - Respect API rate limits and quotas
- **Retry Logic** - Handle API failures gracefully
- **Cost Optimization** - Minimize token usage while maintaining quality

### Prompt Engineering
- **System Prompts** - Role definition and context setting
- **User Prompts** - Templated prompt content with project data
- **Output Formatting** - Structured JSON and markdown outputs
- **Quality Validation** - Built-in output validation and correction

### Response Processing
- **JSON Parsing** - Extract structured data from responses
- **Validation** - Validate response format and content
- **Error Handling** - Handle malformed or incomplete responses
- **Retry Logic** - Retry failed or invalid responses

## Workflow Execution

### 1. Data Preparation
```python
# Load project data from automated tools
project_data = load_project_data("data/collected/")
template_vars = prepare_template_variables(project_data)
```

### 2. Prompt Execution
```python
# Execute prompt sequence
for prompt_file in prompt_sequence:
    prompt_content = load_prompt(prompt_file)
    templated_prompt = template_prompt(prompt_content, template_vars)
    response = call_llm_api(templated_prompt)
    results = process_response(response)
    template_vars.update(results)
```

### 3. Output Generation
```python
# Generate final outputs
generate_reports(template_vars)
validate_outputs(template_vars)
document_decisions(template_vars)
```

## Quality Assurance

### Built-in Validation
- **Prompt Completeness** - Ensure all required prompts are executed
- **Data Consistency** - Validate data flow between prompts
- **Output Quality** - Validate output format and content
- **Decision Rationale** - Ensure all decisions are well-documented

### Cross-Validation
- **Multiple LLM Calls** - Use different LLMs for validation
- **Consensus Building** - Compare results across different approaches
- **Human Review** - Flag decisions for human review when needed
- **Audit Trail** - Complete documentation of all decisions

## Configuration

### LLM Settings
```yaml
llm:
  primary_provider: "openai"
  model: "gpt-4"
  temperature: 0.1
  max_tokens: 4000
  timeout: 60

  secondary_provider: "anthropic"
  model: "claude-3"
  temperature: 0.1
  max_tokens: 4000
```

### Workflow Settings
```yaml
workflows:
  project_selection:
    enabled: true
    prompts: ["pre-assessment", "quality-assessment", "ranking", "selection"]
    output_format: "markdown"

  category_representation:
    enabled: true
    prompts: ["scale-diversity", "approach-diversity", "community-diversity"]
    output_format: "markdown"
```

## Output Formats

### Individual Project Reports
```markdown
# Project Assessment Report: {{ project_name }}

## Quality Assessment
- Code Quality: {{ quality_scores.code_quality }}/10
- Documentation: {{ quality_scores.documentation }}/10
- Testing: {{ quality_scores.testing }}/10

## Selection Rationale
{{ selection_rationale }}

## Recommendations
{{ recommendations }}
```

### Category Summary Reports
```markdown
# Category Representation Report: {{ category_name }}

## Scale Diversity
{{ scale_diversity_analysis }}

## Approach Diversity
{{ approach_diversity_analysis }}

## Community Diversity
{{ community_diversity_analysis }}
```

### Final Selection Report
```markdown
# Final Project Selection Report

## Selected Projects
{{ selected_projects }}

## Quality Standards Applied
{{ quality_standards }}

## Diversity Achieved
{{ diversity_analysis }}

## Handoff to Phase 2
{{ handoff_documentation }}
```

## Integration with Other Tools

### Input Sources
- **GitHub API Collector** - Repository metadata and metrics
- **Package Manager Collector** - Package ecosystem data
- **Trending Analyzer** - Trending patterns and growth
- **Metrics Calculator** - Calculated scores and rankings
- **Repository Data Aggregator** - Unified project datasets

### Output Destinations
- **Report Generator** - Final report formatting
- **Quality Filter** - Quality threshold application
- **Human Review** - Flagged decisions for human review
- **Phase 2 Handoff** - Complete project selection data

## Performance Optimization

### Token Management
- **Prompt Optimization** - Minimize token usage while maintaining quality
- **Response Caching** - Cache similar responses to reduce API calls
- **Batch Processing** - Process multiple projects efficiently
- **Cost Tracking** - Monitor and optimize API costs

### Execution Efficiency
- **Parallel Processing** - Execute independent prompts in parallel
- **Result Caching** - Cache intermediate results for reuse
- **Incremental Updates** - Update only changed data
- **Progress Tracking** - Monitor execution progress and performance

This LLM Orchestrator provides a comprehensive solution for executing human judgment tasks using structured prompts and LLM API calls, ensuring high-quality decisions while maintaining efficiency and auditability.
