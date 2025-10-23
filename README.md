# Repo Intelligence Project

This project analyzes how different types of software projects organize, structure, and maintain their codebases to create high-quality template repositories for GitHub.

## Project Structure

```
repo-intel/
├── docs/                   # Project documentation and design
│   ├── PROJECT_DESIGN.md   # Main project design document
│   └── phases/             # Project phases
│       ├── 01-project-selection/    # Phase 1: Project Selection
│       ├── 02-analysis/            # Phase 2: Repository Analysis
│       ├── 03-pattern-synthesis/   # Phase 3: Pattern Synthesis
│       ├── 04-template-generation/ # Phase 4: Template Generation
│       └── 05-guidelines-documentation/ # Phase 5: Guidelines & Documentation
└── projects/               # Tool designs and implementations
    └── 01-project-selection/ # Phase 1 tool designs
```

## Documentation Navigation

This project uses a **hierarchical navigation system** to maintain clear relationships between documents:

- **Parent Links**: Each document links to its parent document
- **Related Links**: Documents link to related documents with context
- **Bidirectional Navigation**: Easy movement between high-level and detailed views
- **Phase-Based Organization**: Clear separation between project phases and tool designs

**Navigation Principles:**
- Keep navigation clean and focused
- Link to parent documents at the top
- Add related documents in context
- Maintain clear document hierarchy

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

The project selection methodology is documented in [docs/phases/01-project-selection/PROJECT_SELECTION_METHODOLOGY.md](docs/phases/01-project-selection/PROJECT_SELECTION_METHODOLOGY.md).

## Tool Designs

Detailed tool specifications are available in [docs/projects/01-project-selection/](docs/projects/01-project-selection/).

## Development Guidelines

All development follows the [Development Guidelines](docs/DEVELOPMENT_GUIDELINES.md):

- **Template Variables**: All human prompts use Jinja-like templating with standardized variables
- **Documentation Standards**: Clear navigation and self-contained documents
- **Quality Assurance**: Comprehensive quality procedures throughout
- **Development Practices**: Consistent file organization and standards

## Contributing

This project follows a systematic approach to repository analysis. Each phase builds upon the previous one to create comprehensive template repositories and best practice guidelines.
