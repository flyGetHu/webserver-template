use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::MySqlPool;

use crate::app::{
    api::middleware::auth::Claims,
    config::Config,
    domain::models::{CreateUserDto, LoginUserDto, User},
    error::AppError,
};

/// 认证服务
pub struct AuthService {
    db_pool: Arc<MySqlPool>,
    config: Arc<Config>,
}

impl AuthService {
    /// 创建新的认证服务实例
    pub fn new(db_pool: Arc<MySqlPool>, config: Arc<Config>) -> Self {
        Self { db_pool, config }
    }

    /// 注册用户
    pub async fn register_user(&self,
        user_data: CreateUserDto,
    ) -> Result<User, AppError> {
        // 检查用户名是否已存在
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = ? OR email = ?"
        )
        .bind(&user_data.username)
        .bind(&user_data.email)
        .fetch_optional(self.db_pool.as_ref())
        .await
        .map_err(AppError::Database)?;

        if existing_user.is_some() {
            return Err(AppError::Business("Username or email already exists".to_string()));
        }

        // 哈希密码
        let password_hash = self.hash_password(&user_data.password)?;

        // 创建用户
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
        .map_err(AppError::Database)?;

        let user_id = result.last_insert_id() as i32;
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(self.db_pool.as_ref())
            .await
            .map_err(AppError::Database)?;

        Ok(user)
    }

    /// 用户登录
    pub async fn login_user(&self, login_data: LoginUserDto) -> Result<String, AppError> {
        // 查找用户
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE (username = ? OR email = ?) AND is_active = true"
        )
        .bind(&login_data.username_or_email)
        .bind(&login_data.username_or_email)
        .fetch_optional(self.db_pool.as_ref())
        .await
        .map_err(AppError::Database)?;

        let user = user.ok_or_else(|| {
            AppError::Business("Invalid username/email or password".to_string())
        })?;

        // 验证密码
        if !self.verify_password(&login_data.password, &user.password_hash)? {
            return Err(AppError::Business("Invalid username/email or password".to_string()));
        }

        // 生成JWT令牌
        let token = self.generate_jwt(&user)?;
        Ok(token)
    }

    /// 生成JWT令牌
    pub fn generate_jwt(&self, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now()
            + Duration::seconds(self.config.jwt.expiry);

        let claims = Claims {
            user_id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
            exp: expiration.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt.secret.as_bytes()),
        )
        .map_err(|_| AppError::Business("Failed to generate JWT token".to_string()))?;

        Ok(token)
    }

    /// 验证JWT令牌
    pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
        let config = Config::load().map_err(|_| {
            AppError::Business("Failed to load configuration".to_string())
        })?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Business("Invalid or expired token".to_string()))?;

        Ok(token_data.claims)
    }

    /// 哈希密码
    fn hash_password(&self, password: &str,
    ) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Business("Failed to hash password".to_string()))?
            .to_string();

        Ok(password_hash)
    }

    /// 验证密码
    fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| AppError::Business("Invalid password hash".to_string()))?;

        let argon2 = Argon2::default();
        let is_valid = argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    }

    /// 通过ID查找用户
    pub async fn find_user_by_id(&self, user_id: i32,
    ) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(self.db_pool.as_ref())
            .await
            .map_err(AppError::Database)?;

        Ok(user)
    }
}