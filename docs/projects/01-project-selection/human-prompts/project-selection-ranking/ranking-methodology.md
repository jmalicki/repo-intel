# Ranking Methodology

**Parent:** [Project Selection and Ranking](README.md)

## Overview

This prompt guides the ranking of projects based on quality assessment and diversity requirements. It provides a structured approach for combining quantitative metrics with qualitative judgment.

## Ranking Framework

### 1. Quantitative Scoring (40% weight)

**Automated Metrics for {{ project_name }}:**
- [ ] Star count and growth rate: {{ automated_metrics.stars }}
- [ ] Fork count and engagement: {{ automated_metrics.forks }}
- [ ] Commit frequency and consistency: {{ automated_metrics.commits }}
- [ ] Issue resolution time: {{ automated_metrics.issue_resolution }}
- [ ] PR merge rate: {{ automated_metrics.pr_merge_rate }}
- [ ] Documentation completeness: {{ automated_metrics.documentation }}
- [ ] Security file presence: {{ automated_metrics.security }}
- [ ] CI/CD pipeline sophistication: {{ automated_metrics.cicd }}

**Scoring Process:**
- Normalize metrics across categories
- Weight by category importance
- Apply statistical analysis
- Generate quantitative baseline

### 2. Qualitative Assessment (40% weight)

**Quality Indicators:**
- [ ] Code organization and structure
- [ ] Documentation quality
- [ ] Testing practices
- [ ] Security awareness
- [ ] Performance optimization
- [ ] Community health
- [ ] Innovation and best practices

**Assessment Process:**
- Apply human judgment criteria
- Consider context and nuance
- Balance different quality aspects
- Account for project-specific factors

### 3. Diversity Requirements (20% weight)

**Scale Representation:**
- [ ] Small projects (personal/side projects)
- [ ] Medium projects (team projects)
- [ ] Large projects (enterprise/community)

**Approach Diversity:**
- [ ] Different architectural patterns
- [ ] Various technology stacks
- [ ] Different organizational models
- [ ] Various deployment strategies

**Community Diversity:**
- [ ] Different community sizes
- [ ] Various governance models
- [ ] Different contribution models
- [ ] Various communication styles

## Ranking Process

### Step 1: Quantitative Baseline
- Calculate automated metric scores
- Normalize across categories
- Apply statistical weighting
- Generate quantitative rankings

### Step 2: Qualitative Adjustment
- Apply human judgment scores
- Consider quality nuances
- Balance different aspects
- Adjust for context

### Step 3: Diversity Balancing
- Ensure scale representation
- Maintain approach diversity
- Balance technology stacks
- Consider community models

### Step 4: Final Ranking
- Combine all scoring elements
- Apply final weighting
- Generate final rankings
- Document ranking rationale

## Scoring Criteria

### Excellent (9-10)
- Outstanding quality across all dimensions
- Innovative approaches and best practices
- Strong community health and engagement
- Clear leadership in category

### Good (7-8)
- High quality with minor areas for improvement
- Solid practices and good community health
- Strong technical implementation
- Good category representation

### Satisfactory (5-6)
- Adequate quality with some areas for improvement
- Basic practices and community engagement
- Functional implementation
- Acceptable category representation

### Needs Improvement (3-4)
- Quality issues that need attention
- Limited community engagement
- Implementation concerns
- Weak category representation

### Poor (1-2)
- Significant quality issues
- Minimal community engagement
- Implementation problems
- Poor category representation

## Documentation Requirements

### Ranking Summary
- Final rankings with scores
- Quantitative vs. qualitative breakdown
- Diversity contribution analysis
- Category representation assessment

### Rationale for {{ project_name }}
- Ranking methodology for {{ project_category }} projects
- Scoring decisions and rationale for {{ project_name }}
- Diversity balancing decisions for {{ project_category }}
- Final ranking justification for {{ project_name }}

### Recommendations for {{ project_name }}
- Areas for improvement in {{ project_name }}
- Best practices identified in {{ project_name }}
- Innovation examples from {{ project_name }}
- Category insights for {{ project_category }}

### Output Report
- Save ranking to: {{ output_report }}
- Ranking date: {{ assessment_date }}
- Reviewer: {{ reviewer_name }}
- Project: {{ project_name }}
- Category: {{ project_category }}
- Scale: {{ project_scale }}
- URL: {{ project_url }}

This prompt ensures consistent, high-quality ranking while maintaining the balance between quality standards and diversity requirements.
