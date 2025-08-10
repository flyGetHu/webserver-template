use std::sync::Arc;

use rbatis::RBatis;
use rbs::value;

use crate::app::{
    domain::models::{CreateUserDto, User},
    error::AppError,
};

/// 用户仓库 - 处理用户数据持久化
#[derive(Clone)]
pub struct UserRepository {
    rb: Arc<RBatis>,
}

impl UserRepository {
    /// 创建新的用户仓库实例
    #[must_use]
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    /// 创建新用户
    /// # Errors
    /// 可能返回数据库错误、序列化错误或范围转换错误
    pub async fn create(&self, user_data: CreateUserDto, password_hash: String) -> Result<User, AppError> {
        let sql = r"
            INSERT INTO users (username, email, password_hash, age, roles, is_active)
            VALUES (?, ?, ?, ?, ?, ?)
        ";
        let args = vec![
            value!(&user_data.username),
            value!(&user_data.email),
            value!(password_hash),
            value!(user_data.age),
            value!(serde_json::to_string(&vec!["user".to_string()])
                .map_err(|e| AppError::Internal(format!("serialize roles json failed: {e}")))?),
            value!(true),
        ];
        let exec = self.rb.exec(sql, args).await.map_err(AppError::Database)?;
        let user_id: i64 = exec.last_insert_id.into();
        let user_id = i32::try_from(user_id)
            .map_err(|_| AppError::Internal("last_insert_id out of range".to_string()))?;
        self.find_by_id(user_id).await?.ok_or(AppError::NotFound("User not found".to_string()))
    }

    /// 通过ID查找用户
    /// # Errors
    /// 数据库错误
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode("SELECT * FROM users WHERE id = ?", vec![value!(id)])
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 通过用户名查找用户
    /// # Errors
    /// 数据库错误
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode("SELECT * FROM users WHERE username = ?", vec![value!(username)])
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 通过邮箱查找用户
    /// # Errors
    /// 数据库错误
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode("SELECT * FROM users WHERE email = ?", vec![value!(email)])
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 通过用户名或邮箱查找用户
    /// # Errors
    /// 数据库错误
    pub async fn find_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode(
                "SELECT * FROM users WHERE username = ? OR email = ?",
                vec![value!(username_or_email), value!(username_or_email)],
            )
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 检查用户名是否存在
    /// # Errors
    /// 数据库错误
    pub async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
        let rows: Vec<i64> = self
            .rb
            .query_decode(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)",
                vec![value!(username)],
            )
            .await
            .map_err(AppError::Database)?;
        Ok(rows.into_iter().next().unwrap_or(0) != 0)
    }

    /// 检查邮箱是否存在
    /// # Errors
    /// 数据库错误
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let rows: Vec<i64> = self
            .rb
            .query_decode(
                "SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)",
                vec![value!(email)],
            )
            .await
            .map_err(AppError::Database)?;
        Ok(rows.into_iter().next().unwrap_or(0) != 0)
    }

    /// 获取所有用户（分页）
    /// # Errors
    /// 数据库错误
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode(
                "SELECT * FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?",
                vec![value!(limit), value!(offset)],
            )
            .await
            .map_err(AppError::Database)?;
        Ok(users)
    }

    /// 更新用户状态
    /// # Errors
    /// 数据库错误或未找到
    pub async fn update_user_status(&self, id: i32, is_active: bool) -> Result<User, AppError> {
        self
            .rb
            .exec(
                "UPDATE users SET is_active = ? WHERE id = ?",
                vec![value!(is_active), value!(id)],
            )
            .await
            .map_err(AppError::Database)?;

        self.find_by_id(id).await?.ok_or(AppError::NotFound("User not found".to_string()))
    }

    /// 删除用户
    /// # Errors
    /// 数据库错误
    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        self
            .rb
            .exec("DELETE FROM users WHERE id = ?", vec![value!(id)])
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}