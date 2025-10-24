//! Statistical calculation functions
//!
//! This module provides comprehensive statistical analysis functions including
//! descriptive statistics, correlation analysis, and distribution metrics.

use crate::error::{Error, Result};
use crate::logging::Logger;
use std::collections::HashMap;

/// Statistical calculation result
#[derive(Debug, Clone)]
pub struct StatisticalResult {
    pub mean: f64,
    pub median: f64,
    pub mode: Option<f64>,
    pub standard_deviation: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
    pub range: f64,
    pub quartiles: Quartiles,
    pub skewness: f64,
    pub kurtosis: f64,
    pub count: usize,
}

/// Quartile values
#[derive(Debug, Clone)]
pub struct Quartiles {
    pub q1: f64,
    pub q2: f64, // median
    pub q3: f64,
    pub iqr: f64, // interquartile range
}

/// Statistical calculator for data analysis
pub struct StatisticalCalculator {
    logger: Logger,
}

impl StatisticalCalculator {
    /// Create a new statistical calculator
    pub fn new() -> Self {
        Self {
            logger: Logger::new("statistical_calculator"),
        }
    }

    /// Calculate comprehensive statistics for a dataset
    pub fn calculate_statistics(&self, data: &[f64]) -> Result<StatisticalResult> {
        if data.is_empty() {
            return Err(Error::metrics("Cannot calculate statistics for empty dataset".to_string()));
        }

        self.logger.info(&format!("Calculating statistics for {} data points", data.len()));

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = self.calculate_mean(data);
        let median = self.calculate_median(&sorted_data);
        let mode = self.calculate_mode(data);
        let variance = self.calculate_variance(data, mean);
        let standard_deviation = variance.sqrt();
        let min = sorted_data[0];
        let max = sorted_data[sorted_data.len() - 1];
        let range = max - min;
        let quartiles = self.calculate_quartiles(&sorted_data);
        let skewness = self.calculate_skewness(data, mean, standard_deviation);
        let kurtosis = self.calculate_kurtosis(data, mean, standard_deviation);

        let result = StatisticalResult {
            mean,
            median,
            mode,
            standard_deviation,
            variance,
            min,
            max,
            range,
            quartiles,
            skewness,
            kurtosis,
            count: data.len(),
        };

        self.logger.info("Statistical calculations completed successfully");
        Ok(result)
    }

    /// Calculate the mean (average) of the dataset
    pub fn calculate_mean(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        data.iter().sum::<f64>() / data.len() as f64
    }

    /// Calculate the median of the dataset
    pub fn calculate_median(&self, sorted_data: &[f64]) -> f64 {
        if sorted_data.is_empty() {
            return 0.0;
        }

        let len = sorted_data.len();
        if len % 2 == 0 {
            (sorted_data[len / 2 - 1] + sorted_data[len / 2]) / 2.0
        } else {
            sorted_data[len / 2]
        }
    }

    /// Calculate the mode of the dataset
    pub fn calculate_mode(&self, data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            return None;
        }

        let mut frequency_map = HashMap::new();
        for &value in data {
            let count = frequency_map.entry(value.to_bits()).or_insert(0);
            *count += 1;
        }

        let max_frequency = frequency_map.values().max()?;
        if *max_frequency == 1 {
            return None; // No mode if all values appear only once
        }

