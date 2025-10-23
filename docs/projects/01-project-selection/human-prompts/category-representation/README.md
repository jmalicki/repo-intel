# Category Representation

**Parent:** [Human Prompts](../README.md)
**Related:** [Automation Opportunities](../../../phases/01-project-selection/AUTOMATION_OPPORTUNITIES.md) - Category Representation task

## Overview

This directory contains discrete prompts for ensuring appropriate category coverage and diversity. Each prompt focuses on a specific aspect of representation assessment.

## Template Variables

All prompts use Jinja-like templating with the following variables:

### Category Variables
- `{{ category_name }}` - Name of the category being assessed
- `{{ category_projects }}` - List of projects in the category
- `{{ category_requirements }}` - Specific requirements for the category
- `{{ category_diversity_goals }}` - Diversity goals for the category

### Project Variables
- `{{ project_name }}` - Name of the project being evaluated
- `{{ project_url }}` - GitHub URL of the project
- `{{ project_scale }}` - Scale of the project (Small, Medium, Large)
- `{{ project_approach }}` - Approach/architecture of the project
- `{{ project_community }}` - Community characteristics of the project

### Diversity Variables
- `{{ scale_distribution }}` - Distribution of project scales
- `{{ approach_distribution }}` - Distribution of project approaches
- `{{ community_distribution }}` - Distribution of community types
- `{{ diversity_gaps }}` - Identified diversity gaps

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
### 1. [Scale Diversity](scale-diversity.md)
**Purpose:** Assess scale diversity across project categories
**Focus:** Small, medium, and large project representation
**Output:** Scale distribution analysis and recommendations

### 2. [Approach Diversity](approach-diversity.md)
**Purpose:** Assess approach diversity across project categories
**Focus:** Architectural patterns, technology stacks, organizational models
**Output:** Approach diversity analysis and recommendations

### 3. [Community Diversity](community-diversity.md)
**Purpose:** Assess community diversity across project categories
**Focus:** Community sizes, governance models, contribution patterns
**Output:** Community diversity analysis and recommendations

### Process Support Prompts
### 4. [Pre-Assessment Checklist](pre-assessment-checklist.md)
**Purpose:** Ensure all necessary data and preparations are complete
**Focus:** Data collection, team preparation, process readiness
**Output:** Readiness validation and go/no-go decision

### 5. [Quality Assurance](quality-assurance.md)
**Purpose:** Ensure consistent, high-quality decisions throughout the process
**Focus:** Cross-validation, documentation quality, process consistency
**Output:** Quality assurance procedures and validation

### 6. [Documentation Requirements](documentation-requirements.md)
**Purpose:** Define comprehensive documentation requirements
**Focus:** Diversity documentation, representation documentation, category documentation
**Output:** Documentation standards and deliverables

## Usage Workflow

### Step 1: Scale Diversity Assessment
- Use [scale-diversity.md](scale-diversity.md) to assess project scale representation
- Ensure small, medium, and large projects are represented
- Document scale distribution and gaps

### Step 2: Approach Diversity Assessment
- Use [approach-diversity.md](approach-diversity.md) to assess approach representation
- Ensure architectural, technology, and organizational diversity
- Document approach distribution and gaps

### Step 3: Community Diversity Assessment
- Use [community-diversity.md](community-diversity.md) to assess community representation
- Ensure community size, governance, and contribution diversity
- Document community distribution and gaps

## Expected Outcomes

- **Comprehensive Scale Representation**: Appropriate distribution of small, medium, and large projects
- **Diverse Approaches**: Representation of different architectural patterns and technology stacks
- **Community Diversity**: Representation of different community sizes and governance models
- **Quality Balance**: High-quality projects across all diversity dimensions

## Quality Assurance

Use the [Quality Assurance](quality-assurance.md) prompt to ensure consistent, high-quality decisions throughout the process.

This approach ensures comprehensive category representation while maintaining quality standards and enabling meaningful cross-project analysis.
