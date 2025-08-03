use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

/// 统一API响应结构
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    /// 响应代码，200表示成功
    #[schema(example = 200)]
    code: i32,
    /// 响应消息
    #[schema(example = "success")]
    message: String,
    /// 请求ID，用于链路追踪
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
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

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
