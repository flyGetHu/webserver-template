//! 统一错误处理模块
//!
//! 定义应用的统一错误类型，并实现salvo的Writer trait以提供统一的错误响应格式

use salvo::prelude::*;
use serde::Serialize;
use serde_json::Value;
use tracing::error;
use uuid::Uuid;

/// 统一的应用错误类型
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    /// 业务逻辑错误
    #[error("Business error: {0}")]
    Business(String),

    /// 验证错误
    #[error("Validation error: {0}")]
    Validation(String),

    /// 数据库错误
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    /// 未找到资源错误
    #[error("Not found: {0}")]
    NotFound(String),

    /// 其他内部错误
    #[error("Internal server error")]
    Internal(String),
}

/// 统一的API响应格式（包括成功和错误）
#[derive(Serialize)]
pub struct ApiError {
    /// 响应代码，200表示成功，非200表示各种错误
    code: i32,
    /// 响应消息
    message: String,
    /// 请求ID
    request_id: String,
    /// 响应数据
    data: Value,
}

impl AppError {
    pub fn to_api_error(&self, request_id: Uuid) -> (StatusCode, ApiError) {
        let request_id = request_id.to_string();
        match self {
            AppError::Business(msg) => {
                error!("Business error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiError {
                        code: 500,
                        message: msg.clone(),
                        request_id,
                        data: Value::Null,
                    },
                )
            }
            AppError::Validation(msg) => {
                error!("Validation error: {}", msg);
                (
                    StatusCode::BAD_REQUEST,
                    ApiError {
                        code: 400,
                        message: msg.clone(),
                        request_id,
                        data: Value::Null,
                    },
                )
            }
            AppError::Database(err) => {
                error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiError {
                        code: 500,
                        message: "Internal server error".to_string(),
                        request_id,
                        data: Value::Null,
                    },
                )
            }
            AppError::NotFound(msg) => {
                error!("Not found error: {}", msg);
                (
                    StatusCode::NOT_FOUND,
                    ApiError {
                        code: 404,
                        message: msg.clone(),
                        request_id,
                        data: Value::Null,
                    },
                )
            }
            AppError::Internal(err) => {
                error!("Internal server error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiError {
                        code: 500,
                        message: "Internal server error".to_string(),
                        request_id,
                        data: Value::Null,
                    },
                )
            }
        }
    }

    /// 创建特定HTTP状态码的错误
    pub fn unauthorized(message: String) -> Self {
        AppError::Business(message)
    }

    pub fn forbidden(message: String) -> Self {
        AppError::Business(message)
    }

    pub fn too_many_requests(message: String) -> Self {
        AppError::Business(message)
    }

    pub fn method_not_allowed(message: String) -> Self {
        AppError::Business(message)
    }
}

impl Scribe for AppError {
    fn render(self, res: &mut Response) {
        // 生成一个随机的request_id
        let request_id = Uuid::new_v4();
        
        let (status, api_error) = self.to_api_error(request_id);
        res.status_code(status);
        res.render(Json(api_error));
    }
}
