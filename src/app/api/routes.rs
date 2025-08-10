//! API路由定义模块
//!
//! 融合 salvo-template 的简洁路由组织和 webserver-template 的模块化结构

use salvo::prelude::*;

use crate::app::{api::docs, hoops, modules};

/// 创建应用的所有路由
///
/// 融合两个项目的路由组织优势：
/// - 保留 webserver-template 的模块化结构
/// - 引入 salvo-template 的标准中间件应用方式
/// - 优化 `OpenAPI` 文档集成
#[must_use]
pub fn create_routes() -> Router {
    // 创建 API 路由
    let api_router = Router::with_path("api/v1")
        .push(modules::auth::create_routes())
        .push(modules::users::create_routes())
        .push(modules::health::create_routes());

    // 创建主路由，应用标准化中间件
    let router = Router::new()
        // 应用 CORS 中间件 (基于 salvo-template)
        .hoop(hoops::cors::cors_hoop())
        // API 路由
        .push(api_router)
        // OpenAPI 文档路由 (优化集成)
        .push(docs::create_swagger_routes());

    // 生成 OpenAPI 文档 (基于 salvo-template 的方式)
    let doc = OpenApi::new("WebServer Template API", "1.0.0").merge_router(&router);

    // 返回完整的路由，包含 OpenAPI 文档端点
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}
