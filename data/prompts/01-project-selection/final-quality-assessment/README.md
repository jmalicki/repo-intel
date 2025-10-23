# Final Quality Assessment

**Parent:** [Human Prompts](../README.md)
**Related:** [Automation Opportunities](../../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) - Final Quality Assessment task

## Overview

This directory contains discrete prompts for the final quality assessment of all selected projects. Each prompt focuses on a specific aspect of quality evaluation and validation.

## Template Variables

All prompts use Jinja-like templating with the following variables:

### Project Variables
- `{{ project_name }}` - Name of the project being evaluated
- `{{ project_url }}` - GitHub URL of the project
- `{{ project_category }}` - Category of the project
- `{{ project_scale }}` - Scale of the project (Small, Medium, Large)

### Quality Variables
- `{{ quality_scores }}` - Quality assessment scores for the project
- `{{ quality_standards }}` - Quality standards applied to the project
- `{{ excellence_examples }}` - Excellence examples identified
- `{{ quality_thresholds }}` - Quality thresholds for the project

### Assessment Variables
- `{{ assessment_criteria }}` - Assessment criteria used
- `{{ assessment_methodology }}` - Assessment methodology applied
- `{{ assessment_results }}` - Assessment results for the project
- `{{ validation_results }}` - Validation results for the project

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
### 1. [Quality Standards](quality-standards.md)
**Purpose:** Apply quality standards across all selected projects
**Focus:** Code quality, documentation, testing, security, performance, community
**Output:** Quality standards compliance and assessment

### 2. [Excellence Identification](excellence-identification.md)
**Purpose:** Identify excellence examples and best practices
**Focus:** Code excellence, documentation excellence, testing excellence, security excellence, performance excellence, community excellence
**Output:** Excellence examples and best practice documentation

### 3. [Final Validation](final-validation.md)
**Purpose:** Validate final selection and prepare for Phase 2
**Focus:** Quality validation, diversity validation, category validation, selection rationale
**Output:** Final validation report and Phase 2 handoff

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
**Focus:** Quality documentation, excellence documentation, validation documentation
**Output:** Documentation standards and deliverables

## Usage Workflow

### Step 1: Quality Standards Assessment
- Use [quality-standards.md](quality-standards.md) to apply quality standards
- Assess all projects against quality thresholds
- Document quality compliance and gaps

### Step 2: Excellence Identification
- Use [excellence-identification.md](excellence-identification.md) to identify excellence
- Document best practices and innovation examples
- Generate excellence report

### Step 3: Final Validation
- Use [final-validation.md](final-validation.md) to validate final selection
- Ensure all requirements are met
- Prepare for Phase 2 handoff

## Expected Outcomes

- **Quality Standards Compliance**: All projects meet minimum quality thresholds
- **Excellence Examples**: Best practices and innovation examples identified
- **Final Validation**: Comprehensive validation of selection decisions
- **Phase 2 Readiness**: Complete handoff documentation for analysis phase

## Quality Assurance

Use the [Quality Assurance](quality-assurance.md) prompt to ensure consistent, high-quality decisions throughout the process.

This approach ensures comprehensive quality assessment while maintaining diversity requirements and preparing for Phase 2 analysis.
