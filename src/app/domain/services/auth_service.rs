use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rbatis::RBatis;
use rbs::value;

use crate::app::{
    api::middleware::auth::Claims,
    config::Config,
    domain::models::{CreateUserDto, LoginUserDto, User},
    error::AppError,
};

/// 认证服务
pub struct AuthService {
    rb: Arc<RBatis>,
    config: Arc<Config>,
}

impl AuthService {
    /// 创建新的认证服务实例
    #[must_use]
    pub fn new(rb: Arc<RBatis>, config: Arc<Config>) -> Self {
        Self { rb, config }
    }

    /// 注册用户
    /// # Errors
    /// 返回数据库错误、业务错误或序列化错误
    pub async fn register_user(&self, user_data: CreateUserDto) -> Result<User, AppError> {
        // 检查用户名是否已存在
        let existing: Vec<User> = self.rb
            .query_decode(
                "SELECT * FROM users WHERE username = ? OR email = ?",
                vec![value!(&user_data.username), value!(&user_data.email)],
            )
            .await
            .map_err(AppError::Database)?;
        let existing_user = existing.into_iter().next();

        if existing_user.is_some() {
            return Err(AppError::Business("Username or email already exists".to_string()));
        }

        // 哈希密码
        let password_hash = Self::hash_password(&user_data.password)?;

        // 创建用户
        let exec = self.rb.exec(
            r"
            INSERT INTO users (username, email, password_hash, age, roles, is_active)
            VALUES (?, ?, ?, ?, ?, ?)
            ",
            vec![
                value!(&user_data.username),
                value!(&user_data.email),
                value!(password_hash),
                value!(user_data.age),
                value!(serde_json::to_string(&vec!["user".to_string()])
                    .map_err(|e| AppError::Internal(format!("serialize roles json failed: {e}")))?),
                value!(true),
            ],
        ).await.map_err(AppError::Database)?;

        let user_id: i64 = exec.last_insert_id.into();
        let user_id = i32::try_from(user_id)
            .map_err(|_| AppError::Internal("last_insert_id out of range".to_string()))?;
        let mut users: Vec<User> = self.rb
            .query_decode("SELECT * FROM users WHERE id = ?", vec![value!(user_id)])
            .await
            .map_err(AppError::Database)?;
        let user = users.pop().ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    /// 用户登录
    /// # Errors
    /// 返回认证相关错误或数据库错误
    pub async fn login_user(&self, login_data: LoginUserDto) -> Result<String, AppError> {
        // 查找用户
        let users: Vec<User> = self.rb
            .query_decode(
                "SELECT * FROM users WHERE (username = ? OR email = ?) AND is_active = true",
                vec![value!(&login_data.username_or_email), value!(&login_data.username_or_email)],
            )
            .await
            .map_err(AppError::Database)?;
        let user = users.into_iter().next();

        let user = user.ok_or_else(|| {
            AppError::Business("Invalid username/email or password".to_string())
        })?;

        // 验证密码
        if !Self::verify_password(&login_data.password, &user.password_hash)? {
            return Err(AppError::Business("Invalid username/email or password".to_string()));
        }

        // 生成JWT令牌
        let token = self.generate_jwt(&user)?;
        Ok(token)
    }

    /// 生成JWT令牌
    /// # Errors
    /// 生成 token 失败或时间转换失败
    pub fn generate_jwt(&self, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now() + Duration::seconds(self.config.jwt.expiry);

        let claims = Claims {
            user_id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
            exp: usize::try_from(expiration.timestamp())
                .map_err(|_| AppError::Internal("expiration timestamp out of range".to_string()))?,
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
    /// # Errors
    /// token 无效或过期
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
    fn hash_password(password: &str) -> Result<String, AppError> {
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
    /// # Errors
    /// 数据库查询失败
    pub async fn find_user_by_id(&self, user_id: i32) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self.rb
            .query_decode("SELECT * FROM users WHERE id = ?", vec![value!(user_id)])
            .await
            .map_err(AppError::Database)?;
        let user = users.into_iter().next();

        Ok(user)
    }
}