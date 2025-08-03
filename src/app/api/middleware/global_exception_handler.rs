use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;


/// 404错误响应结构体
#[derive(Serialize)]
struct NotFoundResponse {
    code: i32,
    message: String,
    request_id: String,
}

/// 全局异常处理器，处理业务异常和特定的HTTP状态码
pub async fn global_exception_handler(req: Request, next: Next) -> Response {
    let request_id = *req.extensions().get::<Uuid>().unwrap();
    let res = next.run(req).await;

    // 处理业务异常（AppError会被IntoResponse正确处理）
    if res.status() == StatusCode::NOT_FOUND {
        let response = NotFoundResponse {
            code: 404,
            message: "请求的接口不存在".to_string(),
            request_id: request_id.to_string(),
        };
        return (StatusCode::NOT_FOUND, Json(response)).into_response();
    }

    if res.status() == StatusCode::METHOD_NOT_ALLOWED {
        let response = NotFoundResponse {
            code: 405,
            message: "请求的方法不允许".to_string(),
            request_id: request_id.to_string(),
        };
        return (StatusCode::METHOD_NOT_ALLOWED, Json(response)).into_response();
    }

    if res.status() == StatusCode::UNAUTHORIZED {
        let response = NotFoundResponse {
            code: 401,
            message: "未授权访问".to_string(),
            request_id: request_id.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
    }

    if res.status() == StatusCode::FORBIDDEN {
        let response = NotFoundResponse {
            code: 403,
            message: "禁止访问".to_string(),
            request_id: request_id.to_string(),
        };
        return (StatusCode::FORBIDDEN, Json(response)).into_response();
    }

    if res.status() == StatusCode::TOO_MANY_REQUESTS {
        let response = NotFoundResponse {
            code: 429,
            message: "请求过于频繁".to_string(),
            request_id: request_id.to_string(),
        };
        return (StatusCode::TOO_MANY_REQUESTS, Json(response)).into_response();
    }

    // 其他错误（包括验证错误、业务错误等）由AppError的IntoResponse处理
    res
}
