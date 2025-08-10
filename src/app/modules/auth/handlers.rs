//! 认证处理器
//!
//! 处理认证相关的HTTP请求

use salvo::prelude::*;
use uuid::Uuid;

use crate::app::{
    api::response::ApiResponse, container::DepotServiceExt, error::AppError,
    modules::auth::models::{AuthResponse, LoginRequest, RegisterRequest},
};

/// 用户注册处理器
#[endpoint(
    tags("Authentication"),
    operation_id = "register",
    responses(
        (status_code = 200, description = "User registered successfully", body = AuthResponse),
        (status_code = 400, description = "Invalid request data"),
        (status_code = 409, description = "User already exists")
    )
)]
pub async fn register(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    // 获取 request_id
    let request_id = depot
        .get::<Uuid>("request_id")
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4());

    // 解析请求数据
    let payload = req
        .parse_json::<RegisterRequest>()
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // 使用新的依赖注入系统获取认证服务
    let auth_service = depot.get_auth_service()?;

    // 调用认证服务进行用户注册
    let user = auth_service.register_user(payload.into()).await?;

    // 生成 JWT token
    let token = auth_service.generate_jwt(&user)?;

    // 构建响应
    let response = AuthResponse {
        token,
        user_id: user.id,
        username: user.username,
        email: user.email,
        roles: vec!["user".to_string()], // TODO: 从用户角色表获取
    };

    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}

/// 用户登录处理器
#[endpoint(
    tags("Authentication"),
    operation_id = "login",
    responses(
        (status_code = 200, description = "User logged in successfully", body = AuthResponse),
        (status_code = 400, description = "Invalid request data"),
        (status_code = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = depot
        .get::<Uuid>("request_id")
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4());

    let payload = req
        .parse_json::<LoginRequest>()
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // 临时实现 - 直接返回模拟响应
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

/// 用户注销处理器
#[endpoint(
    tags("Authentication"),
    operation_id = "logout",
    responses(
        (status_code = 200, description = "User logged out successfully"),
        (status_code = 401, description = "Unauthorized")
    )
)]
pub async fn logout(depot: &mut Depot, res: &mut Response) -> Result<(), AppError> {
    let request_id = depot
        .get::<Uuid>("request_id")
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4());

    // TODO: 从请求头获取token
    // 临时实现 - JWT 是无状态的，注销操作通常由客户端处理

    res.render(Json(ApiResponse::new((), request_id)));
    Ok(())
}
