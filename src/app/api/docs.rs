//! API文档配置模块
//!
//! 使用utoipa生成OpenAPI文档，提供Swagger UI界面

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::app::api::{handlers::user_handler, response::ApiResponse};

/// API文档结构定义
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Web服务器模板API",
        description = "一个基于Rust Axum的现代Web服务器API模板，提供用户管理、认证等功能。",
        contact(
            name = "API支持",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        version = "1.0.0"
    ),
    paths(
        user_handler::create_user,
        super::routes::health_check
    ),
    components(
        schemas(
            user_handler::CreateUserPayload,
            user_handler::CreateUserResponse,
            ApiResponse<user_handler::CreateUserResponse>
        )
    ),
    tags(
        (name = "用户管理", description = "用户相关的API端点"),
        (name = "系统", description = "系统健康和监控相关的API端点")
    )
)]
pub struct ApiDoc;

/// 创建Swagger UI路由
pub fn create_swagger_routes() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}