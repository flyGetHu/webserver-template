use std::sync::Arc;

use sqlx::MySqlPool;

use crate::app::{
    domain::models::{CreateUserDto, User},
    error::AppError,
};

/// 用户仓库 - 处理用户数据持久化
#[derive(Clone)]
pub struct UserRepository {
    db_pool: Arc<MySqlPool>,
}

impl UserRepository {
    /// 创建新的用户仓库实例
    pub fn new(db_pool: Arc<MySqlPool>) -> Self {
        Self { db_pool }
    }

    /// 创建新用户
    pub async fn create(&self, user_data: CreateUserDto, password_hash: String) -> Result<User, AppError> {
        let result = sqlx::query(
                r#"
                INSERT INTO users (username, email, password_hash, age, roles, is_active)
                VALUES (?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&user_data.username)
            .bind(&user_data.email)
            .bind(password_hash)
            .bind(user_data.age)
            .bind(serde_json::to_string(&vec!["user".to_string()]).unwrap())
            .bind(true)
            .execute(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        let user_id = result.last_insert_id() as i32;
        self.find_by_id(user_id).await?.ok_or(AppError::NotFound("User not found".to_string()))
    }

    /// 通过ID查找用户
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 通过用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 通过邮箱查找用户
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 通过用户名或邮箱查找用户
    pub async fn find_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::
            <_, User>("SELECT * FROM users WHERE username = ? OR email = ?")
            .bind(username_or_email)
            .bind(username_or_email)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(user)
    }

    /// 检查用户名是否存在
    pub async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::
            <_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)")
            .bind(username)
            .fetch_one(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(exists)
    }

    /// 检查邮箱是否存在
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar::
            <_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)")
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
                "SELECT * FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?"
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
        sqlx::query("UPDATE users SET is_active = ? WHERE id = ?")
            .bind(is_active)
            .bind(id)
            .execute(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        self.find_by_id(id).await?.ok_or(AppError::NotFound("User not found".to_string()))
    }

    /// 删除用户
    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(self.db_pool.as_ref())
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(())
    }
}