//! API路由定义模块
//!
//! 定义应用的所有路由端点

use axum::{
    routing::{get, post},
    Router,
};

use crate::app::{api::{handlers::user_handler, docs}, container::ServiceRegistry};

/// 创建应用的所有路由
///
/// 此函数负责将URL路径映射到相应的处理函数
pub fn create_routes() -> Router<ServiceRegistry> {
    Router::new()
        // 健康检查端点
        .route("/health", get(health_check))
        // 用户相关端点
        .route("/api/v1/users", post(user_handler::create_user))
        // Swagger UI文档端点
        .merge(docs::create_swagger_routes())
}

/// 健康检查
///
/// 用于检查服务器是否正常运行，返回简单的状态信息。
#[utoipa::path(
    get,
    path = "/health",
    tag = "系统",
    responses(
        (status = 200, description = "服务正常运行", body = String, example = "OK"),
        (status = 503, description = "服务不可用")
    ),
    operation_id = "health_check"
)]
async fn health_check() -> &'static str {
    "OK"
}