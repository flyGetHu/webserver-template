//! API路由定义模块
//!
//! 定义应用的所有路由端点

use salvo::prelude::*;

use crate::app::api::{handlers::{user_handler, auth_handler}, docs};

/// 创建应用的所有路由
///
/// 此函数负责将URL路径映射到相应的处理函数
pub fn create_routes() -> Router {
    Router::new()
        // 健康检查端点
        .push(Router::with_path("/health").get(health_check))
        // 认证相关端点
        .push(Router::with_path("/api/v1/auth/register").post(auth_handler::register))
        .push(Router::with_path("/api/v1/auth/login").post(auth_handler::login))
        .push(Router::with_path("/api/v1/auth/logout").post(auth_handler::logout))
        // 用户相关端点
        .push(Router::with_path("/api/v1/users").post(user_handler::create_user))
        // Swagger UI文档端点
        .push(docs::create_swagger_routes())
}

/// 健康检查
///
/// 用于检查服务器是否正常运行，返回简单的状态信息。
#[handler]
async fn health_check(res: &mut Response) {
    
    res.render(Text::Plain("OK"));
}