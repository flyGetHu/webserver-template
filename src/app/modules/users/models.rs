//! 用户模块的数据模型

use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use validator::Validate;
use crate::app::infrastructure::pagination::{PaginatedResponse, PaginationInfo};

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
    /// 分页信息
    pub pagination: PaginationInfo,
}

impl From<PaginatedResponse<crate::app::domain::models::User>> for UserListResponse {
    fn from(response: PaginatedResponse<crate::app::domain::models::User>) -> Self {
        let users = response.data.into_iter().map(|user| UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            age: user.age,
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }).collect();

        Self {
            users,
            pagination: response.pagination,
        }
    }
}
