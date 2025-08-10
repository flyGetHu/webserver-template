//! 健康检查处理器
//!
//! 提供应用健康状态检查的HTTP端点

use salvo::prelude::*;

/// 健康检查处理器
///
/// 用于检查服务器是否正常运行，返回简单的状态信息
#[endpoint(
    tags("Health"),
    operation_id = "health_check",
    responses(
        (status_code = 200, description = "Service is healthy")
    )
)]
pub async fn health_check(res: &mut Response) {
    res.render(Text::Plain("OK"));
}

/// 创建健康检查路由
#[must_use]
pub fn create_routes() -> Router {
    Router::with_path("health").get(health_check)
}
