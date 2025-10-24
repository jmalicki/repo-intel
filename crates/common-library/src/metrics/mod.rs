//! Metrics module for statistical calculations and metrics processing
//!
//! This module provides comprehensive statistical analysis, trend detection,
//! growth rate calculations, and performance metrics computation.

pub mod growth;
pub mod normalization;
pub mod performance;
pub mod statistical;
pub mod trends;

// Re-exports for convenient usage
pub use growth::{GrowthCalculator, GrowthMetrics, GrowthRate};
pub use normalization::{DataNormalizer, NormalizationMethod};
pub use performance::{PerformanceAnalyzer, PerformanceMetrics};
pub use statistical::{StatisticalCalculator, StatisticalResult};
pub use trends::{TrendAnalyzer, TrendResult, TrendType};
