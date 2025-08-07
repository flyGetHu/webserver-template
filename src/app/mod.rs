//! 应用模块声明

pub mod config;
pub mod container;
pub mod error;
pub mod state;
pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod logging;
use salvo::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::sync::Arc;

use crate::app::{
    api::{
        middleware::{
            global_exception_handler::global_exception_handler,
            request_id::{add_request_id, request_id_middleware},
        },
        routes::create_routes,
    },
    config::Config,
    container::ServiceRegistry,
    logging::RequestIdFormat,
    state::AppState,
};


/// 运行应用的主要函数
///
/// 该函数封装了应用启动的完整逻辑
pub async fn run() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "webserver_template=debug,salvo=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer().event_format(RequestIdFormat))
        .init();

    // 加载配置
    let config = Config::load().expect("Failed to load configuration");
    tracing::info!("Loaded configuration: {:?}", config);

    // 创建数据库连接池
    let db_pool = match state::create_db_pool(&config.database.url).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::warn!("Failed to create database pool: {}", e);
            // Create a mock pool for testing
            state::create_mock_db_pool().await?
        }
    };
    
    // 创建Redis连接池
    let redis_pool = match state::create_redis_pool(&config.redis.url).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::warn!("Failed to create Redis pool: {}", e);
            // Create a mock pool for testing
            state::create_mock_redis_pool().await?
        }
    };

    // 创建应用状态
    let app_state = AppState::new(db_pool, redis_pool);
    let app_state = Arc::new(app_state);

    // 创建服务注册表
    let _service_registry = ServiceRegistry::new(Arc::new(config.clone()), app_state.clone()).await;

    // 创建路由和中间件
    let router = create_routes()
        .hoop(request_id_middleware())
        .hoop(add_request_id)
        .hoop(global_exception_handler);

    // 绑定地址
    let addr: std::net::SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Failed to parse server address");
    
    tracing::info!("Starting server on {}", addr);

    // 启动服务器
    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
