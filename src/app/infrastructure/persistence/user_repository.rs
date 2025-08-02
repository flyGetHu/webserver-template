use std::sync::Arc;

use sqlx::PgPool;

use crate::app::{
    domain::models::{CreateUserDto, User},
    error::AppError,
};

/// 用户仓库 - 处理用户数据持久化
#[derive(Clone)]
pub struct UserRepository {
    db_pool: Arc<PgPool>,
}

impl UserRepository {
    /// 创建新的用户仓库实例
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    /// 创建新用户
    pub async fn create(&self, user_data: CreateUserDto, password_hash: String) -> Result<User, AppError> {
        let user = sqlx::query_as::
            <_, User>(
                r#"
                INSERT INTO users (username, email, password_hash, age, roles, is_active)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
                "#
            )
            .bind(&user_data.username)
            .bind(&user_data.email)
            .bind(password_hash)
            .bind(user_data.age)
            .bind(vec!["user".to_string()])
            .bind(true)
            .fetch_one(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 通过ID查找用户
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 通过用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 通过邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 通过用户名或邮箱查找用户
    pub async fn find_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE username = $1 OR email = $1")
            .bind(username_or_email)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 检查用户名是否存在
    pub async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::
            <_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(username)
            .fetch_one(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(exists)
    }

    /// 检查邮箱是否存在
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::
            <_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email)
            .fetch_one(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(exists)
    }

    /// 获取所有用户（分页）
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::
            <_, User>(
                "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(users)
    }

    /// 更新用户状态
    pub async fn update_user_status(&self, id: i32, is_active: bool) -> Result<User, AppError> {
        let user = sqlx::query_as::
            <_, User>(
                "UPDATE users SET is_active = $1 WHERE id = $2 RETURNING *"
            )
            .bind(is_active)
            .bind(id)
            .fetch_one(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 删除用户
    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(())
    }
}