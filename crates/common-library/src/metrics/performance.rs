//! Performance metrics computation
//!
//! This module provides performance analysis including efficiency metrics,
//! throughput calculations, and performance benchmarking.

use crate::error::{Error, Result};
use crate::logging::Logger;
use std::time::{Duration, Instant};

/// Performance metrics result
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub throughput: f64,
    pub efficiency: f64,
    pub latency: Duration,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub error_rate: f64,
    pub success_rate: f64,
    pub performance_score: f64,
}

/// Performance benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub operation_name: String,
    pub total_time: Duration,
    pub operations_per_second: f64,
    pub average_latency: Duration,
    pub min_latency: Duration,
    pub max_latency: Duration,
    pub p95_latency: Duration,
    pub p99_latency: Duration,
    pub throughput_mbps: f64,
}

/// Performance analyzer for computing performance metrics
pub struct PerformanceAnalyzer {
    logger: Logger,
}

impl PerformanceAnalyzer {
    /// Create a new performance analyzer
    pub fn new() -> Self {
        Self {
            logger: Logger::new("performance_analyzer"),
        }
    }

    /// Calculate comprehensive performance metrics
    pub fn calculate_performance_metrics(
        &self,
        operations_count: usize,
        total_time: Duration,
        successful_operations: usize,
        failed_operations: usize,
        data_size_bytes: u64,
    ) -> Result<PerformanceMetrics> {
        if operations_count == 0 {
            return Err(Error::metrics("Cannot calculate performance metrics for zero operations".to_string()));
        }

        self.logger.info(&format!("Calculating performance metrics for {} operations", operations_count));

        let total_operations = successful_operations + failed_operations;
        let throughput = operations_count as f64 / total_time.as_secs_f64();
        let efficiency = if operations_count > 0 {
            (successful_operations as f64 / operations_count as f64) * 100.0
        } else {
            0.0
        };

        let latency = if operations_count > 0 {
            Duration::from_secs_f64(total_time.as_secs_f64() / operations_count as f64)
        } else {
            Duration::ZERO
        };

        let error_rate = if total_operations > 0 {
            (failed_operations as f64 / total_operations as f64) * 100.0
        } else {
            0.0
        };

        let success_rate = 100.0 - error_rate;
        let throughput_mbps = if total_time.as_secs_f64() > 0.0 {
            (data_size_bytes as f64 / 1_048_576.0) / total_time.as_secs_f64()
        } else {
            0.0
        };

        // Calculate performance score (0-100)
        let performance_score = self.calculate_performance_score(
            throughput,
            efficiency,
            error_rate,
            latency.as_secs_f64(),
        );

        Ok(PerformanceMetrics {
            throughput,
            efficiency,
            latency,
            cpu_usage: 0.0, // Would need system monitoring
            memory_usage: 0.0, // Would need system monitoring
            error_rate,
            success_rate,
            performance_score,
        })
    }

    /// Calculate performance score based on multiple factors
    fn calculate_performance_score(
        &self,
        throughput: f64,
        efficiency: f64,
        error_rate: f64,
        latency_seconds: f64,
    ) -> f64 {
        // Normalize throughput (assuming 1000 ops/sec is excellent)
        let throughput_score = (throughput / 1000.0).min(1.0) * 30.0;

        // Efficiency score (direct percentage)
        let efficiency_score = (efficiency / 100.0) * 25.0;

        // Error rate score (inverse relationship)
        let error_score = ((100.0 - error_rate) / 100.0) * 25.0;

        // Latency score (assuming 1ms is excellent)
        let latency_score = if latency_seconds > 0.0 {
            (0.001 / latency_seconds).min(1.0) * 20.0
        } else {
            20.0
        };

        (throughput_score + efficiency_score + error_score + latency_score).min(100.0)
    }

