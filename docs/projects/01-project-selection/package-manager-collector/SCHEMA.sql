-- Package Manager Collector Database Schema
--
-- Parent: [Package Manager Collector Design](DESIGN.md)
-- Related: [API Schemas](API_SCHEMAS.md)
--
-- This schema supports the comprehensive data model defined in DESIGN.md
-- and captures all the rich package health and metadata information
-- needed for project selection analysis.

-- ============================================================================
-- CORE PACKAGE DATA
-- ============================================================================

-- Main packages table
CREATE TABLE packages (
    id INTEGER PRIMARY KEY,
    package_id TEXT UNIQUE NOT NULL,  -- Unique identifier across all registries
    registry TEXT NOT NULL CHECK (registry IN ('npm', 'pypi', 'crates.io')),
    name TEXT NOT NULL,
    description TEXT,
    version TEXT NOT NULL,
    license TEXT,
    homepage TEXT,
    repository_url TEXT,
    documentation_url TEXT,
    author_name TEXT,
    author_email TEXT,
    author_url TEXT,
    keywords TEXT,  -- JSON array of keywords
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    collected_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Indexes for common queries
    INDEX idx_packages_registry (registry),
    INDEX idx_packages_name (name),
    INDEX idx_packages_collected_at (collected_at)
);

-- ============================================================================
-- PACKAGE HEALTH DATA
-- ============================================================================

-- Package health scores (weighted composite)
CREATE TABLE package_health (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,

    -- Maintenance Health (30% weight)
    maintenance_score REAL NOT NULL CHECK (maintenance_score >= 0 AND maintenance_score <= 100),
    update_frequency TEXT,  -- 'daily', 'weekly', 'monthly', 'yearly', 'irregular'
    version_stability_score REAL,  -- Semantic versioning compliance
    breaking_changes_count INTEGER DEFAULT 0,
    maintainer_activity_score REAL,
    documentation_freshness_score REAL,

    -- Security Health (25% weight)
    security_score REAL NOT NULL CHECK (security_score >= 0 AND security_score <= 100),
    vulnerability_count INTEGER DEFAULT 0,
    dependency_security_score REAL,
    security_response_time_days REAL,
    security_practices_score REAL,

    -- Community Health (25% weight)
    community_score REAL NOT NULL CHECK (community_score >= 0 AND community_score <= 100),
    download_trend TEXT,  -- 'growing', 'stable', 'declining'
    dependent_packages_count INTEGER DEFAULT 0,
    github_stars INTEGER DEFAULT 0,
    github_forks INTEGER DEFAULT 0,
    github_issues INTEGER DEFAULT 0,
    github_prs INTEGER DEFAULT 0,
    community_engagement_score REAL,

    -- Code Quality Health (20% weight)
    code_quality_score REAL NOT NULL CHECK (code_quality_score >= 0 AND code_quality_score <= 100),
    testing_score REAL,
    code_quality_tools_score REAL,
    documentation_score REAL,
    dependency_health_score REAL,

    -- Overall Health Score
    overall_health_score REAL NOT NULL CHECK (overall_health_score >= 0 AND overall_health_score <= 100),
    health_category TEXT NOT NULL CHECK (health_category IN ('production_ready', 'beta_quality', 'alpha_quality', 'not_recommended')),

    calculated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_package_health_overall_score (overall_health_score),
    INDEX idx_package_health_category (health_category)
);

-- ============================================================================
-- PACKAGE STATISTICS DATA
-- ============================================================================

-- Download statistics
CREATE TABLE package_downloads (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,

    -- Download counts
    total_downloads INTEGER DEFAULT 0,
    daily_downloads INTEGER DEFAULT 0,
    weekly_downloads INTEGER DEFAULT 0,
    monthly_downloads INTEGER DEFAULT 0,
    yearly_downloads INTEGER DEFAULT 0,

    -- Download trends
    growth_rate REAL,
    download_velocity REAL,
    seasonality TEXT,  -- 'stable', 'seasonal', 'irregular'
    popularity_rank INTEGER,

    -- Period tracking
    period_start DATETIME NOT NULL,
    period_end DATETIME NOT NULL,
    calculated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_package_downloads_total (total_downloads),
    INDEX idx_package_downloads_period (period_start, period_end)
);

-- Dependency statistics
CREATE TABLE package_dependencies (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    dependency_name TEXT NOT NULL,
    dependency_version TEXT,
    dependency_type TEXT NOT NULL CHECK (dependency_type IN ('runtime', 'development', 'peer', 'optional')),
    is_vulnerable BOOLEAN DEFAULT FALSE,
    is_maintained BOOLEAN DEFAULT TRUE,
    last_updated DATETIME,

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_package_dependencies_package (package_id),
    INDEX idx_package_dependencies_name (dependency_name),
    INDEX idx_package_dependencies_type (dependency_type)
);

