//! 用户服务模块

use std::sync::Arc;

use crate::app::{
    config::Config, domain::models::User, error::AppError,
    infrastructure::persistence::user_repository::UserRepository,
};

/// 用户服务
///
/// 处理用户相关的业务逻辑
pub struct UserService {
    user_repository: Arc<UserRepository>,
    config: Arc<Config>,
}

impl UserService {
    /// 创建新的用户服务实例
    pub fn new(user_repository: Arc<UserRepository>, config: Arc<Config>) -> Self {
        Self {
            user_repository,
            config,
        }
    }

    /// 获取所有用户（分页）
    pub async fn get_all_users(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        self.user_repository.find_all(limit, offset).await
    }

    /// 通过ID获取用户
    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        self.user_repository.find_by_id(id).await
    }

    /// 通过用户名获取用户
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        self.user_repository.find_by_username(username).await
    }

    /// 检查用户名是否存在
    pub async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
        self.user_repository.username_exists(username).await
    }

    /// 检查邮箱是否存在
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        self.user_repository.email_exists(email).await
    }

    /// 更新用户状态
    pub async fn update_user_status(&self, id: i32, is_active: bool) -> Result<User, AppError> {
        self.user_repository.update_user_status(id, is_active).await
    }

    /// 删除用户
    pub async fn delete_user(&self, id: i32) -> Result<(), AppError> {
        self.user_repository.delete(id).await
    }
}
