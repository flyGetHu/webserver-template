//! 应用模块声明

pub mod config;
pub mod error;
pub mod state;
pub mod api;
pub mod domain;
pub mod infrastructure;
use axum::middleware;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::{
    api::{
        middleware::{
            global_exception_handler::global_exception_handler,
            request_id::add_request_id,
        },
        routes::create_routes,
    },
    config::Config,
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
                .unwrap_or_else(|_| "webserver_template=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = Config::load().expect("Failed to load configuration");
    tracing::info!("Loaded configuration: {:?}", config);

    // 创建数据库连接池
    let db_pool = state::create_db_pool(&config.database.url).await?;
    
    // 创建Redis连接池
    let redis_pool = state::create_redis_pool(&config.redis.url).await?;

    // 创建应用状态
    let app_state = AppState::new(db_pool, redis_pool);

    // 创建路由和中间件
    let app = create_routes()
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new().allow_origin(Any))
                .layer(middleware::from_fn(add_request_id))
                .layer(middleware::from_fn(global_exception_handler))
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(app_state);

    // 绑定地址
    let addr: std::net::SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Failed to parse server address");
    
    tracing::info!("Starting server on {}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
