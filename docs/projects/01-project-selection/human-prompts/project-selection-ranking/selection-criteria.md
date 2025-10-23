# Selection Criteria

**Parent:** [Project Selection and Ranking](README.md)

## Overview

This prompt defines the criteria for project selection, including must-have requirements, quality standards, and excellence indicators. It provides clear guidelines for making selection decisions.

## Selection Criteria

### Must Have (Elimination Criteria)

**Basic Requirements for {{ project_name }}:**
- [ ] Active development (commits in last 6 months): {{ automated_metrics.commits }}
- [ ] Basic documentation (README, basic docs): {{ automated_metrics.documentation }}
- [ ] Community presence (issues, discussions): {{ community_health.engagement }}
- [ ] License clarity: {{ automated_metrics.license }}
- [ ] Security awareness (basic practices): {{ automated_metrics.security }}

**Quality Thresholds:**
- [ ] Minimum quality score (5/10)
- [ ] Basic community engagement
- [ ] Functional implementation
- [ ] Acceptable documentation

### Should Have (Quality Indicators)

**Documentation:**
- [ ] Comprehensive README
- [ ] API documentation
- [ ] Contributing guidelines
- [ ] Examples and tutorials

**Development Practices:**
- [ ] Testing practices
- [ ] CI/CD pipeline
- [ ] Code quality standards
- [ ] Security practices

**Community:**
- [ ] Active maintainers
- [ ] Clear communication
- [ ] Inclusive practices
- [ ] Code of conduct

### Nice to Have (Excellence Indicators)

**Advanced Practices:**
- [ ] Performance optimization
- [ ] Accessibility compliance
- [ ] Internationalization
- [ ] Advanced security practices
- [ ] Innovative approaches

**Community Excellence:**
- [ ] Strong community engagement
- [ ] Excellent documentation
- [ ] Best practice examples
- [ ] Industry leadership

## Selection Process

### Step 1: Initial Filtering
- Apply must-have criteria
- Eliminate projects that don't meet basic requirements
- Generate initial candidate list
- Document elimination rationale

### Step 2: Quality Assessment
- Apply should-have criteria
- Assess quality indicators
- Identify excellence examples
- Generate quality rankings

### Step 3: Diversity Balancing
- Ensure scale representation
- Maintain approach diversity
- Balance technology stacks
- Consider community models

### Step 4: Final Selection
- Apply nice-to-have criteria
- Make final selection decisions
- Document selection rationale
- Generate final shortlist

## Category-Specific Criteria

### Chrome Extensions
- [ ] Chrome Web Store presence
- [ ] Security permission usage
- [ ] User experience quality
- [ ] Chrome API compatibility

### MCP Servers
- [ ] MCP protocol compliance
- [ ] AI model integration
- [ ] Documentation quality
- [ ] Community adoption

### Rust Libraries
- [ ] crates.io presence
- [ ] Performance characteristics
- [ ] Memory safety practices
- [ ] Ecosystem integration

### Full-Stack Systems
- [ ] Architecture complexity
- [ ] Scalability practices
- [ ] Deployment strategies
- [ ] Monitoring setup

### Data Science & ML Projects
- [ ] Reproducibility practices
- [ ] Documentation quality
- [ ] Performance benchmarks
- [ ] Community adoption

### CLI Tools & Applications
- [ ] User experience
- [ ] Cross-platform compatibility
- [ ] Performance characteristics
- [ ] Package manager adoption

### Mobile Applications
- [ ] Platform coverage
- [ ] User experience quality
- [ ] Performance optimization
- [ ] App store compliance

### Documentation Sites
- [ ] Content quality
- [ ] User experience
- [ ] Accessibility compliance
- [ ] Community contribution

## Selection Documentation

### Selection Rationale
- Why each project was chosen
- Key strengths and qualities
- Unique or innovative aspects
- Category representation value

### Quality Assessment
- Specific quality indicators observed
- Areas of excellence
- Potential improvement areas
- Overall quality score

### Diversity Contribution
- How this project contributes to diversity
- Scale representation (small/medium/large)
- Approach representation (architecture/technology)
- Community model representation

## Final Output

### Project Shortlist for {{ project_name }}
- 3-5 projects per category (24-40 total)
- Ranked by quality and diversity
- Clear selection rationale for {{ project_name }}
- Quality assessment summary for {{ project_name }}

### Handoff Documentation
- Selection methodology applied to {{ project_name }}
- Quality thresholds used for {{ project_category }}
- Diversity requirements met for {{ project_category }}
- Ready for Phase 2 analysis

### Output Report
- Save selection to: {{ output_report }}
- Selection date: {{ assessment_date }}
- Reviewer: {{ reviewer_name }}
- Project: {{ project_name }}
- Category: {{ project_category }}
- Scale: {{ project_scale }}
- URL: {{ project_url }}

This prompt ensures consistent, high-quality project selection while maintaining the diversity and quality standards required for comprehensive analysis.
