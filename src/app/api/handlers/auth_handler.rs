use salvo::prelude::*;
use uuid::Uuid;
use validator::Validate;

use crate::app::{
    api::response::ApiResponse,
    error::AppError,
};

/// 用户注册请求的数据传输对象
#[derive(serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct RegisterRequest {
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

/// 用户登录请求的数据传输对象
#[derive(serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct LoginRequest {
    /// 用户名或邮箱
    #[validate(length(min = 1, max = 50))]
    pub username_or_email: String,
    /// 密码
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

/// 认证响应的数据传输对象
#[derive(serde::Serialize)]
pub struct AuthResponse {
    /// JWT访问令牌
    pub token: String,
    /// 用户ID
    pub user_id: i32,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 用户角色
    pub roles: Vec<String>,
}

/// 用户注册处理函数
#[handler]
pub async fn register(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = depot.get::<Uuid>("request_id").cloned().unwrap_or_else(|_| Uuid::new_v4());
    
    let payload = req.parse_json::<RegisterRequest>().await.map_err(|e| AppError::Validation(e.to_string()))?;
    payload.validate().map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;
    
    // 简化处理，直接返回响应
    let response = AuthResponse {
        token: "fake_jwt_token".to_string(),
        user_id: 1,
        username: payload.username,
        email: payload.email,
        roles: vec!["user".to_string()],
    };

    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}

/// 用户登录处理函数
#[handler]
pub async fn login(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = depot.get::<Uuid>("request_id").cloned().unwrap_or_else(|_| Uuid::new_v4());
    
    let payload = req.parse_json::<LoginRequest>().await.map_err(|e| AppError::Validation(e.to_string()))?;
    payload.validate().map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;
    
    // 简化处理，直接返回响应
    let response = AuthResponse {
        token: "fake_jwt_token".to_string(),
        user_id: 1,
        username: payload.username_or_email,
        email: "user@example.com".to_string(),
        roles: vec!["user".to_string()],
    };

    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}

/// 用户注销处理函数
#[handler]
pub async fn logout(
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = depot.get::<Uuid>("request_id").cloned().unwrap_or_else(|_| Uuid::new_v4());
    
    // JWT 是无状态的，注销操作通常由客户端处理
    // 这里可以添加token黑名单等逻辑
    res.render(Json(ApiResponse::new((), request_id)));
    Ok(())
}