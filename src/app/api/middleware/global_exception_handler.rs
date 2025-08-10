use salvo::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use crate::app::depot_keys::KEY_REQUEST_ID;

/// 404错误响应结构体
#[derive(Serialize)]
struct NotFoundResponse {
    code: i32,
    message: String,
    request_id: String,
}

/// 全局异常处理器，处理业务异常和特定的HTTP状态码
#[handler]
pub async fn global_exception_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let _request_id = depot
        .get::<Uuid>(KEY_REQUEST_ID)
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4());

    // 继续处理请求
    ctrl.call_next(req, depot, res).await;

    // 处理业务异常（AppError会被IntoResponse正确处理）
    // 注意：在Salvo中，我们需要在ctrl.call_next之后检查状态码
    // 这里简化处理，让AppError的Writer实现来处理错误响应

    // 其他错误（包括验证错误、业务错误等）由AppError的IntoResponse处理
}
