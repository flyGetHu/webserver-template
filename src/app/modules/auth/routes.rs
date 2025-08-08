//! 认证模块路由定义
//!
//! 使用Salvo的树状路由结构组织认证相关的端点

use salvo::prelude::*;

use super::handlers::{login, logout, register};

/// 创建认证模块的路由
///
/// 使用树状结构组织认证相关的端点：
/// - POST /auth/register - 用户注册
/// - POST /auth/login - 用户登录  
/// - POST /auth/logout - 用户注销
pub fn create_routes() -> Router {
    Router::with_path("auth")
        .push(Router::with_path("register").post(register))
        .push(Router::with_path("login").post(login))
        .push(
            Router::with_path("logout")
                // TODO: 添加认证中间件
                // .hoop(auth_middleware)
                .post(logout),
        )
}
