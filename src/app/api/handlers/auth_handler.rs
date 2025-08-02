use axum::{
    extract::State,
    Extension,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::app::{
    api::{extractors::ValidatedJson, response::ApiResponse},
    config::Config,
    domain::{models::LoginUserDto, services::AuthService},
    error::AppError,
    state::AppState,
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
pub async fn register(
    State(state): State<AppState>,
    State(config): State<Arc<Config>>,
    Extension(request_id): Extension<Uuid>,
    ValidatedJson(payload): ValidatedJson<RegisterRequest>,
) -> Result<ApiResponse<AuthResponse>, AppError> {
    let auth_service = AuthService::new(
        std::sync::Arc::new(state.db_pool.clone()),
        config.clone(),
    );

    let create_user_dto = crate::app::domain::models::CreateUserDto {
        username: payload.username,
        email: payload.email,
        password: payload.password,
        age: payload.age,
    };

    let user = auth_service.register_user(create_user_dto).await?;
    let token = auth_service.generate_jwt(&user)?;

    let response = AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
        email: user.email,
        roles: user.roles,
    };

    Ok(ApiResponse::new(response, request_id))
}

/// 用户登录处理函数
pub async fn login(
    State(state): State<AppState>,
    State(config): State<Arc<Config>>,
    Extension(request_id): Extension<Uuid>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<ApiResponse<AuthResponse>, AppError> {
    let auth_service = AuthService::new(
        std::sync::Arc::new(state.db_pool.clone()),
        config.clone(),
    );

    let login_dto = LoginUserDto {
        username_or_email: payload.username_or_email,
        password: payload.password,
    };

    let token = auth_service.login_user(login_dto).await?;
    
    // 获取用户信息
    let user = auth_service.find_user_by_id(
        jsonwebtoken::decode::<
            crate::app::api::middleware::auth::Claims,
        >(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(config.jwt.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AppError::Business("Invalid token".to_string()))?
        .claims
        .user_id,
    )
    .await?
    .ok_or_else(|| AppError::Business("User not found".to_string()))?;

    let response = AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
        email: user.email,
        roles: user.roles,
    };

    Ok(ApiResponse::new(response, request_id))
}

/// 获取当前用户信息处理函数
pub async fn me(
    Extension(request_id): Extension<Uuid>,
    current_user: crate::app::api::middleware::auth::CurrentUser,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    let response = json!({
        "id": current_user.0.user_id,
        "username": current_user.0.username,
        "email": current_user.0.email,
        "roles": current_user.0.roles,
    });

    Ok(ApiResponse::new(response, request_id))
}