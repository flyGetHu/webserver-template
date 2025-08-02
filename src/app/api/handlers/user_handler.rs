//! 用户相关处理函数模块
//!
//! 包含所有与用户相关的API端点处理函数

use axum::Extension;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::app::api::extractors::ValidatedJson;
use crate::app::api::response::ApiResponse;
use crate::app::error::AppError;

/// 创建用户请求的数据传输对象
#[derive(Deserialize, Validate)]
pub struct CreateUserPayload {
    /// 用户名
    #[validate(length(min = 3), required)]
    pub username: Option<String>,
    /// 邮箱
    #[validate(email, required)]
    pub email: Option<String>,
    /// 年龄
    pub age: Option<u32>,
}

/// 创建用户响应的数据传输对象
#[derive(Serialize)]
pub struct CreateUserResponse {
    /// 用户ID
    pub id: i32,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 年龄
    pub age: Option<u32>,
}

/// 创建用户处理函数
///
/// 处理创建新用户的POST请求
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
