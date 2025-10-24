//! Metrics Tests
//!
//! Comprehensive test coverage for metrics components:
//! - Statistical calculations
//! - Trend analysis
//! - Growth rate calculations
//! - Performance metrics
//! - Data normalization

use common_library::metrics::growth::GrowthTrend;
use common_library::metrics::trends::TrendStrength;
use common_library::metrics::{
    DataNormalizer, GrowthCalculator, NormalizationMethod, PerformanceAnalyzer,
    StatisticalCalculator, TrendAnalyzer, TrendType,
};
use std::time::Duration;

#[tokio::test]
async fn test_statistical_calculations() {
    // Test: Statistical calculations work correctly
    let calculator = StatisticalCalculator::new();
    let test_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

    // Test basic statistics
    let mean = calculator.calculate_mean(&test_data);
    assert_eq!(mean, 5.5, "Mean should be 5.5");

    let median = calculator.calculate_median(&test_data);
    assert_eq!(median, 5.5, "Median should be 5.5");

    // Test comprehensive statistics
    let stats = calculator
        .calculate_statistics(&test_data)
        .expect("Should calculate statistics");
    assert_eq!(stats.mean, 5.5, "Statistics mean should be 5.5");
    assert_eq!(stats.median, 5.5, "Statistics median should be 5.5");
    assert_eq!(stats.min, 1.0, "Min should be 1.0");
    assert_eq!(stats.max, 10.0, "Max should be 10.0");
    assert_eq!(stats.count, 10, "Count should be 10");

    // Test correlation
    let x_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let correlation = calculator
        .calculate_correlation(&x_data, &y_data)
        .expect("Should calculate correlation");
    assert!(
        (correlation - 1.0).abs() < 0.001,
        "Perfect positive correlation should be ~1.0"
    );

    // Test coefficient of variation
    let cv = calculator
        .calculate_coefficient_of_variation(&test_data)
        .expect("Should calculate CV");
    assert!(cv > 0.0, "Coefficient of variation should be positive");
}

#[tokio::test]
async fn test_trend_analysis() {
    // Test: Trend analysis works correctly
    let analyzer = TrendAnalyzer::new();

    // Test increasing trend
    let increasing_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let trend_result = analyzer
        .analyze_trend(&increasing_data)
        .expect("Should analyze trend");
    assert_eq!(
        trend_result.trend_type,
        TrendType::Increasing,
        "Should detect increasing trend"
    );
    assert!(
        trend_result.slope > 0.0,
        "Slope should be positive for increasing trend"
    );
    assert!(
        trend_result.r_squared > 0.8,
        "R-squared should be high for strong trend"
    );

    // Test decreasing trend
    let decreasing_data = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let trend_result = analyzer
        .analyze_trend(&decreasing_data)
        .expect("Should analyze trend");
    assert_eq!(
        trend_result.trend_type,
        TrendType::Decreasing,
        "Should detect decreasing trend"
    );
    assert!(
        trend_result.slope < 0.0,
        "Slope should be negative for decreasing trend"
    );

    // Test stable trend
    let stable_data = vec![5.0, 5.1, 4.9, 5.0, 5.1, 4.9, 5.0, 5.1, 4.9, 5.0];
    let trend_result = analyzer
        .analyze_trend(&stable_data)
        .expect("Should analyze trend");
    assert_eq!(
        trend_result.trend_type,
        TrendType::Stable,
        "Should detect stable trend"
    );

    // Test moving average
    let moving_avg = analyzer
        .calculate_moving_average(&increasing_data, 3)
        .expect("Should calculate moving average");
    assert_eq!(
        moving_avg.len(),
        8,
        "Moving average should have correct length"
    );
    assert!(
        moving_avg[0] < moving_avg[1],
        "Moving average should be increasing for increasing data"
    );

    // Test exponential moving average
    let ema = analyzer
        .calculate_exponential_moving_average(&increasing_data, 0.3)
        .expect("Should calculate EMA");
    assert_eq!(ema.len(), 10, "EMA should have same length as input data");
}

#[tokio::test]
async fn test_growth_calculations() {
    // Test: Growth calculations work correctly
    let calculator = GrowthCalculator::new();
    let test_data = vec![100.0, 110.0, 121.0, 133.1, 146.41, 161.05];

    // Test growth rate calculation
    let growth_rate = calculator
        .calculate_growth_rate(&test_data, None)
        .expect("Should calculate growth rate");
    assert!(
        growth_rate.percentage_growth > 0.0,
        "Should have positive growth"
    );
    assert!(growth_rate.cagr.is_some(), "Should have CAGR");
    assert_eq!(
        growth_rate.growth_trend,
        GrowthTrend::Stable,
        "Should be stable growth"
    );

    // Test CAGR calculation
    let cagr = calculator.calculate_cagr(100.0, 161.05, 5.0);
    assert!(cagr > 0.0, "CAGR should be positive");
    assert!(cagr < 20.0, "CAGR should be reasonable");

    // Test growth metrics
    let growth_metrics = calculator
        .calculate_growth_metrics(&test_data)
        .expect("Should calculate growth metrics");
    assert!(
        growth_metrics.total_growth > 0.0,
        "Total growth should be positive"
    );
    assert!(
        growth_metrics.average_period_growth > 0.0,
        "Average period growth should be positive"
    );
    assert!(
        growth_metrics.peak_growth > 0.0,
        "Peak growth should be positive"
    );

    // Test YoY growth
    let year_periods = vec![2020.0, 2021.0, 2022.0, 2023.0, 2024.0, 2025.0];
    let yoy_growth = calculator
        .calculate_yoy_growth(&test_data, &year_periods)
        .expect("Should calculate YoY growth");
    assert_eq!(yoy_growth.len(), 5, "YoY growth should have correct length");

    // Test QoQ growth
    let quarter_periods = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let qoq_growth = calculator
        .calculate_qoq_growth(&test_data, &quarter_periods)
        .expect("Should calculate QoQ growth");
    assert_eq!(qoq_growth.len(), 5, "QoQ growth should have correct length");
}

