//! Data normalization methods
//!
//! This module provides various data normalization techniques including
//! min-max scaling, z-score normalization, and robust scaling.

use crate::error::{Error, Result};
use crate::logging::Logger;

/// Normalization method types
#[derive(Debug, Clone, PartialEq)]
pub enum NormalizationMethod {
    MinMax,
    ZScore,
    Robust,
    UnitVector,
    DecimalScaling,
}

/// Normalization result
#[derive(Debug, Clone)]
pub struct NormalizationResult {
    pub normalized_data: Vec<f64>,
    pub method: NormalizationMethod,
    pub parameters: NormalizationParameters,
    pub original_range: (f64, f64),
    pub normalized_range: (f64, f64),
}

/// Parameters used for normalization
#[derive(Debug, Clone)]
pub struct NormalizationParameters {
    pub min_value: f64,
    pub max_value: f64,
    pub mean: f64,
    pub standard_deviation: f64,
    pub median: f64,
    pub iqr: f64,
}

/// Data normalizer for various normalization techniques
pub struct DataNormalizer {
    logger: Logger,
}

impl DataNormalizer {
    /// Create a new data normalizer
    pub fn new() -> Self {
        Self {
            logger: Logger::new("data_normalizer"),
        }
    }

    /// Normalize data using the specified method
    pub fn normalize(
        &self,
        data: &[f64],
        method: NormalizationMethod,
    ) -> Result<NormalizationResult> {
        if data.is_empty() {
            return Err(Error::metrics("Cannot normalize empty dataset".to_string()));
        }

        self.logger.info(&format!(
            "Normalizing {} data points using {:?}",
            data.len(),
            method
        ));

        let parameters = self.calculate_parameters(data);
        let normalized_data = match method {
            NormalizationMethod::MinMax => self.min_max_normalize(data, &parameters),
            NormalizationMethod::ZScore => self.z_score_normalize(data, &parameters),
            NormalizationMethod::Robust => self.robust_normalize(data, &parameters),
            NormalizationMethod::UnitVector => self.unit_vector_normalize(data),
            NormalizationMethod::DecimalScaling => self.decimal_scaling_normalize(data),
        };

        let original_range = (parameters.min_value, parameters.max_value);
        let normalized_range = if !normalized_data.is_empty() {
            let min_norm = normalized_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max_norm = normalized_data
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            (min_norm, max_norm)
        } else {
            (0.0, 0.0)
        };

        let result = NormalizationResult {
            normalized_data,
            method,
            parameters,
            original_range,
            normalized_range,
        };

        self.logger
            .info("Data normalization completed successfully");
        Ok(result)
    }

    /// Calculate normalization parameters
    fn calculate_parameters(&self, data: &[f64]) -> NormalizationParameters {
        let min_value = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_value = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let mean = data.iter().sum::<f64>() / data.len() as f64;

        let variance =
            data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (data.len() - 1) as f64;
        let standard_deviation = variance.sqrt();

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if sorted_data.len() % 2 == 0 {
            (sorted_data[sorted_data.len() / 2 - 1] + sorted_data[sorted_data.len() / 2]) / 2.0
        } else {
            sorted_data[sorted_data.len() / 2]
        };

        // Calculate IQR
        let q1_index = sorted_data.len() / 4;
        let q3_index = (3 * sorted_data.len()) / 4;
        let q1 = sorted_data[q1_index];
        let q3 = sorted_data[q3_index];
        let iqr = q3 - q1;

        NormalizationParameters {
            min_value,
            max_value,
            mean,
            standard_deviation,
            median,
            iqr,
        }
    }

    /// Min-Max normalization (0-1 scaling)
    fn min_max_normalize(&self, data: &[f64], params: &NormalizationParameters) -> Vec<f64> {
        let range = params.max_value - params.min_value;
        if range == 0.0 {
            return vec![0.5; data.len()];
        }

        data.iter()
            .map(|&x| (x - params.min_value) / range)
            .collect()
    }

    /// Z-score normalization (mean=0, std=1)
    fn z_score_normalize(&self, data: &[f64], params: &NormalizationParameters) -> Vec<f64> {
        if params.standard_deviation == 0.0 {
            return vec![0.0; data.len()];
        }

        data.iter()
            .map(|&x| (x - params.mean) / params.standard_deviation)
            .collect()
    }

    /// Robust normalization using median and IQR
    fn robust_normalize(&self, data: &[f64], params: &NormalizationParameters) -> Vec<f64> {
        if params.iqr == 0.0 {
            return vec![0.0; data.len()];
        }

        data.iter()
            .map(|&x| (x - params.median) / params.iqr)
            .collect()
    }

    /// Unit vector normalization (L2 norm = 1)
    fn unit_vector_normalize(&self, data: &[f64]) -> Vec<f64> {
        let magnitude = data.iter().map(|&x| x * x).sum::<f64>().sqrt();

        if magnitude == 0.0 {
            return vec![0.0; data.len()];
        }

        data.iter().map(|&x| x / magnitude).collect()
    }

    /// Decimal scaling normalization
    fn decimal_scaling_normalize(&self, data: &[f64]) -> Vec<f64> {
        let max_abs = data
            .iter()
            .map(|&x| x.abs())
            .fold(f64::NEG_INFINITY, |a, b| a.max(b));

        if max_abs == 0.0 {
            return vec![0.0; data.len()];
        }

        let j = max_abs.log10().ceil() as i32;
        let divisor = 10_f64.powi(j);

        data.iter().map(|&x| x / divisor).collect()
    }

