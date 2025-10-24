//! Metrics module for statistical calculations and metrics processing
//!
//! This module provides comprehensive statistical analysis, trend detection,
//! growth rate calculations, and performance metrics computation.

pub mod statistical;
pub mod trends;
pub mod growth;
pub mod performance;
pub mod normalization;

// Re-exports for convenient usage
pub use statistical::{StatisticalCalculator, StatisticalResult};
pub use trends::{TrendAnalyzer, TrendResult, TrendType};
pub use growth::{GrowthCalculator, GrowthRate, GrowthMetrics};
pub use performance::{PerformanceMetrics, PerformanceAnalyzer};
pub use normalization::{NormalizationMethod, DataNormalizer};