#[tokio::test]
async fn test_performance_metrics() {
    // Test: Performance metrics work correctly
    let analyzer = PerformanceAnalyzer::new();

    // Test performance metrics calculation
    let metrics = analyzer
        .calculate_performance_metrics(
            1000,                    // operations
            Duration::from_secs(10), // total time
            950,                     // successful operations
            50,                      // failed operations
            1024 * 1024,             // 1MB data
        )
        .expect("Should calculate performance metrics");

    assert_eq!(
        metrics.throughput, 100.0,
        "Throughput should be 100 ops/sec"
    );
    assert_eq!(metrics.efficiency, 95.0, "Efficiency should be 95%");
    assert_eq!(metrics.error_rate, 5.0, "Error rate should be 5%");
    assert_eq!(metrics.success_rate, 95.0, "Success rate should be 95%");
    assert!(
        metrics.performance_score > 0.0,
        "Performance score should be positive"
    );

    // Test benchmark
    let benchmark_result = analyzer
        .run_benchmark("test_operation", || Ok(()), 100, 1024)
        .expect("Should run benchmark");

    assert_eq!(benchmark_result.operation_name, "test_operation");
    assert!(
        benchmark_result.operations_per_second > 0.0,
        "Should have positive ops/sec"
    );
    assert!(
        benchmark_result.average_latency > Duration::ZERO,
        "Should have positive latency"
    );

    // Test throughput calculation
    let throughput = analyzer.calculate_throughput(1000, Duration::from_secs(10));
    assert_eq!(throughput, 100.0, "Throughput should be 100 ops/sec");

    // Test efficiency calculation
    let efficiency = analyzer.calculate_efficiency(950, 1000);
    assert_eq!(efficiency, 95.0, "Efficiency should be 95%");

    // Test error rate calculation
    let error_rate = analyzer.calculate_error_rate(50, 1000);
    assert_eq!(error_rate, 5.0, "Error rate should be 5%");
}

#[tokio::test]
async fn test_data_normalization() {
    // Test: Data normalization works correctly
    let normalizer = DataNormalizer::new();
    let test_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

    // Test Min-Max normalization
    let min_max_result = normalizer
        .normalize(&test_data, NormalizationMethod::MinMax)
        .expect("Should normalize data");
    assert_eq!(
        min_max_result.normalized_data.len(),
        10,
        "Normalized data should have same length"
    );
    assert_eq!(
        min_max_result.normalized_data[0], 0.0,
        "First value should be 0.0"
    );
    assert_eq!(
        min_max_result.normalized_data[9], 1.0,
        "Last value should be 1.0"
    );

    // Test Z-score normalization
    let z_score_result = normalizer
        .normalize(&test_data, NormalizationMethod::ZScore)
        .expect("Should normalize data");
    assert_eq!(
        z_score_result.normalized_data.len(),
        10,
        "Normalized data should have same length"
    );

    // Test robust normalization
    let robust_result = normalizer
        .normalize(&test_data, NormalizationMethod::Robust)
        .expect("Should normalize data");
    assert_eq!(
        robust_result.normalized_data.len(),
        10,
        "Normalized data should have same length"
    );

    // Test unit vector normalization
    let unit_vector_result = normalizer
        .normalize(&test_data, NormalizationMethod::UnitVector)
        .expect("Should normalize data");
    assert_eq!(
        unit_vector_result.normalized_data.len(),
        10,
        "Normalized data should have same length"
    );

    // Test decimal scaling normalization
    let decimal_scaling_result = normalizer
        .normalize(&test_data, NormalizationMethod::DecimalScaling)
        .expect("Should normalize data");
    assert_eq!(
        decimal_scaling_result.normalized_data.len(),
        10,
        "Normalized data should have same length"
    );

    // Test denormalization
    let denormalized = normalizer
        .denormalize(
            &min_max_result.normalized_data,
            &min_max_result.parameters,
            NormalizationMethod::MinMax,
        )
        .expect("Should denormalize data");

    assert_eq!(
        denormalized.len(),
        10,
        "Denormalized data should have same length"
    );
    assert!(
        (denormalized[0] - 1.0).abs() < 0.001,
        "First denormalized value should be ~1.0"
    );
    assert!(
        (denormalized[9] - 10.0).abs() < 0.001,
        "Last denormalized value should be ~10.0"
    );

    // Test multiple dataset normalization
    let datasets = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let multiple_results = normalizer
        .normalize_multiple(&datasets, NormalizationMethod::MinMax)
        .expect("Should normalize multiple datasets");
    assert_eq!(
        multiple_results.len(),
        3,
        "Should have 3 normalized datasets"
    );

    // Test normalization check
    let is_normalized =
        normalizer.is_normalized(&min_max_result.normalized_data, NormalizationMethod::MinMax);
    assert!(
        is_normalized,
        "Min-max normalized data should be detected as normalized"
    );
}

