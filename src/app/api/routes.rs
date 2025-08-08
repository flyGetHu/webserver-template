//! API路由定义模块
//!
//! 定义应用的所有路由端点

use salvo::prelude::*;

use crate::app::{api::docs, modules};

/// 创建应用的所有路由
///
/// 此函数负责将URL路径映射到相应的处理函数
pub fn create_routes() -> Router {
    Router::new()
        // 使用模块化路由结构
        .push(modules::create_routes())
        // Swagger UI文档端点
        .push(docs::create_swagger_routes())
}