    /// Run a performance benchmark
    pub fn run_benchmark<F>(
        &self,
        operation_name: &str,
        operation: F,
        iterations: usize,
        data_size_bytes: u64,
    ) -> Result<BenchmarkResult>
    where
        F: Fn() -> Result<()>,
    {
        if iterations == 0 {
            return Err(Error::metrics("Cannot run benchmark with zero iterations".to_string()));
        }

        self.logger.info(&format!("Running benchmark '{}' with {} iterations", operation_name, iterations));

        let mut latencies = Vec::with_capacity(iterations);
        let start_time = Instant::now();

        for _ in 0..iterations {
            let operation_start = Instant::now();
            operation()?;
            let operation_duration = operation_start.elapsed();
            latencies.push(operation_duration);
        }

        let total_time = start_time.elapsed();
        let operations_per_second = iterations as f64 / total_time.as_secs_f64();

        // Calculate latency statistics
        latencies.sort();
        let average_latency = Duration::from_nanos(
            latencies.iter().map(|d| d.as_nanos() as u64).sum::<u64>() / latencies.len() as u64
        );
        let min_latency = latencies[0];
        let max_latency = latencies[latencies.len() - 1];
        let p95_latency = latencies[(latencies.len() as f64 * 0.95) as usize];
        let p99_latency = latencies[(latencies.len() as f64 * 0.99) as usize];

        let throughput_mbps = if total_time.as_secs_f64() > 0.0 {
            (data_size_bytes as f64 / 1_048_576.0) / total_time.as_secs_f64()
        } else {
            0.0
        };

        let result = BenchmarkResult {
            operation_name: operation_name.to_string(),
            total_time,
            operations_per_second,
            average_latency,
            min_latency,
            max_latency,
            p95_latency,
            p99_latency,
            throughput_mbps,
        };

        self.logger.info(&format!("Benchmark completed: {:.2} ops/sec", operations_per_second));
        Ok(result)
    }

    /// Calculate throughput for a given operation
    pub fn calculate_throughput(&self, operations_count: usize, duration: Duration) -> f64 {
        if duration.as_secs_f64() == 0.0 {
            return 0.0;
        }
        operations_count as f64 / duration.as_secs_f64()
    }

    /// Calculate efficiency based on successful vs total operations
    pub fn calculate_efficiency(&self, successful_operations: usize, total_operations: usize) -> f64 {
        if total_operations == 0 {
            return 0.0;
        }
        (successful_operations as f64 / total_operations as f64) * 100.0
    }

    /// Calculate error rate
    pub fn calculate_error_rate(&self, failed_operations: usize, total_operations: usize) -> f64 {
        if total_operations == 0 {
            return 0.0;
        }
        (failed_operations as f64 / total_operations as f64) * 100.0
    }

    /// Calculate latency percentiles
    pub fn calculate_latency_percentiles(&self, latencies: &[Duration]) -> Result<Vec<(f64, Duration)>> {
        if latencies.is_empty() {
            return Err(Error::metrics("Cannot calculate percentiles for empty latency data".to_string()));
        }

        let mut sorted_latencies = latencies.to_vec();
        sorted_latencies.sort();

        let percentiles = vec![50.0, 90.0, 95.0, 99.0, 99.9];
        let mut results = Vec::new();

        for percentile in percentiles {
            let index = ((percentile / 100.0) * (sorted_latencies.len() - 1) as f64) as usize;
            let latency = sorted_latencies[index.min(sorted_latencies.len() - 1)];
            results.push((percentile, latency));
        }

        Ok(results)
    }

    /// Calculate memory efficiency
    pub fn calculate_memory_efficiency(&self, data_size: u64, memory_used: u64) -> f64 {
        if memory_used == 0 {
            return 0.0;
        }
        (data_size as f64 / memory_used as f64) * 100.0
    }

    /// Calculate CPU efficiency (simplified)
    pub fn calculate_cpu_efficiency(&self, operations_count: usize, cpu_time: Duration, wall_time: Duration) -> f64 {
        if wall_time.as_secs_f64() == 0.0 {
            return 0.0;
        }
        (cpu_time.as_secs_f64() / wall_time.as_secs_f64()) * 100.0
    }

    /// Compare performance between two benchmarks
    pub fn compare_performance(&self, baseline: &BenchmarkResult, current: &BenchmarkResult) -> PerformanceComparison {
        let throughput_improvement = ((current.operations_per_second - baseline.operations_per_second)
            / baseline.operations_per_second) * 100.0;

        let latency_improvement = ((baseline.average_latency.as_secs_f64() - current.average_latency.as_secs_f64())
            / baseline.average_latency.as_secs_f64()) * 100.0;

        PerformanceComparison {
            throughput_improvement,
            latency_improvement,
            is_better: throughput_improvement > 0.0 && latency_improvement > 0.0,
            overall_improvement: (throughput_improvement + latency_improvement) / 2.0,
        }
    }
}

/// Performance comparison result
#[derive(Debug, Clone)]
pub struct PerformanceComparison {
    pub throughput_improvement: f64,
    pub latency_improvement: f64,
    pub is_better: bool,
    pub overall_improvement: f64,
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
