//! Growth rate calculations
//!
//! This module provides growth rate analysis including compound annual growth rate (CAGR),
//! period-over-period growth, and growth trend analysis.

use crate::error::{Error, Result};
use crate::logging::Logger;

/// Growth rate calculation result
#[derive(Debug, Clone)]
pub struct GrowthRate {
    pub absolute_growth: f64,
    pub percentage_growth: f64,
    pub cagr: Option<f64>,
    pub period_growth: Vec<f64>,
    pub growth_trend: GrowthTrend,
}

/// Growth trend classification
#[derive(Debug, Clone, PartialEq)]
pub enum GrowthTrend {
    Accelerating,
    Decelerating,
    Stable,
    Volatile,
    Declining,
}

/// Growth metrics for comprehensive analysis
#[derive(Debug, Clone)]
pub struct GrowthMetrics {
    pub total_growth: f64,
    pub average_period_growth: f64,
    pub growth_volatility: f64,
    pub growth_consistency: f64,
    pub peak_growth: f64,
    pub trough_growth: f64,
    pub growth_acceleration: f64,
}

/// Growth calculator for analyzing growth patterns
pub struct GrowthCalculator {
    logger: Logger,
}

impl GrowthCalculator {
    /// Create a new growth calculator
    pub fn new() -> Self {
        Self {
            logger: Logger::new("growth_calculator"),
        }
    }

    /// Calculate comprehensive growth metrics for a time series
    pub fn calculate_growth_rate(&self, data: &[f64], periods: Option<&[f64]>) -> Result<GrowthRate> {
        if data.len() < 2 {
            return Err(Error::metrics("Insufficient data points for growth calculation (minimum 2 required)".to_string()));
        }

        self.logger.info(&format!("Calculating growth rate for {} data points", data.len()));

        let first_value = data[0];
        let last_value = data[data.len() - 1];
        let absolute_growth = last_value - first_value;
        let percentage_growth = if first_value != 0.0 {
            (absolute_growth / first_value) * 100.0
        } else {
            0.0
        };

        let cagr = if data.len() > 1 {
            Some(self.calculate_cagr(first_value, last_value, data.len() as f64 - 1.0))
        } else {
            None
        };

        let period_growth = self.calculate_period_growth(data);
        let growth_trend = self.analyze_growth_trend(&period_growth);

        let result = GrowthRate {
            absolute_growth,
            percentage_growth,
            cagr,
            period_growth,
            growth_trend,
        };

        self.logger.info(&format!("Growth calculation completed: {:.2}% total growth", percentage_growth));
        Ok(result)
    }

    /// Calculate Compound Annual Growth Rate (CAGR)
    pub fn calculate_cagr(&self, initial_value: f64, final_value: f64, periods: f64) -> f64 {
        if initial_value <= 0.0 || final_value <= 0.0 || periods <= 0.0 {
            return 0.0;
        }

        ((final_value / initial_value).powf(1.0 / periods) - 1.0) * 100.0
    }

    /// Calculate period-over-period growth rates
    fn calculate_period_growth(&self, data: &[f64]) -> Vec<f64> {
        let mut period_growth = Vec::new();

        for i in 1..data.len() {
            let previous_value = data[i - 1];
            let current_value = data[i];

            if previous_value != 0.0 {
                let growth = ((current_value - previous_value) / previous_value) * 100.0;
                period_growth.push(growth);
            } else {
                period_growth.push(0.0);
            }
        }

        period_growth
    }

    /// Analyze the trend of growth rates
    fn analyze_growth_trend(&self, period_growth: &[f64]) -> GrowthTrend {
        if period_growth.is_empty() {
            return GrowthTrend::Stable;
        }

        let mean_growth = period_growth.iter().sum::<f64>() / period_growth.len() as f64;
        let growth_volatility = self.calculate_growth_volatility(period_growth);

        // High volatility indicates unstable growth
        if growth_volatility > 50.0 {
            return GrowthTrend::Volatile;
        }

        // Check for acceleration/deceleration
        if period_growth.len() >= 3 {
            let recent_avg = period_growth[period_growth.len() - 3..].iter().sum::<f64>() / 3.0;
            let earlier_avg = if period_growth.len() >= 6 {
                period_growth[period_growth.len() - 6..period_growth.len() - 3].iter().sum::<f64>() / 3.0
            } else {
                period_growth[..period_growth.len() - 3].iter().sum::<f64>() / (period_growth.len() - 3) as f64
            };

            if recent_avg > earlier_avg + 5.0 {
                return GrowthTrend::Accelerating;
            } else if recent_avg < earlier_avg - 5.0 {
                return GrowthTrend::Decelerating;
            }
        }

        // Check for declining growth
        if mean_growth < -5.0 {
            return GrowthTrend::Declining;
        }

        GrowthTrend::Stable
    }

