//! JWT 认证中间件 (Hoop)
//!
//! 基于 salvo-template 的标准实现，融合 webserver-template 的业务特色

use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use salvo::jwt_auth::{ConstDecoder, CookieFinder, HeaderFinder, QueryFinder};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::app::{config::Config, error::AppError};

/// JWT 声明结构体
/// 
/// 融合了 salvo-template 的简洁性和 webserver-template 的业务需求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// 用户ID (保留 webserver-template 的业务字段)
    pub user_id: i32,
    /// 用户名
    pub username: String,
    /// 用户邮箱
    pub email: String,
    /// 用户角色
    pub roles: Vec<String>,
    /// 令牌过期时间 (Unix 时间戳)
    pub exp: i64,
    /// 令牌签发时间
    pub iat: i64,
}

impl JwtClaims {
    /// 创建新的 JWT 声明
    pub fn new(user_id: i32, username: String, email: String, roles: Vec<String>, expiry_seconds: i64) -> Self {
        let now = OffsetDateTime::now_utc();
        let exp = now + Duration::seconds(expiry_seconds);
        
        Self {
            user_id,
            username,
            email,
            roles,
            exp: exp.unix_timestamp(),
            iat: now.unix_timestamp(),
        }
    }

    /// 检查用户是否有指定角色
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }

    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        now > self.exp
    }
}

/// 创建 JWT 认证中间件 (Salvo 标准方式)
/// 
/// 基于 salvo-template 的实现，支持多种令牌获取方式
pub fn jwt_auth_hoop(config: &Config) -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(
        config.jwt.secret.as_bytes(),
    ))
    .finders(vec![
        Box::new(HeaderFinder::new()),           // 从 Authorization header 获取
        Box::new(QueryFinder::new("token")),     // 从查询参数获取
        Box::new(CookieFinder::new("jwt_token")), // 从 Cookie 获取
    ])
    .force_passed(false) // 允许无令牌访问，由具体路由决定是否需要认证
}

/// 生成 JWT 令牌
/// 
/// 融合两个项目的令牌生成逻辑
pub fn generate_token(config: &Config, user_id: i32, username: String, email: String, roles: Vec<String>) -> Result<String, AppError> {
    let claims = JwtClaims::new(user_id, username, email, roles, config.jwt.expiry);
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to generate JWT token: {}", e)))?;
    
    Ok(token)
}

/// 验证 JWT 令牌
/// 
/// 提供手动验证令牌的功能
pub fn verify_token(config: &Config, token: &str) -> Result<JwtClaims, AppError> {
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
        &validation,
    )
    .map_err(|e| AppError::Business(format!("Invalid JWT token: {}", e)))?;
    
    let claims = token_data.claims;
    
    // 检查令牌是否过期
    if claims.is_expired() {
        return Err(AppError::Business("JWT token has expired".to_string()));
    }
    
    Ok(claims)
}

/// 当前用户提取器 (保持与现有代码兼容)
#[derive(Debug, Clone)]
pub struct CurrentUser(pub JwtClaims);

impl CurrentUser {
    /// 获取用户ID
    pub fn user_id(&self) -> i32 {
        self.0.user_id
    }

    /// 获取用户名
    pub fn username(&self) -> &str {
        &self.0.username
    }

    /// 获取用户邮箱
    pub fn email(&self) -> &str {
        &self.0.email
    }

    /// 获取用户角色
    pub fn roles(&self) -> &[String] {
        &self.0.roles
    }

    /// 检查用户是否有指定角色
    pub fn has_role(&self, role: &str) -> bool {
        self.0.has_role(role)
    }
}

/// 从 Depot 中获取当前用户
/// 
/// 便捷函数，用于在处理器中获取当前认证用户
pub fn get_current_user(depot: &Depot) -> Result<CurrentUser, AppError> {
    depot
        .get::<JwtClaims>("jwt_claims")
        .map(|claims| CurrentUser(claims.clone()))
        .map_err(|_| AppError::Business("User not authenticated".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::config::JwtConfig;

    fn create_test_config() -> Config {
        Config {
            listen_addr: "127.0.0.1:3000".to_string(),
            server: crate::app::config::ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: crate::app::config::db_config::DatabaseConfig {
                url: "test".to_string(),
                pool_size: 10,
                min_idle: Some(5),
                tcp_timeout: 30,
                connection_timeout: 30000,
                statement_timeout: 30000,
                enforce_tls: false,
                helper_threads: 4,
            },
            redis: crate::app::config::RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
                timeout: 5000,
            },
            jwt: JwtConfig {
                secret: "test_secret_key_for_jwt_signing".to_string(),
                expiry: 3600, // 1 hour
            },
            log: crate::app::config::log_config::LogConfig::default(),
            tls: None,
        }
    }

    #[test]
    fn test_jwt_claims_creation() {
        let claims = JwtClaims::new(
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            vec!["user".to_string()],
            3600,
        );

        assert_eq!(claims.user_id, 1);
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.roles, vec!["user"]);
        assert!(!claims.is_expired());
        assert!(claims.has_role("user"));
        assert!(!claims.has_role("admin"));
    }

    #[test]
    fn test_token_generation_and_verification() {
        let config = create_test_config();
        
        let token = generate_token(
            &config,
            1,
            "testuser".to_string(),
            "test@example.com".to_string(),
            vec!["user".to_string()],
        ).unwrap();

        let claims = verify_token(&config, &token).unwrap();
        
        assert_eq!(claims.user_id, 1);
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.roles, vec!["user"]);
    }

    #[test]
    fn test_invalid_token_verification() {
        let config = create_test_config();
        let result = verify_token(&config, "invalid_token");
        assert!(result.is_err());
    }
}
