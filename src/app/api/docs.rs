//! API文档配置模块
//!
//! 使用Salvo内置的OpenAPI支持生成API文档，提供Swagger UI界面

use salvo::prelude::*;


/// 创建Swagger UI路由
pub fn create_swagger_routes() -> Router {
    // 创建OpenAPI文档
    let doc = OpenApi::new("Web服务器模板API", "1.0.0");

    // 创建路由
    Router::new()
        .push(doc.into_router("/api-docs/openapi.json"))
        .push(SwaggerUi::new("/api-docs/openapi.json").into_router("/swagger-ui"))
}