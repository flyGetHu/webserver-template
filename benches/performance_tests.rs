use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use tokio::runtime::Runtime;
use webserver_template::app::config::Config;
use webserver_template::app::container::AppServices;

// 性能测试配置
const TEST_ITERATIONS: usize = 1000;
const CONCURRENT_REQUESTS: usize = 100;

/// 配置加载性能测试
fn bench_config_loading(c: &mut Criterion) {
    c.bench_function("config_loading", |b| {
        b.iter(|| black_box(Config::load().expect("Failed to load config")))
    });
}

/// 服务容器初始化性能测试
fn bench_service_container_init(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("service_container_init", |b| {
        b.to_async(&rt).iter(|| async {
            let config = Config::load().expect("Failed to load config");
            black_box(
                AppServices::new(&config)
                    .await
                    .expect("Failed to create services"),
            )
        })
    });
}

/// JWT 令牌生成性能测试
fn bench_jwt_token_generation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("jwt_token_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let config = Config::load().expect("Failed to load config");
            let services = AppServices::new(&config)
                .await
                .expect("Failed to create services");

            // 模拟用户数据
            let user_id = 1;
            let username = "test_user";
            let email = "test@example.com";
            let roles = vec!["user".to_string()];

            black_box(
                services
                    .auth_service
                    .generate_token(user_id, username, email, roles)
                    .expect("Failed to generate token"),
            )
        })
    });
}

/// 内存分配测试 - 字符串处理
fn bench_string_operations(c: &mut Criterion) {
    c.bench_function("string_operations", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..100 {
                result.push_str(&format!("item_{}", i));
            }
            black_box(result)
        })
    });
}

/// 内存分配测试 - JSON 序列化
fn bench_json_serialization(c: &mut Criterion) {
    use serde_json;
    use std::collections::HashMap;

    c.bench_function("json_serialization", |b| {
        let mut data = HashMap::new();
        for i in 0..50 {
            data.insert(format!("key_{}", i), format!("value_{}", i));
        }

        b.iter(|| black_box(serde_json::to_string(&data).expect("Failed to serialize")))
    });
}

/// 异步任务处理性能测试
fn bench_async_task_processing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("async_task_processing", |b| {
        b.to_async(&rt).iter(|| async {
            let tasks: Vec<_> = (0..10)
                .map(|i| {
                    tokio::spawn(async move {
                        // 模拟异步工作
                        tokio::time::sleep(Duration::from_millis(1)).await;
                        i * 2
                    })
                })
                .collect();

            let results: Vec<_> = futures::future::join_all(tasks)
                .await
                .into_iter()
                .map(|r| r.unwrap())
                .collect();

            black_box(results)
        })
    });
}

criterion_group!(
    benches,
    bench_config_loading,
    bench_service_container_init,
    bench_jwt_token_generation,
    bench_string_operations,
    bench_json_serialization,
    bench_async_task_processing
);

criterion_main!(benches);
