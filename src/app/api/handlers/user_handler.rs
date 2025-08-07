//! 用户相关处理函数模块
//!
//! 包含所有与用户相关的API端点处理函数

use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use salvo::oapi::ToSchema;

use crate::app::{
    api::response::ApiResponse,
    error::AppError,
};

/// 创建用户请求的数据传输对象
#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUserPayload {
    /// 用户名
    #[validate(length(min = 3))]
    #[salvo(schema(example = "zhangsan", min_length = 3))]
    pub username: String,
    /// 邮箱
    #[validate(email)]
    #[salvo(schema(example = "user@example.com", format = "email"))]
    pub email: String,
    /// 密码
    #[validate(length(min = 6))]
    #[salvo(schema(example = "password123", min_length = 6))]
    pub password: String,
    /// 年龄
    #[salvo(schema(example = 25, minimum = 0, maximum = 150))]
    pub age: Option<u32>,
}

/// 创建用户响应的数据传输对象
#[derive(Serialize, ToSchema)]
pub struct CreateUserResponse {
    /// 用户ID
    #[salvo(schema(example = 1))]
    pub id: i32,
    /// 用户名
    #[salvo(schema(example = "zhangsan"))]
    pub username: String,
    /// 邮箱
    #[salvo(schema(example = "user@example.com"))]
    pub email: String,
    /// 年龄
    #[salvo(schema(example = 25))]
    pub age: Option<u32>,
}

/// 创建用户
///
/// 创建新用户的API端点，需要验证用户输入数据。
/// 成功创建后返回新创建的用户信息。
#[handler]
pub async fn create_user(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = depot.get::<Uuid>("request_id").cloned().unwrap_or_else(|_| Uuid::new_v4());
    
    let payload = req.parse_json::<CreateUserPayload>().await.map_err(|e| AppError::Validation(e.to_string()))?;
    payload.validate().map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;
    
    // 简化处理，直接返回响应
    let response = CreateUserResponse {
        id: 1,
        username: payload.username,
        email: payload.email,
        age: payload.age,
    };

    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}
