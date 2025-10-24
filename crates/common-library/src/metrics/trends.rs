//! Trend analysis algorithms
//!
//! This module provides trend detection and analysis algorithms including
//! linear regression, moving averages, and trend classification.

use crate::error::{Error, Result};
use crate::logging::Logger;

/// Trend analysis result
#[derive(Debug, Clone)]
pub struct TrendResult {
    pub trend_type: TrendType,
    pub slope: f64,
    pub r_squared: f64,
    pub p_value: f64,
    pub confidence: f64,
    pub trend_strength: TrendStrength,
    pub data_points: usize,
}

/// Type of trend detected
#[derive(Debug, Clone, PartialEq)]
pub enum TrendType {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
    Cyclical,
}

/// Strength of the trend
#[derive(Debug, Clone, PartialEq)]
pub enum TrendStrength {
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

/// Linear regression result
#[derive(Debug, Clone)]
pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub standard_error: f64,
}

/// Trend analyzer for detecting patterns in data
pub struct TrendAnalyzer {
    logger: Logger,
}

impl TrendAnalyzer {
    /// Create a new trend analyzer
    pub fn new() -> Self {
        Self {
            logger: Logger::new("trend_analyzer"),
        }
    }

    /// Analyze trends in a dataset
    pub fn analyze_trend(&self, data: &[f64]) -> Result<TrendResult> {
        if data.len() < 3 {
            return Err(Error::metrics("Insufficient data points for trend analysis (minimum 3 required)".to_string()));
        }

        self.logger.info(&format!("Analyzing trends in {} data points", data.len()));

        let linear_regression = self.perform_linear_regression(data)?;
        let trend_type = self.classify_trend(&linear_regression, data);
        let trend_strength = self.assess_trend_strength(&linear_regression);
        let confidence = self.calculate_confidence(&linear_regression, data.len());

        let result = TrendResult {
            trend_type,
            slope: linear_regression.slope,
            r_squared: linear_regression.r_squared,
            p_value: self.calculate_p_value(&linear_regression, data.len()),
            confidence,
            trend_strength,
            data_points: data.len(),
        };

        self.logger.info(&format!("Trend analysis completed: {:?}", result.trend_type));
        Ok(result)
    }

    /// Perform linear regression on the dataset
    pub fn perform_linear_regression(&self, data: &[f64]) -> Result<LinearRegression> {
        if data.len() < 2 {
            return Err(Error::metrics("Insufficient data points for linear regression".to_string()));
        }

        let n = data.len() as f64;
        let x_values: Vec<f64> = (0..data.len()).map(|i| i as f64).collect();

        let sum_x: f64 = x_values.iter().sum();
        let sum_y: f64 = data.iter().sum();
        let sum_xy: f64 = x_values.iter().zip(data.iter()).map(|(x, y)| x * y).sum();
        let sum_x2: f64 = x_values.iter().map(|x| x * x).sum();
        let sum_y2: f64 = data.iter().map(|y| y * y).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared
        let y_mean = sum_y / n;
        let ss_tot: f64 = data.iter().map(|&y| (y - y_mean).powi(2)).sum();
        let ss_res: f64 = x_values.iter()
            .zip(data.iter())
            .map(|(x, y)| {
                let y_pred = slope * x + intercept;
                (y - y_pred).powi(2)
            })
            .sum();

        let r_squared = if ss_tot == 0.0 { 1.0 } else { 1.0 - (ss_res / ss_tot) };

        // Calculate standard error
        let standard_error = if n > 2.0 {
            (ss_res / (n - 2.0)).sqrt()
        } else {
            0.0
        };

        Ok(LinearRegression {
            slope,
            intercept,
            r_squared,
            standard_error,
        })
    }

    /// Classify the type of trend
    fn classify_trend(&self, regression: &LinearRegression, data: &[f64]) -> TrendType {
        let slope = regression.slope;
        let r_squared = regression.r_squared;
        let volatility = self.calculate_volatility(data);

        // Check for cyclical patterns
        if self.detect_cyclical_pattern(data) {
            return TrendType::Cyclical;
        }

        // High volatility indicates unstable trend
        if volatility > 1.0 {
            return TrendType::Volatile;
        }

        // Classify based on slope and R-squared
        if r_squared < 0.3 {
            TrendType::Stable
        } else if slope > 0.01 {
            TrendType::Increasing
        } else if slope < -0.01 {
            TrendType::Decreasing
        } else {
            TrendType::Stable
        }
    }

