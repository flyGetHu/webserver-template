//! 认证中间件模块

use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::error::AppError;

/// JWT令牌中的声明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户ID
    pub user_id: i32,
    /// 用户名
    pub username: String,
    /// 用户邮箱
    pub email: String,
    /// 用户角色
    pub roles: Vec<String>,
    /// 令牌过期时间
    pub exp: usize,
}

/// 当前用户提取器
#[derive(Debug, Clone)]
pub struct CurrentUser(pub Claims);

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

    /// 检查用户是否有指定角色
    pub fn has_role(&self, role: &str) -> bool {
        self.0.roles.contains(&role.to_string())
    }
}

/// JWT认证中间件
#[handler]
pub async fn jwt_auth(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) -> Result<(), AppError> {
    use salvo::http::header::AUTHORIZATION;
    
    // 从请求头中获取Authorization
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| AppError::Business("Missing authorization header".to_string()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError::Business("Invalid authorization header format".to_string()))?;

    // 检查Bearer格式
    if !auth_str.starts_with("Bearer ") {
        return Err(AppError::Business("Invalid authorization header format".to_string()));
    }

    let token = auth_str.trim_start_matches("Bearer ");

    // 验证JWT令牌
    let claims = crate::app::domain::services::AuthService::verify_jwt(token)?;

    // 将claims添加到depot中
    depot.insert("current_user", CurrentUser(claims));

    // 继续处理请求
    ctrl.call_next(req, depot, res).await;
    Ok(())
}

/// 可选的JWT认证中间件
/// 如果提供了有效的JWT令牌，则设置用户，否则继续处理
#[handler]
pub async fn optional_jwt_auth(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    use salvo::http::header::AUTHORIZATION;
    
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ");
                if let Ok(claims) = crate::app::domain::services::AuthService::verify_jwt(token) {
                    depot.insert("current_user", CurrentUser(claims));
                }
            }
        }
    }

    // 继续处理请求
    ctrl.call_next(req, depot, res).await;
}