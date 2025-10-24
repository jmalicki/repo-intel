//! Performance Benchmarks
//!
//! Comprehensive performance benchmarks for all common-library components
//! including HTTP client, storage operations, metrics calculations, and validation.

use common_library::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use serde_json::json;
use std::time::Duration;

fn bench_configuration_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("configuration");

    group.bench_function("config_set", |b| {
        let mut config = ConfigManager::new();
        b.iter(|| {
            config.set("test.key", "test.value").unwrap();
        });
    });

    group.bench_function("config_get", |b| {
        let mut config = ConfigManager::new();
        config.set("test.key", "test.value").unwrap();
        b.iter(|| {
            let _: String = config.get("test.key").unwrap();
        });
    });

    group.bench_function("config_keys", |b| {
        let mut config = ConfigManager::new();
        for i in 0..100 {
            config.set(&format!("key.{}", i), &format!("value.{}", i)).unwrap();
        }
        b.iter(|| {
            let _ = config.keys().unwrap();
        });
    });

    group.finish();
}

fn bench_http_client_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("http_client");

    group.bench_function("client_creation", |b| {
        b.iter(|| {
            let config = HttpClientConfig {
                timeout: Duration::from_secs(30),
                max_retries: 3,
                rate_limit: 100,
                ..Default::default()
            };
            black_box(APIClient::new(config));
        });
    });

    group.bench_function("auth_configuration", |b| {
        let mut client = APIClient::new(HttpClientConfig::default());
        let auth_config = AuthConfig {
            auth_type: AuthType::Bearer,
            token: Some("test-token".to_string()),
            username: None,
            password: None,
            api_key: None,
            custom_headers: None,
        };
        b.iter(|| {
            client.set_auth(auth_config.clone());
        });
    });

    group.finish();
}

fn bench_storage_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage");

    group.bench_function("file_write", |b| {
        let file_manager = FileManager::new();
        let test_content = "test content for benchmarking";
        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                file_manager.write_file("bench_test.txt", test_content).await.unwrap();
            });
        });
    });

    group.bench_function("file_read", |b| {
        let file_manager = FileManager::new();
        let test_content = "test content for benchmarking";
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            file_manager.write_file("bench_test.txt", test_content).await.unwrap();
        });

        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = file_manager.read_file("bench_test.txt").await.unwrap();
            });
        });
    });

    group.bench_function("json_write", |b| {
        let json_manager = JsonFileManager::new();
        let test_data = json!({
            "id": "benchmark-test",
            "value": 42.5,
            "array": [1, 2, 3, 4, 5],
            "nested": {
                "key": "value",
                "number": 123
            }
        });

        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                json_manager.write_json("bench_test.json", &test_data).await.unwrap();
            });
        });
    });

    group.bench_function("json_read", |b| {
        let json_manager = JsonFileManager::new();
        let test_data = json!({
            "id": "benchmark-test",
            "value": 42.5,
            "array": [1, 2, 3, 4, 5],
            "nested": {
                "key": "value",
                "number": 123
            }
        });

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            json_manager.write_json("bench_test.json", &test_data).await.unwrap();
        });

        b.iter(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let _ = json_manager.read_json("bench_test.json").await.unwrap();
            });
        });
    });

    group.finish();
}

fn bench_metrics_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("metrics");

    // Test with different data sizes
    for size in [100, 1000, 10000].iter() {
        let data: Vec<f64> = (0..*size).map(|i| i as f64).collect();
        let statistical_calculator = StatisticalCalculator::new();
        let trend_analyzer = TrendAnalyzer::new();
        let growth_calculator = GrowthCalculator::new();
        let data_normalizer = DataNormalizer::new();

        group.bench_with_input(BenchmarkId::new("statistics", size), size, |b, _| {
            b.iter(|| {
                black_box(statistical_calculator.calculate_statistics(&data).unwrap());
            });
        });

        group.bench_with_input(BenchmarkId::new("trend_analysis", size), size, |b, _| {
            b.iter(|| {
                black_box(trend_analyzer.analyze_trend(&data).unwrap());
            });
        });

        group.bench_with_input(BenchmarkId::new("growth_calculation", size), size, |b, _| {
            b.iter(|| {
                black_box(growth_calculator.calculate_cagr(&data, 5.0).unwrap());
            });
        });

        group.bench_with_input(BenchmarkId::new("data_normalization", size), size, |b, _| {
            b.iter(|| {
                black_box(data_normalizer.normalize(&data, NormalizationMethod::ZScore).unwrap());
            });
        });
    }

    group.finish();
}

