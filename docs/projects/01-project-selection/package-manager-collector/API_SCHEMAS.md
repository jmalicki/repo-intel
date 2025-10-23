# Package Manager API Schemas

**Parent:** [Package Manager Collector README](README.md) → [Package Manager Collector Design](DESIGN.md)

## Overview

This document contains the actual API response schemas for NPM, PyPI, and Crates.io based on their real API endpoints. These schemas reflect what the APIs actually return, not idealized data models.

## NPM Registry API

**Endpoint:** `https://registry.npmjs.org/{package_name}`

### Example Response (lodash)
```json
{
  "_id": "lodash",
  "_rev": "1-abc123",
  "name": "lodash",
  "description": "Lodash modular utilities.",
  "dist-tags": {
    "latest": "4.17.21"
  },
  "versions": {
    "4.17.21": {
      "name": "lodash",
      "version": "4.17.21",
      "description": "Lodash modular utilities.",
      "main": "lodash.js",
      "homepage": "https://lodash.com/",
      "repository": {
        "type": "git",
        "url": "git+https://github.com/lodash/lodash.git"
      },
      "author": {
        "name": "John-David Dalton",
        "email": "john.david.dalton@gmail.com",
        "url": "http://allyoucanleet.com/"
      },
      "license": "MIT",
      "keywords": [
        "browser",
        "client",
        "functional",
        "server",
        "util"
      ],
      "dependencies": {},
      "devDependencies": {},
      "engines": {
        "node": ">=4.0.0"
      }
    }
  },
  "readme": "# lodash\n\nA modern JavaScript utility library...",
  "time": {
    "4.17.21": "2021-02-22T01:07:13.000Z",
    "created": "2012-04-23T16:03:23.000Z",
    "modified": "2021-02-22T01:07:13.000Z"
  },
  "maintainers": [
    {
      "name": "jdalton",
      "email": "john.david.dalton@gmail.com"
    }
  ],
  "repository": {
    "type": "git",
    "url": "git+https://github.com/lodash/lodash.git"
  },
  "homepage": "https://lodash.com/",
  "license": "MIT",
  "keywords": [
    "browser",
    "client",
    "functional",
    "server",
    "util"
  ]
}
```

### Key Fields Available
- `name` - Package name
- `description` - Package description
- `version` - Latest version (from dist-tags.latest)
- `homepage` - Package homepage URL
- `repository` - Git repository information
- `license` - License type
- `keywords` - Package keywords
- `author` - Author information
- `maintainers` - List of maintainers
- `time` - Version timestamps
- `readme` - README content
- `versions` - All version metadata
- `dependencies` - Package dependencies (per version)

### Additional Data Sources Available
- **Download Statistics**: `https://api.npmjs.org/downloads/point/{period}/{package}` (npm-statistics)
- **Search API**: `https://registry.npmjs.org/-/v1/search?text={query}` (package discovery)
- **Dependent Packages**: `https://registry.npmjs.org/{package}/dependents` (reverse dependencies)
- **Security Advisories**: `https://github.com/advisories` (security vulnerabilities)

