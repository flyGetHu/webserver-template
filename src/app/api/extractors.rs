
//! 自定义提取器模块
//!
//! 提供自定义的axum提取器，例如用于自动验证请求体的提取器

use async_trait::async_trait;
use axum::{extract::FromRequest, http::Request, Json, body::Body};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app::{api::middleware::auth::CurrentUser, error::AppError};

/// 一个自定义提取器，它包装了`axum::Json`，并在反序列化后自动验证数据
///
/// 如果验证失败，它会返回一个`AppError::Validation`错误
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::Validation(e.to_string()))?;

        value
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        Ok(ValidatedJson(value))
    }
}

/// 当前用户提取器
///
/// 从请求扩展中提取当前认证用户的Claims
#[async_trait]
impl<S> FromRequest<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        req.extensions()
            .get::<CurrentUser>()
            .cloned()
            .ok_or_else(|| AppError::Business("User not authenticated".to_string()))
    }
}