    /// Denormalize data back to original scale
    pub fn denormalize(
        &self,
        normalized_data: &[f64],
        original_parameters: &NormalizationParameters,
        method: NormalizationMethod,
    ) -> Result<Vec<f64>> {
        if normalized_data.is_empty() {
            return Ok(Vec::new());
        }

        self.logger.info(&format!(
            "Denormalizing {} data points using {:?}",
            normalized_data.len(),
            method
        ));

        let denormalized_data = match method {
            NormalizationMethod::MinMax => {
                let range = original_parameters.max_value - original_parameters.min_value;
                normalized_data
                    .iter()
                    .map(|&x| x * range + original_parameters.min_value)
                    .collect()
            }
            NormalizationMethod::ZScore => normalized_data
                .iter()
                .map(|&x| x * original_parameters.standard_deviation + original_parameters.mean)
                .collect(),
            NormalizationMethod::Robust => normalized_data
                .iter()
                .map(|&x| x * original_parameters.iqr + original_parameters.median)
                .collect(),
            NormalizationMethod::UnitVector => {
                // For unit vector, we need the original magnitude
                // This is a simplified approach - in practice, you'd store the original magnitude
                normalized_data.to_vec()
            }
            NormalizationMethod::DecimalScaling => {
                let max_abs = original_parameters
                    .max_value
                    .abs()
                    .max(original_parameters.min_value.abs());
                let j = max_abs.log10().ceil() as i32;
                let divisor = 10_f64.powi(j);
                normalized_data.iter().map(|&x| x * divisor).collect()
            }
        };

        self.logger
            .info("Data denormalization completed successfully");
        Ok(denormalized_data)
    }

    /// Normalize multiple datasets consistently
    pub fn normalize_multiple(
        &self,
        datasets: &[Vec<f64>],
        method: NormalizationMethod,
    ) -> Result<Vec<NormalizationResult>> {
        if datasets.is_empty() {
            return Ok(Vec::new());
        }

        self.logger.info(&format!(
            "Normalizing {} datasets using {:?}",
            datasets.len(),
            method
        ));

        // Calculate global parameters across all datasets
        let all_data: Vec<f64> = datasets.iter().flatten().copied().collect();
        let global_parameters = self.calculate_parameters(&all_data);

        let mut results = Vec::new();

        for dataset in datasets {
            let normalized_data = match method {
                NormalizationMethod::MinMax => self.min_max_normalize(dataset, &global_parameters),
                NormalizationMethod::ZScore => self.z_score_normalize(dataset, &global_parameters),
                NormalizationMethod::Robust => self.robust_normalize(dataset, &global_parameters),
                NormalizationMethod::UnitVector => self.unit_vector_normalize(dataset),
                NormalizationMethod::DecimalScaling => self.decimal_scaling_normalize(dataset),
            };

            let original_range = (global_parameters.min_value, global_parameters.max_value);
            let normalized_range = if !normalized_data.is_empty() {
                let min_norm = normalized_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max_norm = normalized_data
                    .iter()
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                (min_norm, max_norm)
            } else {
                (0.0, 0.0)
            };

            results.push(NormalizationResult {
                normalized_data,
                method: method.clone(),
                parameters: global_parameters.clone(),
                original_range,
                normalized_range,
            });
        }

        self.logger
            .info("Multiple dataset normalization completed successfully");
        Ok(results)
    }

    /// Check if data is already normalized
    pub fn is_normalized(&self, data: &[f64], method: NormalizationMethod) -> bool {
        if data.is_empty() {
            return false;
        }

        match method {
            NormalizationMethod::MinMax => {
                let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                min_val >= 0.0 && max_val <= 1.0
            }
            NormalizationMethod::ZScore => {
                let mean = data.iter().sum::<f64>() / data.len() as f64;
                let variance =
                    data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
                let std_dev = variance.sqrt();
                mean.abs() < 0.1 && (std_dev - 1.0).abs() < 0.1
            }
            NormalizationMethod::Robust => {
                let median = self.calculate_median(data);
                let iqr = self.calculate_iqr(data);
                median.abs() < 0.1 && iqr.abs() < 0.1
            }
            NormalizationMethod::UnitVector => {
                let magnitude = data.iter().map(|&x| x * x).sum::<f64>().sqrt();
                (magnitude - 1.0).abs() < 0.1
            }
            NormalizationMethod::DecimalScaling => {
                let max_abs = data
                    .iter()
                    .map(|&x| x.abs())
                    .fold(f64::NEG_INFINITY, |a, b| a.max(b));
                max_abs <= 1.0
            }
        }
    }

    /// Calculate median
    fn calculate_median(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sorted_data.len() % 2 == 0 {
            (sorted_data[sorted_data.len() / 2 - 1] + sorted_data[sorted_data.len() / 2]) / 2.0
        } else {
            sorted_data[sorted_data.len() / 2]
        }
    }

    /// Calculate IQR
    fn calculate_iqr(&self, data: &[f64]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let q1_index = sorted_data.len() / 4;
        let q3_index = (3 * sorted_data.len()) / 4;
        let q1 = sorted_data[q1_index];
        let q3 = sorted_data[q3_index];

        q3 - q1
    }
}

impl Default for DataNormalizer {
    fn default() -> Self {
        Self::new()
    }
}
