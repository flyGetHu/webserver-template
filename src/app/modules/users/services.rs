//! 用户服务层
//!
//! 处理用户管理相关的业务逻辑

use std::sync::Arc;
use validator::Validate;

use std::collections::HashMap;

use crate::app::{
    domain::models::User,
    error::AppError,
    infrastructure::pagination::PaginationParams,
    infrastructure::persistence::UserRepository,
    modules::users::models::{CreateUserRequest, UpdateUserRequest, UserResponse},
};

/// 用户服务
pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    /// 创建新的用户服务实例
    #[must_use]
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    /// 获取用户列表（支持分页和搜索）
    /// # Errors
    /// 验证失败时返回 `AppError::Validation`
    pub async fn list_users(
        &self,
        params: PaginationParams,
        _query_params: HashMap<String, rbs::Value>,
    ) -> Result<crate::app::infrastructure::pagination::PaginatedResponse<User>, AppError> {
        self.user_repository.find_users_paginated(&params).await
    }

    /// 根据关键词搜索用户
    /// # Errors
    /// 验证失败时返回 `AppError::Validation`
    pub async fn search_by_keyword(
        &self,
        params: PaginationParams,
        keyword: Option<String>,
    ) -> Result<crate::app::infrastructure::pagination::PaginatedResponse<User>, AppError> {
        let mut query_params = HashMap::new();
        if let Some(kw) = keyword {
            if !kw.trim().is_empty() {
                query_params.insert("username".to_string(), rbs::Value::String(format!("%{}%", kw.trim())));
                query_params.insert("email".to_string(), rbs::Value::String(format!("%{}%", kw.trim())));
            }
        }
        self.user_repository.find_users_paginated(&params).await
    }

    /// 根据ID获取用户
    /// # Errors
    /// 当 `id` 非法时返回 `NotFound`
    pub fn get_user(&self, id: i32) -> Result<UserResponse, AppError> {
        // TODO: 实际的查询逻辑
        // 1. 从数据库根据ID查询用户
        // 2. 如果不存在返回错误

        // 临时实现
        if id <= 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(UserResponse {
            id,
            username: format!("user{id}"),
            email: format!("user{id}@example.com"),
            age: Some(25),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        })
    }

    /// 创建用户
    /// # Errors
    /// 验证失败时返回 `AppError::Validation`
    pub fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse, AppError> {
        // 验证请求数据
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {e}")))?;

        // TODO: 实际的创建逻辑
        // 1. 检查用户名和邮箱是否已存在
        // 2. 保存用户到数据库
        // 3. 返回创建的用户信息

        // 临时实现
        Ok(UserResponse {
            id: 1,
            username: request.username,
            email: request.email,
            age: request.age,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        })
    }

    /// 更新用户
    /// # Errors
    /// 验证失败或用户不存在时返回错误
    pub fn update_user(
        &self,
        id: i32,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        // 验证请求数据
        request
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {e}")))?;

        // TODO: 实际的更新逻辑
        // 1. 检查用户是否存在
        // 2. 更新用户信息
        // 3. 返回更新后的用户信息

        // 临时实现
        if id <= 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(UserResponse {
            id,
            username: request.username.unwrap_or_else(|| format!("user{id}")),
            email: request
                .email
                .unwrap_or_else(|| format!("user{id}@example.com")),
            age: request.age,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        })
    }

    /// 删除用户
    /// # Errors
    /// 用户不存在时返回错误
    pub fn delete_user(&self, id: i32) -> Result<(), AppError> {
        // TODO: 实际的删除逻辑
        // 1. 检查用户是否存在
        // 2. 删除用户

        // 临时实现
        if id <= 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }
}
