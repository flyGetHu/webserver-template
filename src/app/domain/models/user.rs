use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// 用户领域模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// 用户ID
    pub id: i32,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 密码哈希
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// 年龄
    pub age: Option<i32>,
    /// 用户角色列表
    #[sqlx(json)]
    pub roles: Vec<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 是否激活
    pub is_active: bool,
}

/// 创建用户的数据传输对象
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
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

/// 用户登录的数据传输对象
#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserDto {
    /// 用户名或邮箱
    #[validate(length(min = 1, max = 50))]
    pub username_or_email: String,
    /// 密码
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

/// 用户响应的数据传输对象
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// 用户ID
    pub id: i32,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 年龄
    pub age: Option<i32>,
    /// 用户角色列表
    pub roles: Vec<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 是否激活
    pub is_active: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            age: user.age,
            roles: user.roles,
            created_at: user.created_at,
            is_active: user.is_active,
        }
    }
}