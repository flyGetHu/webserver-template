//! 自定义提取器模块
//!
//! 提供自定义的salvo提取器，例如用于自动验证请求体的提取器

use salvo::prelude::*;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app::{api::middleware::auth::CurrentUser, error::AppError};

/// 一个自定义提取器，它包装了`salvo::JsonBody`，并在反序列化后自动验证数据
///
/// 如果验证失败，它会返回一个`AppError::Validation`错误
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T> ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
{
    /// # Errors
    /// 解析或校验失败返回 `AppError`
    pub async fn extract(req: &mut Request) -> Result<Self, AppError> {
        let value = req.parse_json::<T>().await
            .map_err(|e| AppError::Validation(e.to_string()))?;

        value
            .validate()
            .map_err(|e| AppError::Validation(format!("Validation failed: {e}")))?;

        Ok(ValidatedJson(value))
    }
}

/// 当前用户提取器
///
/// 从请求扩展中提取当前认证用户的Claims
impl CurrentUser {
    /// # Errors
    /// 未认证或上下文缺失时返回 `AppError`
    pub fn extract(depot: &mut Depot) -> Result<Self, AppError> {
        if let Ok(current_user) = depot.get::<CurrentUser>("current_user") {
            Ok(current_user.clone())
        } else {
            Err(AppError::Business("User not authenticated".to_string()))
        }
    }
}