//! API路由定义模块
//!
//! 定义应用的所有路由端点

use axum::{
    routing::{get, post},
    Router,
};

use crate::app::{api::handlers::user_handler, state::AppState};

/// 创建应用的所有路由
///
/// 此函数负责将URL路径映射到相应的处理函数
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // 健康检查端点
        .route("/health", get(health_check))
        // 用户相关端点
        .route("/api/v1/users", post(user_handler::create_user))
}

/// 健康检查处理函数
///
/// 用于检查服务器是否正常运行
async fn health_check() -> &'static str {
    "OK"
}