    /// Calculate growth volatility
    fn calculate_growth_volatility(&self, period_growth: &[f64]) -> f64 {
        if period_growth.len() < 2 {
            return 0.0;
        }

        let mean = period_growth.iter().sum::<f64>() / period_growth.len() as f64;
        let variance: f64 = period_growth.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (period_growth.len() - 1) as f64;

        variance.sqrt()
    }

    /// Calculate comprehensive growth metrics
    pub fn calculate_growth_metrics(&self, data: &[f64]) -> Result<GrowthMetrics> {
        if data.len() < 2 {
            return Err(Error::metrics("Insufficient data points for growth metrics calculation".to_string()));
        }

        let period_growth = self.calculate_period_growth(data);
        let total_growth = if data[0] != 0.0 {
            ((data[data.len() - 1] - data[0]) / data[0]) * 100.0
        } else {
            0.0
        };

        let average_period_growth = if !period_growth.is_empty() {
            period_growth.iter().sum::<f64>() / period_growth.len() as f64
        } else {
            0.0
        };

        let growth_volatility = self.calculate_growth_volatility(&period_growth);
        let growth_consistency = self.calculate_growth_consistency(&period_growth);
        let peak_growth = period_growth.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let trough_growth = period_growth.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let growth_acceleration = self.calculate_growth_acceleration(&period_growth);

        Ok(GrowthMetrics {
            total_growth,
            average_period_growth,
            growth_volatility,
            growth_consistency,
            peak_growth,
            trough_growth,
            growth_acceleration,
        })
    }

    /// Calculate growth consistency (inverse of volatility)
    fn calculate_growth_consistency(&self, period_growth: &[f64]) -> f64 {
        if period_growth.is_empty() {
            return 0.0;
        }

        let volatility = self.calculate_growth_volatility(period_growth);
        // Convert volatility to consistency score (0-100)
        (100.0 - volatility.min(100.0)).max(0.0)
    }

    /// Calculate growth acceleration
    fn calculate_growth_acceleration(&self, period_growth: &[f64]) -> f64 {
        if period_growth.len() < 3 {
            return 0.0;
        }

        let recent_avg = period_growth[period_growth.len() - 3..].iter().sum::<f64>() / 3.0;
        let earlier_avg = if period_growth.len() >= 6 {
            period_growth[period_growth.len() - 6..period_growth.len() - 3].iter().sum::<f64>() / 3.0
        } else {
            period_growth[..period_growth.len() - 3].iter().sum::<f64>() / (period_growth.len() - 3) as f64
        };

        recent_avg - earlier_avg
    }

    /// Calculate year-over-year growth
    pub fn calculate_yoy_growth(&self, data: &[f64], year_periods: &[f64]) -> Result<Vec<f64>> {
        if data.len() != year_periods.len() {
            return Err(Error::metrics("Data and year periods must have the same length".to_string()));
        }

        if data.len() < 2 {
            return Ok(Vec::new());
        }

        let mut yoy_growth = Vec::new();

        for i in 1..data.len() {
            let current_year = year_periods[i];
            let previous_year = year_periods[i - 1];

            // Find the same period in the previous year
            if let Some(prev_index) = year_periods.iter().position(|&y| y == current_year - 1.0) {
                if prev_index < data.len() {
                    let current_value = data[i];
                    let previous_value = data[prev_index];

                    if previous_value != 0.0 {
                        let growth = ((current_value - previous_value) / previous_value) * 100.0;
                        yoy_growth.push(growth);
                    } else {
                        yoy_growth.push(0.0);
                    }
                } else {
                    yoy_growth.push(0.0);
                }
            } else {
                yoy_growth.push(0.0);
            }
        }

        Ok(yoy_growth)
    }

    /// Calculate quarter-over-quarter growth
    pub fn calculate_qoq_growth(&self, data: &[f64], quarter_periods: &[f64]) -> Result<Vec<f64>> {
        if data.len() != quarter_periods.len() {
            return Err(Error::metrics("Data and quarter periods must have the same length".to_string()));
        }

        if data.len() < 2 {
            return Ok(Vec::new());
        }

        let mut qoq_growth = Vec::new();

        for i in 1..data.len() {
            let current_quarter = quarter_periods[i];
            let previous_quarter = quarter_periods[i - 1];

            // Find the same quarter in the previous period
            if let Some(prev_index) = quarter_periods.iter().position(|&q| q == current_quarter - 1.0) {
                if prev_index < data.len() {
                    let current_value = data[i];
                    let previous_value = data[prev_index];

                    if previous_value != 0.0 {
                        let growth = ((current_value - previous_value) / previous_value) * 100.0;
                        qoq_growth.push(growth);
                    } else {
                        qoq_growth.push(0.0);
                    }
                } else {
                    qoq_growth.push(0.0);
                }
            } else {
                qoq_growth.push(0.0);
            }
        }

        Ok(qoq_growth)
    }
}

impl Default for GrowthCalculator {
    fn default() -> Self {
        Self::new()
    }
}
