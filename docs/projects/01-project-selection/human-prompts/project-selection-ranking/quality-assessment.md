# Quality Assessment

**Parent:** [Project Selection and Ranking](README.md)

## Overview

This prompt guides the assessment of project quality across multiple dimensions. It provides structured criteria for evaluating code quality, documentation, testing, and community health.

## Pre-Assessment Checklist

- [ ] Automated metrics have been collected for {{ project_name }}
- [ ] Quantitative scores are available for {{ project_name }}
- [ ] Project documentation has been reviewed for {{ project_name }}
- [ ] Community metrics are available for {{ project_name }}
- [ ] Quality assessment data is ready for {{ project_name }}
- [ ] Review team is prepared for {{ project_name }} assessment

## Quality Dimensions

### 1. Code Quality (25% weight)

**Architecture Assessment:**
- [ ] Clear separation of concerns
- [ ] Appropriate abstraction levels
- [ ] Consistent design patterns
- [ ] Scalable architecture

**Code Organization:**
- [ ] Logical directory structure
- [ ] Clear module boundaries
- [ ] Consistent naming conventions
- [ ] Appropriate file organization

**Code Quality:**
- [ ] Code readability and clarity
- [ ] Appropriate complexity levels
- [ ] Error handling practices
- [ ] Documentation within code

### 2. Documentation Quality (20% weight)

**Completeness:**
- [ ] Comprehensive README
- [ ] API documentation
- [ ] Contributing guidelines
- [ ] Examples and tutorials

**Quality:**
- [ ] Clear and accurate content
- [ ] Up-to-date information
- [ ] Appropriate detail level
- [ ] User-friendly presentation

**Maintenance:**
- [ ] Regular updates
- [ ] Version consistency
- [ ] Community contributions
- [ ] Feedback integration

### 3. Testing Practices (20% weight)

**Test Coverage:**
- [ ] Unit test presence
- [ ] Integration test coverage
- [ ] End-to-end testing
- [ ] Test quality and relevance

**Testing Strategy:**
- [ ] Testing approach clarity
- [ ] Test organization
- [ ] CI/CD integration
- [ ] Test maintenance

**Quality Assurance:**
- [ ] Test automation
- [ ] Quality gates
- [ ] Performance testing
- [ ] Security testing

### 4. Security Practices (15% weight)

**Security Awareness:**
- [ ] Security documentation
- [ ] Vulnerability handling
- [ ] Dependency management
- [ ] Security updates

**Implementation:**
- [ ] Secure coding practices
- [ ] Input validation
- [ ] Authentication/authorization
- [ ] Data protection

**Monitoring:**
- [ ] Security scanning
- [ ] Dependency checking
- [ ] Vulnerability reporting
- [ ] Security updates

### 5. Performance Practices (10% weight)

**Optimization:**
- [ ] Performance considerations
- [ ] Resource usage
- [ ] Scalability planning
- [ ] Monitoring setup

**Documentation:**
- [ ] Performance characteristics
- [ ] Optimization guides
- [ ] Benchmarking results
- [ ] Resource requirements

### 6. Community Health (10% weight)

**Maintainer Quality:**
- [ ] Responsiveness
- [ ] Communication clarity
- [ ] Decision transparency
- [ ] Long-term commitment

**Community Engagement:**
- [ ] Active discussions
- [ ] Helpful responses
- [ ] Inclusive practices
- [ ] Code of conduct

## Scoring Methodology

### Quantitative Review
- Review automated metrics
- Validate quantitative scores
- Identify metric outliers
- Cross-reference with qualitative assessment

### Qualitative Assessment
- Apply human judgment criteria
- Consider context and nuance
- Balance different quality aspects
- Account for project-specific factors

### Final Score
- Combine quantitative and qualitative scores
- Weight by importance (see percentages above)
- Document reasoning for scores
- Identify areas of excellence and improvement

## Documentation Requirements

### Quality Assessment Summary for {{ project_name }}
- Overall quality score: {{ quality_scores.overall }}
- Strengths: {{ quality_scores.strengths }}
- Weaknesses: {{ quality_scores.weaknesses }}
- Areas of excellence: {{ excellence_examples }}
- Improvement opportunities: {{ improvement_recommendations }}

### Rationale
- Quality assessment reasoning for {{ project_name }}
- Context considerations for {{ project_category }} projects
- Score justification based on {{ quality_standards }}
- Recommendations for improvement

### Output Report
- Save assessment to: {{ output_report }}
- Assessment date: {{ assessment_date }}
- Reviewer: {{ reviewer_name }}
- Review team: {{ review_team }}

## Quality Assurance

### Cross-Validation
- [ ] Multiple reviewers for borderline cases
- [ ] Cross-reference with automated metrics
- [ ] Validate against quality criteria
- [ ] Ensure consistency across assessments

### Documentation Review
- [ ] Quality assessment is documented
- [ ] Scoring rationale is clear
- [ ] Recommendations are actionable
- [ ] Assessment is well-supported

This prompt ensures consistent, high-quality assessment while maintaining the balance between quantitative metrics and qualitative judgment.
