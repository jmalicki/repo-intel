//! Package Manager Collector Data Models
//!
//! This module contains all data models for the package manager collector,
//! including core package data, registry-specific data, and collection metadata.

pub mod core;
pub mod registry;
pub mod collection;

pub use core::*;
pub use registry::*;
pub use collection::*;