        frequency_map
            .iter()
            .find(|(_, &count)| count == *max_frequency)
            .map(|(&bits, _)| f64::from_bits(bits))
    }

    /// Calculate the variance of the dataset
    pub fn calculate_variance(&self, data: &[f64], mean: f64) -> f64 {
        if data.len() <= 1 {
            return 0.0;
        }

        let sum_squared_differences: f64 = data
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum();

        sum_squared_differences / (data.len() - 1) as f64
    }

    /// Calculate quartiles and interquartile range
    pub fn calculate_quartiles(&self, sorted_data: &[f64]) -> Quartiles {
        let len = sorted_data.len();
        if len == 0 {
            return Quartiles {
                q1: 0.0,
                q2: 0.0,
                q3: 0.0,
                iqr: 0.0,
            };
        }

        let q2 = self.calculate_median(sorted_data);

        let (q1, q3) = if len == 1 {
            (sorted_data[0], sorted_data[0])
        } else if len == 2 {
            (sorted_data[0], sorted_data[1])
        } else {
            let mid = len / 2;
            let q1_data = if len % 2 == 0 {
                &sorted_data[..mid]
            } else {
                &sorted_data[..=mid]
            };
            let q3_data = if len % 2 == 0 {
                &sorted_data[mid..]
            } else {
                &sorted_data[mid..]
            };

            (self.calculate_median(q1_data), self.calculate_median(q3_data))
        };

        let iqr = q3 - q1;

        Quartiles { q1, q2, iqr, q3 }
    }

    /// Calculate the skewness of the dataset
    pub fn calculate_skewness(&self, data: &[f64], mean: f64, std_dev: f64) -> f64 {
        if data.len() < 3 || std_dev == 0.0 {
            return 0.0;
        }

        let n = data.len() as f64;
        let sum_cubed_differences: f64 = data
            .iter()
            .map(|&x| ((x - mean) / std_dev).powi(3))
            .sum();

        (n / ((n - 1.0) * (n - 2.0))) * sum_cubed_differences
    }

    /// Calculate the kurtosis of the dataset
    pub fn calculate_kurtosis(&self, data: &[f64], mean: f64, std_dev: f64) -> f64 {
        if data.len() < 4 || std_dev == 0.0 {
            return 0.0;
        }

        let n = data.len() as f64;
        let sum_fourth_differences: f64 = data
            .iter()
            .map(|&x| ((x - mean) / std_dev).powi(4))
            .sum();

        let kurtosis = (n * (n + 1.0) / ((n - 1.0) * (n - 2.0) * (n - 3.0))) * sum_fourth_differences
            - (3.0 * (n - 1.0).powi(2) / ((n - 2.0) * (n - 3.0)));

        kurtosis
    }

    /// Calculate the correlation coefficient between two datasets
    pub fn calculate_correlation(&self, x: &[f64], y: &[f64]) -> Result<f64> {
        if x.len() != y.len() {
            return Err(Error::metrics("Datasets must have the same length for correlation calculation".to_string()));
        }

        if x.is_empty() {
            return Err(Error::metrics("Cannot calculate correlation for empty datasets".to_string()));
        }

        let n = x.len() as f64;
        let mean_x = self.calculate_mean(x);
        let mean_y = self.calculate_mean(y);

        let numerator: f64 = x.iter()
            .zip(y.iter())
            .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
            .sum();

        let sum_sq_x: f64 = x.iter().map(|&xi| (xi - mean_x).powi(2)).sum();
        let sum_sq_y: f64 = y.iter().map(|&yi| (yi - mean_y).powi(2)).sum();

        let denominator = (sum_sq_x * sum_sq_y).sqrt();

        if denominator == 0.0 {
            return Ok(0.0);
        }

        Ok(numerator / denominator)
    }

    /// Calculate the coefficient of variation
    pub fn calculate_coefficient_of_variation(&self, data: &[f64]) -> Result<f64> {
        if data.is_empty() {
            return Err(Error::metrics("Cannot calculate coefficient of variation for empty dataset".to_string()));
        }

        let mean = self.calculate_mean(data);
        if mean == 0.0 {
            return Err(Error::metrics("Cannot calculate coefficient of variation when mean is zero".to_string()));
        }

        let variance = self.calculate_variance(data, mean);
        let std_dev = variance.sqrt();

        Ok(std_dev / mean.abs())
    }

    /// Detect outliers using the IQR method
    pub fn detect_outliers_iqr(&self, data: &[f64]) -> Result<Vec<f64>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let quartiles = self.calculate_quartiles(&sorted_data);
        let lower_bound = quartiles.q1 - 1.5 * quartiles.iqr;
        let upper_bound = quartiles.q3 + 1.5 * quartiles.iqr;

        let outliers: Vec<f64> = data.iter()
            .filter(|&&x| x < lower_bound || x > upper_bound)
            .copied()
            .collect();

        Ok(outliers)
    }

    /// Calculate the confidence interval for the mean
    pub fn calculate_confidence_interval(&self, data: &[f64], confidence_level: f64) -> Result<(f64, f64)> {
        if data.is_empty() {
            return Err(Error::metrics("Cannot calculate confidence interval for empty dataset".to_string()));
        }

        let mean = self.calculate_mean(data);
        let variance = self.calculate_variance(data, mean);
        let std_dev = variance.sqrt();
        let n = data.len() as f64;

        // For simplicity, using normal distribution approximation
        // In practice, you might want to use t-distribution for small samples
        let z_score = match confidence_level {
            0.90 => 1.645,
            0.95 => 1.96,
            0.99 => 2.576,
            _ => return Err(Error::metrics("Unsupported confidence level. Use 0.90, 0.95, or 0.99".to_string())),
        };

        let margin_of_error = z_score * (std_dev / n.sqrt());
        let lower_bound = mean - margin_of_error;
        let upper_bound = mean + margin_of_error;

        Ok((lower_bound, upper_bound))
    }
}

impl Default for StatisticalCalculator {
    fn default() -> Self {
        Self::new()
    }
}
