//! 统一错误处理模块
//!
//! 定义应用的统一错误类型，并实现axum的IntoResponse trait以提供统一的错误响应格式

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
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

/// 统一的API错误响应格式
#[derive(Serialize)]
pub struct ApiError {
    /// 错误代码，0表示成功，非0表示各种错误
    code: i32,
    /// 错误消息
    message: String,
    /// 请求ID
    request_id: String,
}

impl AppError {
    pub fn to_api_error(&self, request_id: Uuid) -> (StatusCode, ApiError) {
        let request_id = request_id.to_string();
        match self {
            AppError::Business(msg) => {
                error!("Business error: {}", msg);
                (
                    StatusCode::BAD_REQUEST,
                    ApiError {
                        code: 1000,
                        message: msg.clone(),
                        request_id,
                    },
                )
            }
            AppError::Validation(msg) => {
                error!("Validation error: {}", msg);
                (
                    StatusCode::BAD_REQUEST,
                    ApiError {
                        code: 2000,
                        message: msg.clone(),
                        request_id,
                    },
                )
            }
            AppError::Database(err) => {
                error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiError {
                        code: 3000,
                        message: "Internal server error".to_string(),
                        request_id,
                    },
                )
            }
            AppError::NotFound(msg) => {
                error!("Not found error: {}", msg);
                (
                    StatusCode::NOT_FOUND,
                    ApiError {
                        code: 4000,
                        message: msg.clone(),
                        request_id,
                    },
                )
            }
            AppError::Internal(err) => {
                error!("Internal server error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiError {
                        code: 5000,
                        message: "Internal server error".to_string(),
                        request_id,
                    },
                )
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 创建一个临时的响应
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // 将AppError字符串插入到响应的扩展中
        response.extensions_mut().insert(self.to_string());
        response
    }
}
