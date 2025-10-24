//! Validation module for data validation and schema management
//!
//! This module provides comprehensive data validation including JSON schema validation,
//! data integrity checks, type validation, and schema registry management.

pub mod errors;
pub mod integrity;
pub mod registry;
pub mod schema;
pub mod types;

// Re-exports for convenient usage
pub use errors::{ValidationError, ValidationErrorType, ValidationSuggestion};
pub use integrity::{DataIntegrityChecker, IntegrityResult, IntegrityViolation};
pub use registry::{SchemaMetadata, SchemaRegistry, SchemaRegistryError};
pub use schema::{SchemaError, SchemaValidationResult, SchemaValidator};
pub use types::{TypeConstraint, TypeValidationResult, TypeValidator};
