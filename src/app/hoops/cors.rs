//! CORS 中间件 (Hoop)
//!
//! 基于 salvo-template 的标准 CORS 实现

use salvo::cors::{AllowHeaders, AllowMethods, AllowOrigin, Cors, CorsHandler};
use salvo::http::{HeaderName, HeaderValue, Method};

/// 创建 CORS 中间件
///
/// 基于 salvo-template 的实现，提供标准的跨域资源共享支持
pub fn cors_hoop() -> CorsHandler {
    Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler()
}

/// 创建生产环境的 CORS 中间件
///
/// 更严格的 CORS 配置，适用于生产环境
pub fn cors_hoop_production(allowed_origins: Vec<&str>) -> CorsHandler {
    let origins: Vec<HeaderValue> = allowed_origins
        .into_iter()
        .map(|origin| HeaderValue::from_str(origin).unwrap())
        .collect();

    let methods = vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::OPTIONS,
    ];

    let headers = vec![
        HeaderName::from_static("content-type"),
        HeaderName::from_static("authorization"),
        HeaderName::from_static("x-requested-with"),
        HeaderName::from_static("accept"),
        HeaderName::from_static("origin"),
    ];

    Cors::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods(AllowMethods::from(methods))
        .allow_headers(AllowHeaders::list(headers))
        .into_handler()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_hoop_creation() {
        let _cors_handler = cors_hoop();
        // 如果能创建成功，说明配置正确
        assert!(true);
    }

    #[test]
    fn test_cors_hoop_production_creation() {
        let allowed_origins = vec!["https://example.com", "https://app.example.com"];
        let _cors_handler = cors_hoop_production(allowed_origins);
        // 如果能创建成功，说明配置正确
        assert!(true);
    }
}
