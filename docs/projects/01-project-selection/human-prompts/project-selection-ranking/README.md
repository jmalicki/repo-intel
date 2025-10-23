# Project Selection and Ranking

**Parent:** [Human Prompts](../README.md)
**Related:** [Automation Opportunities](../../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) - Project Selection and Ranking task

## Overview

This directory contains discrete prompts for the final project selection and ranking decisions. Each prompt focuses on a specific aspect of the selection process, providing structured guidance for human reviewers.

## Template Variables

All prompts use Jinja-like templating with the following variables:

### Project Variables
- `{{ project_name }}` - Name of the project being evaluated
- `{{ project_url }}` - GitHub URL of the project
- `{{ project_category }}` - Category of the project (Chrome Extension, MCP Server, etc.)
- `{{ project_scale }}` - Scale of the project (Small, Medium, Large)

### Data Variables
- `{{ quality_scores }}` - Quality assessment scores for the project
- `{{ diversity_metrics }}` - Diversity metrics for the project
- `{{ automated_metrics }}` - Automated metrics collected for the project
- `{{ community_health }}` - Community health metrics for the project

### Output Variables
- `{{ output_report }}` - Path to the output report file
- `{{ assessment_date }}` - Date of the assessment
- `{{ reviewer_name }}` - Name of the reviewer
- `{{ review_team }}` - Review team information

### Template Usage
- Use `{{ variable_name }}` for single values
- Use `{% for item in list %}{{ item }}{% endfor %}` for lists
- Use `{% if condition %}content{% endif %}` for conditional content
- All variables will be populated by the templating system before use

## Discrete Prompts

### Core Assessment Prompts
### 1. [Quality Assessment](quality-assessment.md)
**Purpose:** Assess project quality across multiple dimensions
**Focus:** Code quality, documentation, testing, security, performance, community health
**Output:** Quality scores and assessment rationale

### 2. [Ranking Methodology](ranking-methodology.md)
**Purpose:** Rank projects based on quality and diversity requirements
**Focus:** Quantitative scoring, qualitative assessment, diversity balancing
**Output:** Final project rankings with rationale

### 3. [Selection Criteria](selection-criteria.md)
**Purpose:** Define criteria for project selection decisions
**Focus:** Must-have requirements, quality standards, excellence indicators
**Output:** Selection decisions and rationale

### Process Support Prompts
### 4. [Pre-Assessment Checklist](../pre-assessment-checklist.md)
**Purpose:** Ensure all necessary data and preparations are complete
**Focus:** Data collection, team preparation, process readiness
**Output:** Readiness validation and go/no-go decision

### 5. [Quality Assurance](../quality-assurance.md)
**Purpose:** Ensure consistent, high-quality decisions throughout the process
**Focus:** Cross-validation, documentation quality, process consistency
**Output:** Quality assurance procedures and validation

### 6. [Documentation Requirements](../documentation-requirements.md)
**Purpose:** Define comprehensive documentation requirements
**Focus:** Assessment documentation, ranking documentation, selection documentation
**Output:** Documentation standards and deliverables

## Usage Workflow

### Step 1: Quality Assessment
- Use [quality-assessment.md](quality-assessment.md) to evaluate project quality
- Apply structured criteria across all quality dimensions
- Document quality scores and assessment rationale

### Step 2: Ranking Methodology
- Use [ranking-methodology.md](ranking-methodology.md) to rank projects
- Combine quantitative metrics with qualitative assessment
- Apply diversity requirements and balancing

### Step 3: Selection Criteria
- Use [selection-criteria.md](selection-criteria.md) to make final selections
- Apply must-have, should-have, and nice-to-have criteria
- Document selection rationale and decisions

## Expected Outcomes

- **Consistent Quality Assessment**: Standardized evaluation across all projects
- **Informed Rankings**: Well-reasoned project rankings based on quality and diversity
- **Clear Selection Decisions**: Transparent criteria and rationale for selections
- **Comprehensive Documentation**: Complete rationale for all decisions

## Quality Assurance

Use the [Quality Assurance](quality-assurance.md) prompt to ensure consistent, high-quality decisions throughout the process.

This approach ensures consistent, high-quality project selection while maintaining the diversity and quality standards required for comprehensive analysis.