### Missing/Not Available
- Star ratings (NPM doesn't have this)
- User reviews/ratings
- Social metrics (likes, follows, etc.)

## PyPI API

**Endpoint:** `https://pypi.org/pypi/{package_name}/json`

### Example Response (requests)
```json
{
  "info": {
    "author": "Kenneth Reitz",
    "author_email": "me@kennethreitz.org",
    "bugtrack_url": null,
    "classifiers": [
      "Development Status :: 4 - Beta",
      "Intended Audience :: Developers",
      "License :: OSI Approved :: Apache Software License",
      "Natural Language :: English",
      "Operating System :: OS Independent",
      "Programming Language :: Python :: 2",
      "Programming Language :: Python :: 2.7",
      "Programming Language :: Python :: 3",
      "Programming Language :: Python :: 3.6",
      "Programming Language :: Python :: 3.7",
      "Programming Language :: Python :: 3.8",
      "Programming Language :: Python :: 3.9"
    ],
    "description": "Python HTTP for Humans.",
    "description_content_type": "text/x-rst",
    "docs_url": null,
    "download_url": "",
    "home_page": "https://requests.readthedocs.io",
    "keywords": "requests,http,urllib3",
    "license": "Apache 2.0",
    "maintainer": "",
    "maintainer_email": "",
    "name": "requests",
    "package_url": "https://pypi.org/project/requests/",
    "platform": "",
    "project_url": "https://pypi.org/project/requests/",
    "project_urls": {
      "Homepage": "https://requests.readthedocs.io",
      "Repository": "https://github.com/psf/requests"
    },
    "release_url": "https://pypi.org/project/requests/2.28.1/",
    "requires_dist": [
      "urllib3 (>=1.21.1,<2.0.0)",
      "certifi (>=2017.4.17)",
      "charset-normalizer (>=2,<3); python_version>=\"3\"",
      "idna (>=2.5,<4); python_version>=\"3\""
    ],
    "requires_python": ">=2.7, !=3.0.*, !=3.1.*, !=3.2.*, !=3.3.*, !=3.4.*, !=3.5.*",
    "summary": "Python HTTP for Humans.",
    "version": "2.28.1",
    "yanked": false,
    "yanked_reason": null
  },
  "last_serial": 12345678,
  "releases": {
    "2.28.1": [
      {
        "comment_text": "",
        "digests": {
          "md5": "abc123...",
          "sha256": "def456..."
        },
        "downloads": 0,
        "filename": "requests-2.28.1-py2.py3-none-any.whl",
        "has_sig": false,
        "md5_digest": "abc123...",
        "packagetype": "bdist_wheel",
        "python_version": "py2.py3",
        "requires_python": ">=2.7, !=3.0.*, !=3.1.*, !=3.2.*, !=3.3.*, !=3.4.*, !=3.5.*",
        "size": 1234567,
        "upload_time": "2022-01-15T10:00:00",
        "upload_time_iso_8601": "2022-01-15T10:00:00.000Z",
        "url": "https://files.pythonhosted.org/packages/...",
        "yanked": false,
        "yanked_reason": null
      }
    ]
  },
  "urls": [
    {
      "comment_text": "",
      "digests": {
        "md5": "abc123...",
        "sha256": "def456..."
      },
      "downloads": 0,
      "filename": "requests-2.28.1-py2.py3-none-any.whl",
      "has_sig": false,
      "md5_digest": "abc123...",
      "packagetype": "bdist_wheel",
      "python_version": "py2.py3",
      "requires_python": ">=2.7, !=3.0.*, !=3.1.*, !=3.2.*, !=3.3.*, !=3.4.*, !=3.5.*",
      "size": 1234567,
      "upload_time": "2022-01-15T10:00:00",
      "upload_time_iso_8601": "2022-01-15T10:00:00.000Z",
      "url": "https://files.pythonhosted.org/packages/...",
      "yanked": false,
      "yanked_reason": null
    }
  ]
}
```

### Key Fields Available
- `info.name` - Package name
- `info.version` - Latest version
- `info.summary` - Package summary
- `info.description` - Full description
- `info.author` - Author name
- `info.author_email` - Author email
- `info.license` - License
- `info.home_page` - Homepage URL
- `info.project_urls` - Project URLs (homepage, repository)
- `info.keywords` - Keywords
- `info.requires_dist` - Dependencies
- `info.requires_python` - Python version requirements
- `info.classifiers` - PyPI classifiers
- `releases` - All release information
- `urls` - Download URLs for latest version

### Additional Data Sources Available
- **Download Statistics**: `https://pypistats.org/api/packages/{package}/overall` (PyPI Stats API)
- **Search API**: `https://pypi.org/search/?q={query}` (package discovery)
- **Dependent Packages**: `https://libraries.io/api/pypi/{package}/dependents` (Libraries.io API)
- **Security Advisories**: `https://github.com/pypa/advisory-database` (Python security advisories)
- **Package Health**: `https://libraries.io/api/pypi/{package}` (Libraries.io health metrics)

### Missing/Not Available
- Star ratings (PyPI doesn't have this)
- User reviews/ratings
- Social metrics (likes, follows, etc.)

## Crates.io API

**Endpoint:** `https://crates.io/api/v1/crates/{crate_name}`

### Example Response (serde)
```json
{
  "categories": [
    {
      "category": "encoding",
      "crates_cnt": 150,
      "created_at": "2014-11-15T19:20:12Z",
      "description": "Encoding and decoding",
      "id": "encoding",
      "slug": "encoding"
    }
  ],
  "crate": {
    "badges": [],
    "categories": ["encoding"],
    "created_at": "2014-11-15T19:20:12Z",
    "description": "A generic serialization/deserialization framework",
    "documentation": "https://docs.rs/serde",
    "downloads": 123456789,
    "exact_match": false,
    "features": {
      "alloc": [],
      "derive": [
        "serde_derive"
      ],
      "std": [
        "serde/std"
      ]
    },
    "homepage": "https://serde.rs",
    "id": "serde",
    "keywords": [
      "serde",
      "serialization",
      "deserialization",
      "json"
    ],
    "links": {
      "crate": "https://crates.io/crates/serde",
      "owners": "https://crates.io/crates/serde/owners",
      "owners_team": "https://crates.io/crates/serde/owner_team",
      "reverse_dependencies": "https://crates.io/crates/serde/reverse_dependencies",
      "versions": "https://crates.io/crates/serde/versions"
    },
    "max_stable_version": "1.0.163",
    "max_version": "1.0.163",
    "name": "serde",
    "new_version": null,
    "next_version": null,
    "recent_downloads": 1234567,
    "repository": "https://github.com/serde-rs/serde",
    "updated_at": "2023-01-15T10:00:00Z",
    "versions": [1, 2, 3, 4, 5]
  },
  "keywords": [
    {
      "crates_cnt": 150,
      "created_at": "2014-11-15T19:20:12Z",
      "id": "serde",
      "keyword": "serde"
    }
  ],
  "versions": [
    {
      "crate": "serde",
      "crate_size": 1234567,
      "created_at": "2023-01-15T10:00:00Z",
      "dl_path": "/api/v1/crates/serde/1.0.163/download",
      "downloads": 1234567,
      "features": {
        "alloc": [],
        "derive": ["serde_derive"],
        "std": ["serde/std"]
      },
      "id": 123456,
      "license": "MIT OR Apache-2.0",
      "links": {
        "authors": "https://crates.io/crates/serde/1.0.163/authors",
        "dependencies": "https://crates.io/crates/serde/1.0.163/dependencies",
        "version_downloads": "https://crates.io/crates/serde/1.0.163/downloads"
      },
      "num": "1.0.163",
      "published_by": {
        "avatar": "https://avatars.githubusercontent.com/u/123456",
        "id": 123456,
        "login": "dtolnay",
        "name": "David Tolnay",
        "url": "https://github.com/dtolnay"
      },
      "readme_path": "/api/v1/crates/serde/1.0.163/readme",
      "updated_at": "2023-01-15T10:00:00Z",
      "yanked": false
    }
  ]
}
```

### Key Fields Available
- `crate.name` - Crate name
- `crate.description` - Crate description
- `crate.downloads` - Total downloads
- `crate.recent_downloads` - Recent downloads
- `crate.repository` - Repository URL
- `crate.homepage` - Homepage URL
- `crate.documentation` - Documentation URL
- `crate.license` - License
- `crate.keywords` - Keywords
- `crate.categories` - Categories
- `crate.max_version` - Latest version
- `crate.created_at` - Creation date
- `crate.updated_at` - Last update date
- `versions` - All version information
- `categories` - Category information
- `keywords` - Keyword information

### Additional Data Sources Available
- **Download Statistics**: `https://crates.io/api/v1/crates/{crate}/downloads` (download counts)
- **Dependent Packages**: `https://crates.io/api/v1/crates/{crate}/reverse_dependencies` (reverse dependencies)
- **Search API**: `https://crates.io/api/v1/crates?q={query}` (crate discovery)
- **Security Advisories**: `https://github.com/RustSec/advisory-db` (Rust security advisories)
- **Package Health**: `https://libraries.io/api/cargo/{crate}` (Libraries.io health metrics)

### Missing/Not Available
- Star ratings (Crates.io doesn't have this)
- User reviews/ratings
- Social metrics (likes, follows, etc.)

## Additional Data Sources

### Download Statistics APIs
**NPM**: `https://api.npmjs.org/downloads/point/{period}/{package}`
- **Periods**: last-day, last-week, last-month, last-year
- **Data**: Download counts, trends, popularity metrics

**PyPI**: `https://pypistats.org/api/packages/{package}/overall`
- **Data**: Download counts, trends, popularity metrics
- **Periods**: Daily, weekly, monthly breakdowns

**Crates.io**: `https://crates.io/api/v1/crates/{crate}/downloads`
- **Data**: Download counts, trends, popularity metrics
- **Periods**: Daily, weekly, monthly breakdowns

### Security Advisory APIs
**NPM**: `https://github.com/advisories` (GitHub Security Advisories)
- **Data**: Security vulnerabilities, CVEs, affected versions
- **Format**: JSON API with vulnerability details

**PyPI**: `https://github.com/pypa/advisory-database` (Python Security Advisories)
- **Data**: Security vulnerabilities, CVEs, affected versions
- **Format**: JSON API with vulnerability details

**Crates.io**: `https://github.com/RustSec/advisory-db` (Rust Security Advisories)
- **Data**: Security vulnerabilities, CVEs, affected versions
- **Format**: JSON API with vulnerability details

### Package Health APIs
**Libraries.io**: `https://libraries.io/api/{platform}/{package}`
- **Platforms**: npm, pypi, cargo, maven, etc.
- **Data**: Package health scores, maintenance status, popularity metrics
- **Metrics**: Dependencies health, license compatibility, maintenance status

### Dependent Packages APIs
**NPM**: `https://registry.npmjs.org/{package}/dependents`
- **Data**: Direct and indirect dependents
- **Format**: JSON with dependent package information

**PyPI**: `https://libraries.io/api/pypi/{package}/dependents`
- **Data**: Direct and indirect dependents
- **Format**: JSON with dependent package information

**Crates.io**: `https://crates.io/api/v1/crates/{crate}/reverse_dependencies`
- **Data**: Direct and indirect dependents
- **Format**: JSON with dependent package information

## High-Value Data for Project Selection

### NPM Scripts Analysis
**Field**: `scripts` in package.json
**Value**: Reveals development practices and project maturity

**Key Scripts to Look For**:
- **`test`**: Indicates testing practices
- **`build`**: Shows build process sophistication
- **`lint`**: Code quality enforcement
- **`format`**: Code formatting standards
- **`precommit`**: Pre-commit hooks for quality
- **`coverage`**: Test coverage reporting
- **`docs`**: Documentation generation
- **`release`**: Release automation
- **`ci`**: Continuous integration setup

**Quality Indicators**:
- **High Quality**: Has test, lint, build, format, precommit
- **Medium Quality**: Has test, build, lint
- **Low Quality**: Only has build or no scripts

### PyPI Classifiers Analysis
**Field**: `classifiers` in PyPI metadata
**Value**: Reveals project maturity and target audience

**Key Classifiers**:
- **Development Status**: Beta, Stable, Production/Stable
- **Intended Audience**: Developers, End Users, System Administrators
- **License**: OSI Approved, Proprietary, Public Domain
- **Programming Language**: Python 2, Python 3, specific versions
- **Topic**: Scientific/Engineering, Software Development, System

**Quality Indicators**:
- **High Quality**: Stable status, OSI license, Python 3, clear topics
- **Medium Quality**: Beta status, clear audience, good topics
- **Low Quality**: Alpha status, unclear audience, missing topics

### Crates.io Features Analysis
**Field**: `features` in Cargo.toml
**Value**: Reveals project architecture and optional functionality

**Key Features to Look For**:
- **`std`**: Standard library usage
- **`serde`**: Serialization support
- **`derive`**: Code generation capabilities
- **`async`**: Async/await support
- **`no_std`**: Embedded/constrained environments
- **`unstable`**: Experimental features

**Quality Indicators**:
- **High Quality**: Well-organized features, clear optionality
- **Medium Quality**: Basic feature organization
- **Low Quality**: Poor feature organization or missing features

### Repository Links Analysis
**Field**: `repository`, `homepage`, `documentation` URLs
**Value**: Reveals project organization and documentation

**Quality Indicators**:
- **High Quality**: GitHub repo, dedicated docs site, clear homepage
- **Medium Quality**: GitHub repo, basic documentation
- **Low Quality**: Missing or broken links

### License Analysis
**Field**: `license` across all package managers
**Value**: Reveals project openness and commercial viability

**License Categories**:
- **Permissive**: MIT, Apache-2.0, BSD-3-Clause
- **Copyleft**: GPL-2.0, GPL-3.0, AGPL-3.0
- **Proprietary**: Commercial, Private
- **Public Domain**: CC0, Unlicense

**Quality Indicators**:
- **High Quality**: Clear, permissive license (MIT, Apache)
- **Medium Quality**: Clear, copyleft license (GPL)
- **Low Quality**: Unclear or missing license

### Dependencies Analysis
**Field**: `dependencies`, `devDependencies`, `peerDependencies`
**Value**: Reveals project complexity and maintenance burden

**Quality Indicators**:
- **High Quality**: Minimal, well-maintained dependencies
- **Medium Quality**: Moderate dependencies, good maintenance
- **Low Quality**: Many dependencies, outdated packages

### Keywords Analysis
**Field**: `keywords` across all package managers
**Value**: Reveals project purpose and domain expertise

**Quality Indicators**:
- **High Quality**: Clear, relevant keywords, good categorization
- **Medium Quality**: Basic keywords, some relevance
- **Low Quality**: Missing or irrelevant keywords

## Package Health Analysis

### Maintenance Health Indicators
**Field**: Version history, update frequency, maintainer activity
**Value**: Reveals project maintenance quality and sustainability

**Key Metrics**:
- **Update Frequency**: Regular releases vs. abandoned projects
- **Version Stability**: Semantic versioning compliance
- **Breaking Changes**: Frequency of breaking changes
- **Maintainer Activity**: Recent commits, issue responses
- **Documentation Updates**: README, docs, changelog freshness

**Health Scores**:
- **Excellent (90-100)**: Active maintenance, regular updates, good docs
- **Good (70-89)**: Regular updates, some maintenance gaps
- **Fair (50-69)**: Irregular updates, basic maintenance
- **Poor (30-49)**: Infrequent updates, maintenance issues
- **Critical (0-29)**: Abandoned or poorly maintained

### Security Health Indicators
**Field**: Security advisories, dependency vulnerabilities
**Value**: Reveals security posture and risk assessment

**Key Metrics**:
- **Vulnerability Count**: Known security issues
- **Dependency Security**: Vulnerable dependencies
- **Security Response**: Time to fix vulnerabilities
- **Security Practices**: Security-focused development

**Health Scores**:
- **Excellent (90-100)**: No known vulnerabilities, secure dependencies
- **Good (70-89)**: Few vulnerabilities, quick fixes
- **Fair (50-69)**: Some vulnerabilities, slow fixes
- **Poor (30-49)**: Multiple vulnerabilities, slow response
- **Critical (0-29)**: High-risk vulnerabilities, no response

### Community Health Indicators
**Field**: Download counts, dependent packages, GitHub activity
**Value**: Reveals community adoption and support

**Key Metrics**:
- **Download Trends**: Growing, stable, or declining usage
- **Dependent Packages**: How many projects depend on this
- **GitHub Activity**: Stars, forks, issues, PRs
- **Community Engagement**: Issue responses, PR reviews

**Health Scores**:
- **Excellent (90-100)**: High adoption, active community
- **Good (70-89)**: Good adoption, engaged community
- **Fair (50-69)**: Moderate adoption, some community
- **Poor (30-49)**: Low adoption, limited community
- **Critical (0-29)**: Minimal adoption, no community

### Code Quality Health Indicators
**Field**: Scripts, dependencies, repository structure
**Value**: Reveals code quality and development practices

**Key Metrics**:
- **Testing**: Test scripts, coverage, CI/CD
- **Code Quality**: Linting, formatting, pre-commit hooks
- **Documentation**: README quality, API docs, examples
- **Dependencies**: Well-maintained, minimal, secure

**Health Scores**:
- **Excellent (90-100)**: Comprehensive testing, quality tools, great docs
- **Good (70-89)**: Good testing, some quality tools, decent docs
- **Fair (50-69)**: Basic testing, minimal quality tools, basic docs
- **Poor (30-49)**: Limited testing, no quality tools, poor docs
- **Critical (0-29)**: No testing, no quality tools, no docs

### Overall Package Health Score
**Formula**: Weighted average of all health indicators
**Weights**: Maintenance (30%), Security (25%), Community (25%), Code Quality (20%)

**Health Categories**:
- **Production Ready (80-100)**: Suitable for production use
- **Beta Quality (60-79)**: Good for testing, some risks
- **Alpha Quality (40-59)**: Experimental, significant risks
- **Not Recommended (0-39)**: Avoid for production use

## Comprehensive Data Model

### Core Package Data (Available from all APIs)
```json
{
  "package_id": "unique_identifier",
  "name": "package_name",
  "registry": "npm|pypi|crates.io",
  "description": "Package description",
  "version": "1.0.0",
  "license": "MIT",
  "homepage": "https://package-homepage.com",
  "repository": {
    "type": "git",
    "url": "https://github.com/owner/repo"
  },
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://author-website.com"
  },
  "keywords": ["keyword1", "keyword2"],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z",
  "collection_timestamp": "2024-01-15T10:30:00Z"
}
```

### Version History Data
```json
{
  "versions": [
    {
      "version": "1.0.0",
      "published_at": "2024-01-15T10:00:00Z",
      "size": 1234567,
      "download_url": "https://registry.npmjs.org/package/-/package-1.0.0.tgz",
      "dependencies": {
        "dep1": "^1.0.0",
        "dep2": "~2.0.0"
      },
      "dev_dependencies": {
        "test-dep": "^1.0.0"
      },
      "peer_dependencies": {
        "peer-dep": "^1.0.0"
      },
      "engines": {
        "node": ">=14.0.0"
      },
      "yanked": false,
      "yanked_reason": null
    }
  ],
  "latest_version": "1.0.0",
  "total_versions": 50,
  "release_frequency": "monthly"
}
```

### Download Statistics (Where Available)
```json
{
  "downloads": {
    "total": 1234567,
    "recent": 12345,
    "periods": {
      "daily": 1000,
      "weekly": 7000,
      "monthly": 30000,
      "yearly": 360000
    },
    "trends": {
      "growth_rate": 0.15,
      "velocity": 0.85,
      "seasonality": "stable"
    }
  }
}
```

### Dependency Analysis
```json
{
  "dependencies": {
    "direct": [
      {
        "name": "dep1",
        "version": "^1.0.0",
        "type": "runtime"
      }
    ],
    "dev_dependencies": [
      {
        "name": "test-dep",
        "version": "^1.0.0",
        "type": "development"
      }
    ],
    "peer_dependencies": [
      {
        "name": "peer-dep",
        "version": "^1.0.0",
        "type": "peer"
      }
    ],
    "total_count": 25,
    "dev_count": 5,
    "peer_count": 2
  },
  "dependents": {
    "direct_count": 800,
    "indirect_count": 700,
    "total_count": 1500,
    "recent_dependents": [
      {
        "name": "dependent-package",
        "version": "1.0.0",
        "added_at": "2024-01-10T00:00:00Z"
      }
    ]
  }
}
```

### Package Health Metrics
```json
{
  "health": {
    "maintenance_status": "active",
    "last_updated": "2024-01-15T10:00:00Z",
    "update_frequency": "monthly",
    "breaking_changes": 2,
    "security_vulnerabilities": 0,
    "deprecation_warnings": 0,
    "license_compatibility": "permissive"
  },
  "quality": {
    "documentation_score": 0.85,
    "test_coverage": 0.90,
    "code_quality": 0.88,
    "maintainability": 0.82
  }
}
```

### Registry-Specific Data

#### NPM-Specific
```json
{
  "npm": {
    "dist_tags": {
      "latest": "1.0.0",
      "beta": "1.1.0-beta.1",
      "alpha": "1.2.0-alpha.1"
    },
    "maintainers": [
      {
        "name": "maintainer1",
        "email": "maintainer1@example.com"
      }
    ],
    "readme": "# Package README content...",
    "scripts": {
      "test": "jest",
      "build": "webpack"
    },
    "bin": {
      "package-cli": "./bin/cli.js"
    }
  }
}
```

#### PyPI-Specific
```json
{
  "pypi": {
    "classifiers": [
      "Development Status :: 4 - Beta",
      "Intended Audience :: Developers",
      "License :: OSI Approved :: MIT License",
      "Programming Language :: Python :: 3"
    ],
    "requires_python": ">=3.6",
    "project_urls": {
      "Homepage": "https://package-homepage.com",
      "Documentation": "https://docs.package.com",
      "Repository": "https://github.com/owner/repo",
      "Bug Reports": "https://github.com/owner/repo/issues"
    },
    "platform": "any",
    "supported_platforms": ["linux", "windows", "macos"]
  }
}
```

#### Crates.io-Specific
```json
{
  "crates": {
    "categories": [
      {
        "id": "encoding",
        "name": "Encoding and decoding",
        "description": "Data encoding and decoding"
      }
    ],
    "keywords": [
      {
        "id": "serialization",
        "name": "serialization"
      }
    ],
    "features": {
      "default": ["std"],
      "std": ["serde/std"],
      "derive": ["serde_derive"]
    },
    "badges": [
      {
        "type": "travis-ci",
        "url": "https://travis-ci.org/owner/repo"
      }
    ],
    "documentation": "https://docs.rs/package"
  }
}
```

## Data Collection Strategy

### What We Can Collect Directly
- **Package metadata** (name, description, version, license, etc.)
- **Repository URLs** and homepage links
- **Author/maintainer information**
- **Dependencies** (direct, dev, peer)
- **Version history** with timestamps
- **Keywords and categories**
- **Registry-specific data** (dist-tags, classifiers, features)

### What Requires Additional API Calls
- **Download statistics** (npm-statistics, PyPI stats APIs)
- **Dependent packages** (reverse dependency APIs)
- **Security vulnerabilities** (security advisory APIs)
- **Package health metrics** (derived from multiple sources)

### What's Not Available
- **Star ratings** (none of these registries have this)
- **User reviews/ratings**
- **Social metrics** (likes, follows, etc.)
- **Real-time download counts** (most are aggregated)

## Implementation Notes

1. **Rate Limiting**: Each API has different rate limits
   - NPM: No official limits, but respectful usage recommended
   - PyPI: No official limits, but respectful usage recommended  
   - Crates.io: 10 requests per second
2. **Authentication**: Most endpoints don't require authentication
3. **Data Consistency**: Different APIs have different data structures
4. **Error Handling**: APIs may return 404 for non-existent packages
5. **Caching**: Consider caching responses to avoid repeated API calls
6. **Data Validation**: Validate required fields and handle missing data gracefully