-- Dependent packages (reverse dependencies)
CREATE TABLE package_dependents (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    dependent_name TEXT NOT NULL,
    dependent_version TEXT,
    dependent_type TEXT NOT NULL CHECK (dependent_type IN ('direct', 'indirect')),
    added_at DATETIME,

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_package_dependents_package (package_id),
    INDEX idx_package_dependents_name (dependent_name)
);

-- ============================================================================
-- PACKAGE METADATA DATA
-- ============================================================================

-- Content information
CREATE TABLE package_content (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    readme_content TEXT,
    changelog_content TEXT,
    api_documentation TEXT,
    repository_structure TEXT,  -- JSON of file organization
    scripts TEXT,  -- JSON of package scripts (NPM)

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- Classification data
CREATE TABLE package_classifications (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    classification_type TEXT NOT NULL,  -- 'keyword', 'category', 'tag', 'classifier'
    classification_value TEXT NOT NULL,
    classification_metadata TEXT,  -- JSON for additional metadata

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_package_classifications (package_id, classification_type)
);

-- ============================================================================
-- REGISTRY-SPECIFIC DATA
-- ============================================================================

-- NPM-specific data
CREATE TABLE npm_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    scripts TEXT,  -- JSON of package scripts
    dist_tags TEXT,  -- JSON of distribution tags
    maintainers TEXT,  -- JSON of maintainer information
    bin_commands TEXT,  -- JSON of binary commands
    engines TEXT,  -- JSON of engine requirements

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- PyPI-specific data
CREATE TABLE pypi_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    classifiers TEXT,  -- JSON of PyPI classifiers
    python_requirements TEXT,
    project_urls TEXT,  -- JSON of project URLs
    platform_support TEXT,  -- JSON of supported platforms
    requires_dist TEXT,  -- JSON of dependencies

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- Crates.io-specific data
CREATE TABLE crates_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    features TEXT,  -- JSON of optional features
    categories TEXT,  -- JSON of package categories
    keywords TEXT,  -- JSON of package keywords
    badges TEXT,  -- JSON of CI/CD badges
    documentation_url TEXT,

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- Maven Central-specific data
CREATE TABLE maven_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    group_id TEXT NOT NULL,
    artifact_id TEXT NOT NULL,
    packaging TEXT,  -- JAR, WAR, POM, etc.
    parent_group_id TEXT,
    parent_artifact_id TEXT,
    parent_version TEXT,
    properties TEXT,  -- JSON of Maven properties

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- Go Modules-specific data
CREATE TABLE go_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    module_path TEXT NOT NULL,
    go_version TEXT,
    replace_directives TEXT,  -- JSON of replace directives
    exclude_directives TEXT,  -- JSON of exclude directives
    retract_directives TEXT,  -- JSON of retract directives

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- RubyGems-specific data
CREATE TABLE rubygems_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    gem_specification TEXT,  -- JSON of gem specification
    ruby_version TEXT,
    platform TEXT,  -- ruby, jruby, etc.
    extensions TEXT,  -- JSON of native extensions
    certificates TEXT,  -- JSON of code signing certificates

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- Packagist-specific data
CREATE TABLE packagist_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    composer_requirements TEXT,  -- JSON of PHP version and dependencies
    autoloading TEXT,  -- JSON of PSR-0, PSR-4 autoloading
    scripts TEXT,  -- JSON of Composer scripts
    repositories TEXT,  -- JSON of additional repositories
    minimum_stability TEXT,

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- NuGet-specific data
CREATE TABLE nuget_package_data (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    package_id_nuget TEXT NOT NULL,
    target_frameworks TEXT,  -- JSON of .NET target frameworks
    dependencies TEXT,  -- JSON of package dependencies
    tags TEXT,  -- JSON of package tags
    license_url TEXT,

    FOREIGN KEY (package_id) REFERENCES packages(package_id)
);

-- ============================================================================
-- VERSION HISTORY DATA
-- ============================================================================

-- Package versions
CREATE TABLE package_versions (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    version TEXT NOT NULL,
    published_at DATETIME NOT NULL,
    size INTEGER,
    download_url TEXT,
    yanked BOOLEAN DEFAULT FALSE,
    yanked_reason TEXT,

    -- Version-specific metadata
    version_metadata TEXT,  -- JSON of version-specific data

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_package_versions_package (package_id),
    INDEX idx_package_versions_published (published_at)
);

-- Version dependencies (per version)
CREATE TABLE version_dependencies (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    version TEXT NOT NULL,
    dependency_name TEXT NOT NULL,
    dependency_version TEXT,
    dependency_type TEXT NOT NULL CHECK (dependency_type IN ('runtime', 'development', 'peer', 'optional')),

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_version_dependencies_package_version (package_id, version)
);

-- ============================================================================
-- COLLECTION METADATA
-- ============================================================================

-- Collection runs
CREATE TABLE collection_runs (
    id INTEGER PRIMARY KEY,
    run_id TEXT UNIQUE NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    status TEXT NOT NULL CHECK (status IN ('running', 'completed', 'failed', 'cancelled')),
    packages_collected INTEGER DEFAULT 0,
    errors_encountered INTEGER DEFAULT 0,
    rate_limit_hits INTEGER DEFAULT 0,

    INDEX idx_collection_runs_status (status),
    INDEX idx_collection_runs_start_time (start_time)
);

-- Collection errors
CREATE TABLE collection_errors (
    id INTEGER PRIMARY KEY,
    run_id TEXT NOT NULL,
    package_id TEXT,
    error_type TEXT NOT NULL,
    error_message TEXT,
    error_timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (run_id) REFERENCES collection_runs(run_id),
    INDEX idx_collection_errors_run (run_id),
    INDEX idx_collection_errors_type (error_type)
);

-- API rate limits tracking
CREATE TABLE api_rate_limits (
    id INTEGER PRIMARY KEY,
    registry TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    remaining_requests INTEGER NOT NULL,
    reset_time DATETIME NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    INDEX idx_api_rate_limits_registry (registry),
    INDEX idx_api_rate_limits_reset (reset_time)
);

-- Package conflicts tracking
CREATE TABLE package_conflicts (
    id INTEGER PRIMARY KEY,
    package_id TEXT NOT NULL,
    conflict_type TEXT NOT NULL CHECK (conflict_type IN ('version', 'license', 'author', 'repository', 'description')),
    source_registry TEXT NOT NULL,
    conflicting_value TEXT NOT NULL,
    resolution_applied TEXT,
    resolution_confidence REAL CHECK (resolution_confidence >= 0 AND resolution_confidence <= 1),
    requires_manual_review BOOLEAN DEFAULT FALSE,
    resolved_at DATETIME,
    resolved_by TEXT CHECK (resolved_by IN ('automatic', 'manual')),

    FOREIGN KEY (package_id) REFERENCES packages(package_id),
    INDEX idx_package_conflicts_package (package_id),
    INDEX idx_package_conflicts_type (conflict_type),
    INDEX idx_package_conflicts_review (requires_manual_review)
);

-- ============================================================================
-- VIEWS FOR COMMON QUERIES
-- ============================================================================

-- Package health summary view
CREATE VIEW package_health_summary AS
SELECT
    p.package_id,
    p.name,
    p.registry,
    p.version,
    ph.overall_health_score,
    ph.health_category,
    ph.maintenance_score,
    ph.security_score,
    ph.community_score,
    ph.code_quality_score,
    pd.total_downloads,
    pd.growth_rate,
    ph.calculated_at
FROM packages p
LEFT JOIN package_health ph ON p.package_id = ph.package_id
LEFT JOIN package_downloads pd ON p.package_id = pd.package_id
WHERE pd.period_end = (
    SELECT MAX(period_end)
    FROM package_downloads pd2
    WHERE pd2.package_id = p.package_id
);

-- High-quality packages view
CREATE VIEW high_quality_packages AS
SELECT
    p.package_id,
    p.name,
    p.registry,
    p.version,
    ph.overall_health_score,
    ph.health_category,
    pd.total_downloads,
    pd.growth_rate
FROM packages p
JOIN package_health ph ON p.package_id = ph.package_id
JOIN package_downloads pd ON p.package_id = pd.package_id
WHERE ph.health_category = 'production_ready'
  AND ph.overall_health_score >= 80
  AND pd.total_downloads > 1000;

-- Package dependency health view
CREATE VIEW package_dependency_health AS
SELECT
    p.package_id,
    p.name,
    p.registry,
    COUNT(d.id) as dependency_count,
    COUNT(CASE WHEN d.is_vulnerable = TRUE THEN 1 END) as vulnerable_dependencies,
    COUNT(CASE WHEN d.is_maintained = FALSE THEN 1 END) as unmaintained_dependencies,
    COUNT(dep.id) as dependent_count
FROM packages p
LEFT JOIN package_dependencies d ON p.package_id = d.package_id
LEFT JOIN package_dependents dep ON p.package_id = dep.package_id
GROUP BY p.package_id, p.name, p.registry;
