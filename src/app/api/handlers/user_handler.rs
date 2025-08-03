//! 用户相关处理函数模块
//!
//! 包含所有与用户相关的API端点处理函数

use axum::Extension;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

use crate::app::api::extractors::ValidatedJson;
use crate::app::api::response::ApiResponse;
use crate::app::error::AppError;

/// 创建用户请求的数据传输对象
#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUserPayload {
    /// 用户名
    #[validate(length(min = 3), required)]
    #[schema(example = "zhangsan", min_length = 3)]
    pub username: Option<String>,
    /// 邮箱
    #[validate(email, required)]
    #[schema(example = "user@example.com", format = "email")]
    pub email: Option<String>,
    /// 年龄
    #[schema(example = 25, minimum = 0, maximum = 150)]
    pub age: Option<u32>,
}

/// 创建用户响应的数据传输对象
#[derive(Serialize, ToSchema)]
pub struct CreateUserResponse {
    /// 用户ID
    #[schema(example = 1)]
    pub id: i32,
    /// 用户名
    #[schema(example = "zhangsan")]
    pub username: String,
    /// 邮箱
    #[schema(example = "user@example.com")]
    pub email: String,
    /// 年龄
    #[schema(example = 25)]
    pub age: Option<u32>,
}

/// 创建用户
///
/// 创建新用户的API端点，需要验证用户输入数据。
/// 成功创建后返回新创建的用户信息。
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "用户管理",
    request_body = CreateUserPayload,
    responses(
        (status = 200, description = "用户创建成功", body = ApiResponse<CreateUserResponse>),
        (status = 400, description = "请求参数验证失败"),
        (status = 500, description = "服务器内部错误")
    ),
    operation_id = "create_user"
)]
pub async fn create_user(
    Extension(request_id): Extension<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
) -> Result<ApiResponse<CreateUserResponse>, AppError> {
    // 在实际应用中，这里会调用领域服务或仓库来创建用户
    // 目前我们只是模拟创建过程
    let user = CreateUserResponse {
        id: 1,
        username: payload.username.unwrap(),
        email: payload.email.unwrap(),
        age: payload.age,
    };

    Ok(ApiResponse::new(user, request_id))
}
