//! 用户模块的数据模型

use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use validator::Validate;

/// 创建用户请求
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct CreateUserRequest {
    /// 用户名
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    /// 邮箱
    #[validate(email)]
    pub email: String,
    /// 年龄
    #[validate(range(min = 1, max = 150))]
    pub age: Option<i32>,
}

/// 更新用户请求
#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct UpdateUserRequest {
    /// 用户名
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    /// 邮箱
    #[validate(email)]
    pub email: Option<String>,
    /// 年龄
    #[validate(range(min = 1, max = 150))]
    pub age: Option<i32>,
}

/// 用户响应
#[derive(Serialize, Debug, ToSchema)]
pub struct UserResponse {
    /// 用户ID
    pub id: i32,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 年龄
    pub age: Option<i32>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 用户列表响应
#[derive(Serialize, Debug, ToSchema)]
pub struct UserListResponse {
    /// 用户列表
    pub users: Vec<UserResponse>,
    /// 总数
    pub total: i64,
    /// 当前页
    pub page: i32,
    /// 每页大小
    pub page_size: i32,
}