    /// Assess the strength of the trend
    fn assess_trend_strength(&self, regression: &LinearRegression) -> TrendStrength {
        let r_squared = regression.r_squared;

        match r_squared {
            r if r >= 0.8 => TrendStrength::VeryStrong,
            r if r >= 0.6 => TrendStrength::Strong,
            r if r >= 0.4 => TrendStrength::Moderate,
            _ => TrendStrength::Weak,
        }
    }

    /// Calculate confidence level
    fn calculate_confidence(&self, regression: &LinearRegression, n: usize) -> f64 {
        let r_squared = regression.r_squared;
        let n_f64 = n as f64;

        // Simple confidence calculation based on R-squared and sample size
        let base_confidence = r_squared;
        let size_factor = (n_f64 / 100.0).min(1.0);

        (base_confidence * size_factor).min(1.0)
    }

    /// Calculate p-value (simplified approximation)
    fn calculate_p_value(&self, regression: &LinearRegression, n: usize) -> f64 {
        let r_squared = regression.r_squared;
        let n_f64 = n as f64;

        if n_f64 < 3.0 {
            return 1.0;
        }

        // Simplified p-value calculation
        let t_stat = (r_squared.sqrt() * (n_f64 - 2.0).sqrt()) / (1.0 - r_squared).sqrt();

        // Approximate p-value (in practice, you'd use proper t-distribution)
        if t_stat.abs() > 2.0 {
            0.05
        } else if t_stat.abs() > 1.5 {
            0.1
        } else {
            0.2
        }
    }

    /// Calculate volatility of the dataset
    fn calculate_volatility(&self, data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }

        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance: f64 = data.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (data.len() - 1) as f64;

        let std_dev = variance.sqrt();
        if mean == 0.0 { 0.0 } else { std_dev / mean.abs() }
    }

    /// Detect cyclical patterns in the data
    fn detect_cyclical_pattern(&self, data: &[f64]) -> bool {
        if data.len() < 10 {
            return false;
        }

        // Simple cyclical detection using autocorrelation
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance: f64 = data.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / data.len() as f64;

        if variance == 0.0 {
            return false;
        }

        // Check for autocorrelation at different lags
        let max_lag = (data.len() / 4).min(5);
        let mut autocorrelations = Vec::new();

        for lag in 2..=max_lag {
            let autocorr = self.calculate_autocorrelation(data, lag, mean, variance);
            autocorrelations.push(autocorr.abs());
        }

        // Only consider it cyclical if multiple lags show high autocorrelation
        let significant_correlations = autocorrelations.iter().filter(|&&corr| corr > 0.7).count();
        significant_correlations >= 2
    }

    /// Calculate autocorrelation at a given lag
    fn calculate_autocorrelation(&self, data: &[f64], lag: usize, mean: f64, variance: f64) -> f64 {
        if lag >= data.len() {
            return 0.0;
        }

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for i in 0..(data.len() - lag) {
            numerator += (data[i] - mean) * (data[i + lag] - mean);
        }

        for i in 0..data.len() {
            denominator += (data[i] - mean).powi(2);
        }

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    /// Calculate moving average
    pub fn calculate_moving_average(&self, data: &[f64], window_size: usize) -> Result<Vec<f64>> {
        if window_size == 0 || window_size > data.len() {
            return Err(Error::metrics("Invalid window size for moving average".to_string()));
        }

        let mut moving_averages = Vec::new();

        for i in (window_size - 1)..data.len() {
            let start_idx = i.saturating_sub(window_size - 1);
            let window_sum: f64 = data[start_idx..=i].iter().sum();
            let average = window_sum / window_size as f64;
            moving_averages.push(average);
        }

        Ok(moving_averages)
    }

    /// Calculate exponential moving average
    pub fn calculate_exponential_moving_average(&self, data: &[f64], alpha: f64) -> Result<Vec<f64>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        if alpha <= 0.0 || alpha > 1.0 {
            return Err(Error::metrics("Alpha must be between 0 and 1 for exponential moving average".to_string()));
        }

        let mut ema_values = Vec::new();
        let mut ema = data[0];

        for &value in data {
            ema = alpha * value + (1.0 - alpha) * ema;
            ema_values.push(ema);
        }

        Ok(ema_values)
    }
}

impl Default for TrendAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