fn bench_validation_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");

    group.bench_function("schema_validation", |b| {
        let mut schema_validator = SchemaValidator::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "name": {"type": "string", "minLength": 1},
                "value": {"type": "number", "minimum": 0}
            },
            "required": ["id", "name", "value"]
        });
        schema_validator.register_schema("test_schema", schema).unwrap();

        let test_data = json!({
            "id": "test-001",
            "name": "Test Item",
            "value": 42.5
        });

        b.iter(|| {
            black_box(schema_validator.validate(&test_data, "test_schema").unwrap());
        });
    });

    group.bench_function("type_validation", |b| {
        let type_validator = TypeValidator::new();
        let test_data = json!(42.5);

        b.iter(|| {
            black_box(type_validator.validate_type(&test_data, "number").unwrap());
        });
    });

    group.bench_function("integrity_check", |b| {
        let integrity_checker = DataIntegrityChecker::new();
        let test_data = json!({
            "id": "test-001",
            "value": 42.5
        });

        b.iter(|| {
            black_box(integrity_checker.check_integrity(&test_data, "test_data").unwrap());
        });
    });

    group.bench_function("error_reporting", |b| {
        let mut error_reporter = ValidationErrorReporter::new();

        b.iter(|| {
            let error = ValidationError {
                error_type: ValidationErrorType::SchemaValidation,
                path: "test.path".to_string(),
                message: "Test error message".to_string(),
                severity: ErrorSeverity::Error,
                suggestion: None,
                context: None,
                timestamp: chrono::Utc::now(),
            };
            error_reporter.add_error(error);
        });
    });

    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");

    group.bench_function("component_creation", |b| {
        b.iter(|| {
            let _config = ConfigManager::new();
            let _http_client = APIClient::new(HttpClientConfig::default());
            let _file_manager = FileManager::new();
            let _json_manager = JsonFileManager::new();
            let _statistical_calculator = StatisticalCalculator::new();
            let _trend_analyzer = TrendAnalyzer::new();
            let _schema_validator = SchemaValidator::new();
            let _integrity_checker = DataIntegrityChecker::new();
            let _type_validator = TypeValidator::new();
            let _error_reporter = ValidationErrorReporter::new();
            let _schema_registry = SchemaRegistry::new();
        });
    });

    group.bench_function("large_data_processing", |b| {
        let large_data: Vec<f64> = (0..100000).map(|i| i as f64).collect();
        let statistical_calculator = StatisticalCalculator::new();

        b.iter(|| {
            black_box(statistical_calculator.calculate_statistics(&large_data).unwrap());
        });
    });

    group.finish();
}

fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent");

    group.bench_function("concurrent_config", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..10).map(|i| {
                std::thread::spawn(move || {
                    let mut config = ConfigManager::new();
                    config.set(&format!("key.{}", i), &format!("value.{}", i)).unwrap();
                    config.get::<String>(&format!("key.{}", i)).unwrap()
                })
            }).collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    group.bench_function("concurrent_metrics", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..5).map(|i| {
                std::thread::spawn(move || {
                    let data: Vec<f64> = (0..1000).map(|j| (i * 1000 + j) as f64).collect();
                    let calculator = StatisticalCalculator::new();
                    calculator.calculate_statistics(&data).unwrap()
                })
            }).collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_configuration_operations,
    bench_http_client_operations,
    bench_storage_operations,
    bench_metrics_calculations,
    bench_validation_operations,
    bench_memory_usage,
    bench_concurrent_operations
);

criterion_main!(benches);
