# Development Guidelines

**Parent:** [Project Design](PROJECT_DESIGN.md)

## Overview

This document outlines the development guidelines for the repo intelligence project. It establishes standards for documentation, templating, and development practices.

## Documentation Standards

### Template Variables

All human prompts must use Jinja-like templating with standardized variables:

#### Project Variables
- `{{ project_name }}` - Name of the project being evaluated
- `{{ project_url }}` - GitHub URL of the project
- `{{ project_category }}` - Category of the project (Chrome Extension, MCP Server, etc.)
- `{{ project_scale }}` - Scale of the project (Small, Medium, Large)

#### Data Variables
- `{{ quality_scores }}` - Quality assessment scores for the project
- `{{ diversity_metrics }}` - Diversity metrics for the project
- `{{ automated_metrics }}` - Automated metrics collected for the project
- `{{ community_health }}` - Community health metrics for the project

#### Output Variables
- `{{ output_report }}` - Path to the output report file
- `{{ assessment_date }}` - Date of the assessment
- `{{ reviewer_name }}` - Name of the reviewer
- `{{ review_team }}` - Review team information

### Template Usage Standards

#### Variable Syntax
- Use `{{ variable_name }}` for single values
- Use `{% for item in list %}{{ item }}{% endfor %}` for lists
- Use `{% if condition %}content{% endif %}` for conditional content
- All variables must be documented in the README.md of each prompt directory

#### Template Requirements
- All prompts must be templated with variables
- Variables must be clearly documented
- Template usage must be explained
- Output file paths must be templated

## Human Prompts Standards

### Prompt Structure
- Each prompt must be a discrete, focused task
- Each prompt must have clear purpose and output
- Each prompt must use templating variables
- Each prompt must include documentation requirements

### Prompt Documentation
- README.md must explain template variables
- README.md must provide usage workflow
- README.md must link to all discrete prompts
- Each prompt must be self-contained

### Prompt Quality
- Prompts must be actionable and specific
- Prompts must include quality assurance
- Prompts must include documentation requirements
- Prompts must be maintainable and updateable

## Development Practices

### File Organization
- Use clear, descriptive filenames
- Organize files by phase and function
- Maintain consistent directory structure
- Use README.md files for navigation

### Documentation Standards
- All documents must have parent links
- All documents must have related links
- All documents must be self-contained
- All documents must be maintainable

### Quality Assurance
- All prompts must include quality assurance
- All processes must be documented
- All decisions must be justified
- All outputs must be validated

## Template System Requirements

### Templating Engine
- Use Jinja-like templating syntax
- Support variable substitution
- Support conditional content
- Support list iteration

### Variable Management
- Standardize variable names across all prompts
- Document all variables in README.md files
- Provide variable examples
- Maintain variable consistency

### Output Generation
- Generate templated prompts for specific projects
- Create output report files with templated paths
- Include all necessary variables in output
- Validate template completeness

## Implementation Guidelines

### Prompt Development
1. Create discrete, focused prompts
2. Add templating variables throughout
3. Document variables in README.md
4. Include quality assurance procedures
5. Test template functionality

### Documentation Development
1. Maintain clear navigation links
2. Document all template variables
3. Provide usage workflows
4. Include quality assurance
5. Ensure maintainability

### Quality Assurance
1. Validate template completeness
2. Test variable substitution
3. Verify output generation
4. Ensure documentation quality
5. Maintain consistency

## Standards Compliance

### Template Compliance
- [ ] All prompts use templating variables
- [ ] All variables are documented
- [ ] Template syntax is consistent
- [ ] Output paths are templated

### Documentation Compliance
- [ ] All README.md files document variables
- [ ] All prompts are self-contained
- [ ] All workflows are documented
- [ ] All quality assurance is included

### Development Compliance
- [ ] All files are properly organized
- [ ] All links are maintained
- [ ] All standards are followed
- [ ] All quality is maintained

This document ensures consistent development practices while maintaining high standards for templating, documentation, and quality assurance.
