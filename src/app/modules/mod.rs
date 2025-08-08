//! 应用模块
//!
//! 使用模块化架构组织业务逻辑，每个模块包含：
//! - handlers: HTTP请求处理器
//! - models: 数据传输对象和领域模型
//! - routes: 路由定义
//! - services: 业务逻辑服务

pub mod auth;
pub mod health;
pub mod users;

use salvo::prelude::*;

/// 创建所有模块的路由
///
/// 将各个模块的路由组合成完整的API路由树
pub fn create_routes() -> Router {
    Router::with_path("api/v1")
        .push(auth::create_routes())
        .push(users::create_routes())
        .push(health::create_routes())
}
