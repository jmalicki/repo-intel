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

## Model Implementation: Common Library

### Documentation Standards Model
The [Common Library](../projects/01-project-selection/common-library/) serves as the **gold standard** for documentation and implementation practices:

#### Design Documentation
- **[Common Library Design](../projects/01-project-selection/common-library/DESIGN.md)** - API-focused architecture documentation
- **No Implementation Code** - Pure design and API descriptions
- **Component Purposes** - Clear descriptions of what each component provides
- **API Surface Definitions** - Detailed method and function specifications

#### Implementation Planning
- **[Common Library Implementation Plan](../projects/01-project-selection/common-library/IMPLEMENTATION_PLAN.md)** - Step-by-step development workflow
- **Phased Development** - 6 focused phases with specific deliverables
- **Stacked Branches** - Each phase builds on the previous
- **Quality Standards** - Comprehensive testing, benchmarking, and review processes

### Implementation Standards Model

#### Project Structure
```
common-library/
├── README.md              # Overview and navigation
├── DESIGN.md              # API-focused architecture
├── IMPLEMENTATION_PLAN.md # Step-by-step development
└── src/                   # Implementation code
```

#### Documentation Hierarchy
- **README.md** - Project overview and navigation
- **DESIGN.md** - Architecture and API specifications
- **IMPLEMENTATION_PLAN.md** - Development workflow and standards

#### Quality Standards
- **API-Focused Design** - No implementation code in design docs
- **Comprehensive Testing** - Unit, integration, performance, mock tests
- **Benchmarking** - Performance measurement and optimization
- **Code Quality** - Formatting, linting, documentation standards

### Using Common Library as a Model

#### For New Projects
1. **Follow the Structure** - Use the same documentation hierarchy
2. **API-Focused Design** - Keep design docs focused on architecture
3. **Implementation Planning** - Create detailed step-by-step plans
4. **Quality Standards** - Apply the same testing and review processes

#### For Documentation Updates
1. **Maintain API Focus** - Keep design docs architectural
2. **Update Implementation Plans** - Keep development workflows current
3. **Follow Quality Standards** - Apply consistent testing and review
4. **Link to Model** - Reference Common Library as the standard

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

### Model Compliance
- [ ] Follow Common Library documentation structure
- [ ] Use API-focused design approach
- [ ] Implement comprehensive testing standards
- [ ] Apply quality review processes

This document ensures consistent development practices while maintaining high standards for templating, documentation, and quality assurance. The Common Library serves as the model implementation for all project documentation and development standards.
