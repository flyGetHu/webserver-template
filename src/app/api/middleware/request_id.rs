use salvo::http::HeaderName;
use salvo::prelude::*;
use tracing::info_span;
use tracing::Instrument;
use uuid::Uuid;

/// 创建配置好的 `RequestId` 中间件
///
/// 根据 Salvo 官方文档，使用内置的 `RequestId` 中间件
/// 参考: <https://salvo.rs/zh-hans/guide/features/request-id.html>
#[must_use]
pub fn create_request_id_middleware() -> RequestId {
    RequestId::new().header_name(HeaderName::from_static("x-request-id")) // 设置响应头名称
}

/// 自定义 request_id 处理器，用于集成 tracing 和 depot
#[handler]
pub async fn request_id_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // 从请求头或 Salvo 内置中间件获取 request_id
    let request_id = req
        .header::<String>("x-request-id")
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // 将 request_id 存储到 depot 中供后续使用
    let request_uuid = Uuid::parse_str(&request_id).unwrap_or_else(|_| Uuid::new_v4());
    depot.insert("request_id", request_uuid);

    // 创建包含 request_id 的 tracing span
    let span = info_span!("request", request_id = %request_id);

    // 在 span 上下文中处理请求
    async move {
        ctrl.call_next(req, depot, res).await;
    }
    .instrument(span)
    .await;
}
