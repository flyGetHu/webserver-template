use salvo::prelude::*;
use serde::Serialize;
use salvo::oapi::ToSchema;
use uuid::Uuid;

/// 统一API响应结构
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    /// 响应代码，200表示成功
    #[salvo(schema(example = 200))]
    code: i32,
    /// 响应消息
    #[salvo(schema(example = "success"))]
    message: String,
    /// 请求ID，用于链路追踪
    #[salvo(schema(example = "550e8400-e29b-41d4-a716-446655440000"))]
    request_id: String,
    /// 响应数据
    data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T, request_id: Uuid) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            request_id: request_id.to_string(),
            data,
        }
    }
}

impl<T> Scribe for ApiResponse<T>
where
    T: Serialize + Send + Sync,
{
    fn render(self, res: &mut Response) {
        res.status_code(StatusCode::OK);
        res.render(Json(self));
    }
}
