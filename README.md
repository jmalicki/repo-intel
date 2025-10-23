# Repo Intelligence Project

This project analyzes how different types of software projects organize, structure, and maintain their codebases to create high-quality template repositories for GitHub.

## Project Structure

```
repo-intel/
├── 01-project-selection/    # Phase 1: Project identification and selection
├── 02-analysis/            # Phase 2: Repository analysis and reporting
├── 03-templates/           # Phase 3: Template repository generation
├── 04-guidelines/          # Phase 4: Best practice guidelines
└── docs/                   # Project documentation and design
```

## Target Categories

We analyze 8 main categories of projects:

1. **Chrome Extensions** - Browser extension projects
2. **MCP Servers** - Model Context Protocol server implementations  
3. **Rust Libraries** - Ranging from small utilities to large frameworks
4. **Full-Stack Systems** - Projects with both frontend and backend components
5. **Data Science & ML Projects** - Machine learning libraries and data science tools
6. **CLI Tools & Applications** - Command-line utilities and desktop applications
7. **Mobile Applications** - React Native, Flutter, and native mobile apps
8. **Documentation Sites** - Docusaurus, GitBook, and custom documentation platforms

## Analysis Framework

Each project is analyzed across 11 dimensions:

1. **Pre-commit Setup** - Hook configuration and tool integration
2. **CI/CD Pipeline Structure** - Job organization, triggers, artifacts, tooling
3. **Codebase Organization** - Directory structure, subproject management
4. **Release Management** - Branching strategy, release process, artifact distribution
5. **Documentation Strategy** - README scope, documentation depth, maintenance
6. **Testing Philosophy** - Testing strategy, organization, automation
7. **Contribution Standards** - Commit messages, PR process, bug reports, code review
8. **Security & Compliance** - Security practices, compliance requirements
9. **Performance & Monitoring** - Performance testing, observability, resource management
10. **Internationalization & Accessibility** - i18n strategy, accessibility standards
11. **Dependency & Licensing** - Dependency management, licensing strategy

## Getting Started

See the [Project Design Document](docs/PROJECT_DESIGN.md) for the complete project overview and methodology.

## Phase 1: Project Selection

The project selection methodology is documented in [01-project-selection/PROJECT_SELECTION_METHODOLOGY.md](01-project-selection/PROJECT_SELECTION_METHODOLOGY.md).

## Contributing

This project follows a systematic approach to repository analysis. Each phase builds upon the previous one to create comprehensive template repositories and best practice guidelines.
