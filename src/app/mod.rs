//! 应用模块声明

pub mod api;
pub mod config;
pub mod container;
pub mod domain;
pub mod error;
pub mod hoops;
pub mod infrastructure;
pub mod modules;
pub mod state;
use salvo::prelude::*;

use std::sync::Arc;

use crate::app::{
    api::{
        middleware::{
            global_exception_handler::global_exception_handler,
            request_id::{create_request_id_middleware, request_id_handler},
            request_logger::request_logger,
        },
        routes::create_routes,
    },
    config::Config,
    container::{inject_services_middleware, AppServices},
    state::AppState,
};

/// 运行应用的主要函数
///
/// 该函数封装了应用启动的完整逻辑
///
/// # Panics
/// 当配置加载失败时会 panic（内部使用 `expect`）。
///
/// # Errors
/// 网络绑定或服务运行失败时返回错误。
pub async fn run() -> anyhow::Result<()> {
    // 加载配置
    let config = Config::load().expect("Failed to load configuration");

    // 使用新的日志配置初始化日志系统
    let _log_guard = config.log.guard();
    tracing::info!("Loaded configuration: {:?}", config);

    // 创建数据库连接
    let rb = match state::create_db_pool(&config.database.url) {
        Ok(rb) => rb,
        Err(e) => {
            tracing::warn!("Failed to create database pool: {}", e);
            // Create a mock rb for testing
            state::create_mock_db_pool()?
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
    let app_state = AppState::new(rb, redis_pool);
    let app_state = Arc::new(app_state);

    // 创建应用服务容器
    let services = Arc::new(AppServices::new(Arc::new(config.clone()), app_state.clone()));

    // 绑定地址 - 使用新的 listen_addr 配置
    let addr: std::net::SocketAddr = config
        .listen_addr
        .parse()
        .expect("Failed to parse server address");

    tracing::info!("Starting server on {}", addr);

    // 创建路由和中间件
    let router = create_routes()
        .hoop(create_request_id_middleware()) // Salvo 内置 RequestId 中间件
        .hoop(request_id_handler) // 自定义处理器，集成 tracing 和 depot
        .hoop(inject_services_middleware(services)) // 新的服务注入中间件
        .hoop(request_logger) // 请求日志中间件
        .hoop(global_exception_handler); // 全局异常处理

    // 启动服务器
    let acceptor = TcpListener::new(addr).bind().await;
    Server::new(acceptor).serve(router).await;

    Ok(())
}
