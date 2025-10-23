# Phase 1: Project Selection Tools

**Parent:** [Projects Overview](../README.md)
**Related:** [Automation Opportunities](../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) - Project Selection Tools

This directory contains tools for **selecting** high-quality projects for analysis across 8 categories.

## Phase 1 Purpose
**Goal**: Identify and select 3-5 high-quality projects per category (24-40 total) for deep analysis

## What Phase 1 Does
1. **Automated data collection** from GitHub, package managers, and trending APIs
2. **Quantitative analysis** using metrics and scoring algorithms
3. **Human judgment** for final selection and quality assessment
4. **Project shortlisting** with clear selection rationale

## What Phase 1 Does NOT Do
- Deep repository analysis (that's Phase 2)
- Pattern discovery (that's Phase 2)
- Template generation (that's Phase 3)

## Tools for Phase 1

### 1. Data Collection Tools
- **[GitHub API Collector](github-api-collector/README.md)** - Automated GitHub API data collection
- **[GitHub Archive Collector](github-archive-collector/README.md)** - Historical time series data collection
- **[Package Manager Collector](package-manager-collector/README.md)** - NPM, PyPI, crates.io data collection
- **[Trending Analyzer](trending-analyzer/README.md)** - GitHub trending repository analysis

### 2. Analysis Tools
- **[Metrics Calculator](metrics-calculator/README.md)** - Quantitative metrics calculation and scoring
- **[Pattern Matcher](pattern-matcher/README.md)** - File detection and pattern matching
- **[Quality Filter](quality-filter/README.md)** - Automated filtering and threshold application

### 3. Integration Tools
- **[Repository Data Aggregator](repository-data-aggregator/README.md)** - Multi-source data integration
- **[Report Generator](report-generator/README.md)** - Automated report generation
- **[LLM Orchestrator](llm-orchestrator/README.md)** - Human judgment task execution
- **[Common Library](common-library/README.md)** - Shared functionality across all tools

### 4. Human Judgment Tasks
- **[Human Prompts](../../data/prompts/01-project-selection/)** - Structured prompts for human judgment

## LLM Orchestrator Integration

The **[LLM Orchestrator](llm-orchestrator/README.md)** executes human judgment tasks by:

- **Templating Prompts** - Uses Jinja templating with project data
- **Chaining Workflows** - Sequential prompt execution with data flow
- **Quality Preservation** - Maintains human-level decision quality
- **Structured Outputs** - Generates reports following prompt specifications
- **Audit Trail** - Complete documentation of all decisions

## Common Library Benefits

The **[Common Library](common-library/README.md)** eliminates duplication across all tools by providing:

- **HTTP Client Library** - Rate limiting, retry logic, authentication
- **Data Processing Library** - Validation, normalization, transformation
- **Storage Library** - File I/O, database operations, serialization
- **Configuration Library** - Settings management, environment variables
- **Logging Library** - Structured logging, performance metrics
- **Metrics Library** - Statistical calculations, scoring algorithms
- **Validation Library** - Schema validation, business rules

## Output of Phase 1
- **Project shortlist** (3-5 projects per category)
- **Quality assessments** for each selected project
- **Selection rationale** and decision documentation
- **Handoff to Phase 2** with complete project data

## Next Phase
Phase 1 feeds into **Phase 2: Repository Analysis** where we perform deep analysis of selected projects.
