use salvo::prelude::*;
use tracing::info_span;
use tracing::Instrument;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

/// 请求ID中间件
pub fn request_id_middleware() -> RequestId {
    RequestId::new()
}

#[handler]
pub async fn add_request_id(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    // 生成请求ID
    let request_id = Uuid::new_v4().to_string();
    
    // 将请求ID存储到depot中供后续使用
    depot.insert("request_id", request_id.clone());
    
    // 创建包含请求ID的span
    let span = info_span!("request", request_id = %request_id);
    
    // 继续处理请求
    async move {
        ctrl.call_next(req, depot, res).await;
        
        // 确保响应头中包含请求ID
        if !res.headers().contains_key(REQUEST_ID_HEADER) {
            res.headers_mut()
                .insert(REQUEST_ID_HEADER, request_id.parse().unwrap());
        }
    }.instrument(span).await;
}
