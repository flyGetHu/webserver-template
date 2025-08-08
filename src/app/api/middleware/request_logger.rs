use salvo::prelude::*;
use std::time::Instant;
use tracing::{info, warn};
use uuid::Uuid;

/// 请求日志中间件
///
/// 记录每个请求的详细信息，包括：
/// - 请求方法和路径
/// - 请求头信息
/// - 客户端IP
/// - 响应状态码
/// - 请求处理时间
/// - Request ID
#[handler]
pub async fn request_logger(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let start_time = Instant::now();

    // 获取请求信息
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let version = format!("{:?}", req.version());

    // 获取客户端IP
    let client_ip = req.remote_addr().to_string();

    // 获取 User-Agent
    let user_agent = req
        .header::<String>("user-agent")
        .unwrap_or_else(|| "unknown".to_string());

    // 获取 request_id
    let request_id = depot
        .get::<Uuid>("request_id")
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4());

    // 记录请求开始日志
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        version = %version,
        client_ip = %client_ip,
        user_agent = %user_agent,
        "Request started"
    );

    // 处理请求
    ctrl.call_next(req, depot, res).await;

    // 计算处理时间
    let duration = start_time.elapsed();
    let duration_ms = duration.as_millis();

    // 获取响应状态码
    let status_code = res.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    // 获取响应内容长度
    let content_length = res
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    // 根据状态码选择日志级别
    if status_code.is_success() {
        info!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            status_code = %status_code.as_u16(),
            duration_ms = %duration_ms,
            content_length = %content_length,
            "Request completed successfully"
        );
    } else if status_code.is_client_error() {
        warn!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            status_code = %status_code.as_u16(),
            duration_ms = %duration_ms,
            content_length = %content_length,
            "Request completed with client error"
        );
    } else {
        warn!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            status_code = %status_code.as_u16(),
            duration_ms = %duration_ms,
            content_length = %content_length,
            "Request completed with server error"
        );
    }
}

/// 详细请求日志中间件（可选）
///
/// 记录更详细的请求信息，包括请求头和查询参数
#[handler]
pub async fn detailed_request_logger(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let start_time = Instant::now();

    // 获取基本请求信息
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let path = req.uri().path().to_string();
    let query = req.uri().query().unwrap_or("").to_string();

    // 获取请求头信息
    let mut headers = Vec::new();
    for (name, value) in req.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            // 过滤敏感头信息
            if !is_sensitive_header(name.as_str()) {
                headers.push(format!("{}={}", name, value_str));
            }
        }
    }

    // 获取 request_id
    let request_id = depot
        .get::<Uuid>("request_id")
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4());

    // 记录详细请求信息
    info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        query = %query,
        headers = ?headers,
        "Detailed request info"
    );

    // 处理请求
    ctrl.call_next(req, depot, res).await;

    // 记录响应信息
    let duration = start_time.elapsed();
    let status_code = res.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    info!(
        request_id = %request_id,
        status_code = %status_code.as_u16(),
        duration_ms = %duration.as_millis(),
        "Request processing completed"
    );
}

/// 检查是否为敏感请求头
fn is_sensitive_header(header_name: &str) -> bool {
    let sensitive_headers = [
        "authorization",
        "cookie",
        "set-cookie",
        "x-api-key",
        "x-auth-token",
        "x-access-token",
    ];

    sensitive_headers
        .iter()
        .any(|&sensitive| header_name.to_lowercase() == sensitive)
}