#[tokio::test]
async fn test_metrics_integration() {
    // Test: All metrics modules work together correctly
    let statistical_calc = StatisticalCalculator::new();
    let trend_analyzer = TrendAnalyzer::new();
    let growth_calc = GrowthCalculator::new();
    let performance_analyzer = PerformanceAnalyzer::new();
    let normalizer = DataNormalizer::new();

    // Create test dataset
    let test_data = vec![
        100.0, 110.0, 121.0, 133.1, 146.41, 161.05, 177.16, 194.87, 214.36, 235.79,
    ];

    // Statistical analysis
    let stats = statistical_calc
        .calculate_statistics(&test_data)
        .expect("Should calculate statistics");
    assert!(stats.mean > 0.0, "Mean should be positive");
    assert!(
        stats.standard_deviation > 0.0,
        "Standard deviation should be positive"
    );

    // Trend analysis
    let trend = trend_analyzer
        .analyze_trend(&test_data)
        .expect("Should analyze trend");
    assert_eq!(
        trend.trend_type,
        TrendType::Increasing,
        "Should detect increasing trend"
    );
    assert!(trend.slope > 0.0, "Slope should be positive");

    // Growth analysis
    let growth = growth_calc
        .calculate_growth_rate(&test_data, None)
        .expect("Should calculate growth");
    assert!(
        growth.percentage_growth > 0.0,
        "Should have positive growth"
    );
    assert!(growth.cagr.is_some(), "Should have CAGR");

    // Performance analysis
    let performance = performance_analyzer
        .calculate_performance_metrics(1000, Duration::from_secs(10), 950, 50, 1024 * 1024)
        .expect("Should calculate performance");
    assert!(
        performance.performance_score > 0.0,
        "Performance score should be positive"
    );

    // Data normalization
    let normalized = normalizer
        .normalize(&test_data, NormalizationMethod::MinMax)
        .expect("Should normalize data");
    assert_eq!(
        normalized.normalized_data.len(),
        test_data.len(),
        "Normalized data should have same length"
    );

    // Verify all components work together
    assert!(
        stats.count == test_data.len(),
        "Statistical count should match data length"
    );
    assert!(
        trend.data_points == test_data.len(),
        "Trend data points should match data length"
    );
    assert!(
        growth.period_growth.len() == test_data.len() - 1,
        "Growth periods should be correct length"
    );
}

#[tokio::test]
async fn test_edge_cases() {
    // Test: Edge cases are handled correctly
    let calculator = StatisticalCalculator::new();
    let analyzer = TrendAnalyzer::new();
    let normalizer = DataNormalizer::new();

    // Test empty data
    let empty_data: Vec<f64> = vec![];
    assert!(
        calculator.calculate_statistics(&empty_data).is_err(),
        "Should error on empty data"
    );
    assert!(
        analyzer.analyze_trend(&empty_data).is_err(),
        "Should error on empty data"
    );
    assert!(
        normalizer
            .normalize(&empty_data, NormalizationMethod::MinMax)
            .is_err(),
        "Should error on empty data"
    );

    // Test single data point
    let single_data = vec![5.0];
    let stats = calculator
        .calculate_statistics(&single_data)
        .expect("Should handle single data point");
    assert_eq!(stats.count, 1, "Should have count of 1");
    assert!(
        analyzer.analyze_trend(&single_data).is_err(),
        "Should error on single data point"
    );

    // Test two data points
    let two_data = vec![1.0, 2.0];
    let stats = calculator
        .calculate_statistics(&two_data)
        .expect("Should handle two data points");
    assert_eq!(stats.count, 2, "Should have correct count");

    // Test identical values
    let identical_data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
    let stats = calculator
        .calculate_statistics(&identical_data)
        .expect("Should handle identical values");
    assert_eq!(stats.mean, 5.0, "Mean should be correct");
    assert_eq!(
        stats.standard_deviation, 0.0,
        "Standard deviation should be 0"
    );

    // Test zero values
    let zero_data = vec![0.0, 0.0, 0.0];
    let stats = calculator
        .calculate_statistics(&zero_data)
        .expect("Should handle zero values");
    assert_eq!(stats.mean, 0.0, "Mean should be 0");
    assert_eq!(
        stats.standard_deviation, 0.0,
        "Standard deviation should be 0"
    );
}
