//! 认证模块的数据模型

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::domain::models::CreateUserDto;

/// 用户注册请求的数据传输对象
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct RegisterRequest {
    /// 用户名
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 密码
    #[validate(length(min = 6, max = 128))]
    pub password: String,
    /// 年龄
    #[validate(range(min = 1, max = 150))]
    pub age: Option<i32>,
}

/// 用户登录请求的数据传输对象
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct LoginRequest {
    /// 用户名或邮箱
    #[validate(length(min = 1, max = 50))]
    pub username_or_email: String,
    /// 密码
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

/// 认证响应的数据传输对象
#[derive(Serialize, Debug, ToSchema)]
pub struct AuthResponse {
    /// JWT访问令牌
    pub token: String,
    /// 用户ID
    pub user_id: i32,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 用户角色
    pub roles: Vec<String>,
}

/// 从 `RegisterRequest` 转换为 `CreateUserDto`
impl From<RegisterRequest> for CreateUserDto {
    fn from(request: RegisterRequest) -> Self {
        Self {
            username: request.username,
            email: request.email,
            password: request.password,
            age: request.age,
        }
    }
}
