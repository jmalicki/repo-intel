# Package Manager Collector

A comprehensive package manager collector that integrates with multiple package registries to collect package metadata, perform health analysis, and resolve data conflicts.

## Features

- **Multi-Registry Support**: Collects data from NPM, PyPI, Crates.io, Maven Central, Go Modules, RubyGems, Packagist, and NuGet
- **Data Conflict Resolution**: Handles conflicting data from multiple sources
- **Package Health Analysis**: Comprehensive health scoring and analysis
- **Rate Limiting**: Respects API rate limits across all registries
- **Async Processing**: High-performance async collection and processing
- **CLI Interface**: Command-line interface for all operations

## Quick Start

```bash
# Build the project
cargo build

# Run with help
cargo run -- --help

# Collect package data
cargo run -- collect --manager npm --packages react lodash

# Analyze package health
cargo run -- analyze --packages react

# Show collection status
cargo run -- status
```

## Configuration

The collector uses a configuration file to specify database connections, API credentials, and collection settings. See the [Design Document](../../../docs/projects/01-project-selection/package-manager-collector/DESIGN.md) for detailed configuration options.

## Development

This project follows the implementation plan outlined in [IMPLEMENTATION_PLAN.md](../../../docs/projects/01-project-selection/package-manager-collector/IMPLEMENTATION_PLAN.md).

### Dependencies

- **Common Library**: This project depends on the common library for HTTP client, database operations, logging, and configuration management.
- **Rust**: Requires Rust 1.70+ for async Diesel support

### Testing

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_package_collection
```

## Architecture

The collector is organized into several key components:

- **Models**: Data structures for packages, versions, metadata, and health metrics
- **Collectors**: Registry-specific collection logic
- **Processors**: Data processing and conflict resolution
- **Storage**: Database operations and data persistence
- **CLI**: Command-line interface

See the [Design Document](../../../docs/projects/01-project-selection/package-manager-collector/DESIGN.md) for detailed architecture information.

## License

MIT License
