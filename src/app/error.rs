//! 统一错误处理模块
//!
//! 融合 salvo-template 的标准化实现和 webserver-template 的业务特色
//! 提供统一的错误类型和响应格式

use salvo::http::{ParseError, StatusCode, StatusError};
use salvo::oapi::{self, EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

/// 统一的应用错误类型
///
/// 融合 salvo-template 的标准化错误类型和 webserver-template 的业务特色
#[derive(Error, Debug)]
pub enum AppError {
    /// 公开错误 - 可以直接返回给客户端的错误信息
    #[error("public: `{0}`")]
    Public(String),

    /// 业务逻辑错误 - 保留 webserver-template 的业务特色
    #[error("business error: `{0}`")]
    Business(String),

    /// 验证错误 - 支持多种验证错误类型
    #[error("validation error: `{0}`")]
    Validation(String),

    /// 验证错误 - 来自 validator crate
    #[error("validation errors: `{0}`")]
    ValidationErrors(#[from] validator::ValidationErrors),

    /// 数据库错误 - 保留 SQLx 支持
    #[error("database error: `{0}`")]
    Database(#[from] sqlx::Error),

    /// 未找到资源错误
    #[error("not found: `{0}`")]
    NotFound(String),

    /// 认证错误
    #[error("authentication error: `{0}`")]
    Authentication(String),

    /// 授权错误
    #[error("authorization error: `{0}`")]
    Authorization(String),

    /// 内部错误 - 不应该暴露给客户端的错误
    #[error("internal error: `{0}`")]
    Internal(String),

    /// Salvo 框架错误
    #[error("salvo error: `{0}`")]
    Salvo(#[from] ::salvo::Error),

    /// HTTP 状态错误
    #[error("http status error: `{0}`")]
    HttpStatus(#[from] StatusError),

    /// HTTP 解析错误
    #[error("http parse error: `{0}`")]
    HttpParse(#[from] ParseError),

    /// Anyhow 错误
    #[error("anyhow error: `{0}`")]
    Anyhow(#[from] anyhow::Error),
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
    /// 创建公开错误 (基于 salvo-template)
    pub fn public<S: Into<String>>(msg: S) -> Self {
        Self::Public(msg.into())
    }

    /// 创建内部错误 (基于 salvo-template)
    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }

    /// 创建业务错误 (保留 webserver-template 特色)
    pub fn business<S: Into<String>>(msg: S) -> Self {
        Self::Business(msg.into())
    }

    /// 创建认证错误
    pub fn authentication<S: Into<String>>(msg: S) -> Self {
        Self::Authentication(msg.into())
    }

    /// 创建授权错误
    pub fn authorization<S: Into<String>>(msg: S) -> Self {
        Self::Authorization(msg.into())
    }

    /// 创建未找到错误
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        Self::NotFound(msg.into())
    }

    /// 创建验证错误
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        Self::Validation(msg.into())
    }

    /// 创建特定HTTP状态码的错误 (保持向后兼容)
    pub fn unauthorized(message: String) -> Self {
        AppError::Authentication(message)
    }

    pub fn forbidden(message: String) -> Self {
        AppError::Authorization(message)
    }

    pub fn too_many_requests(message: String) -> Self {
        AppError::Business(message)
    }

    pub fn method_not_allowed(message: String) -> Self {
        AppError::Business(message)
    }

    /// 转换为 API 错误响应 (保留 webserver-template 的响应格式)
    pub fn to_api_error(&self, request_id: Uuid) -> (StatusCode, ApiError) {
        let request_id = request_id.to_string();
        match self {
            AppError::Public(msg) => {
                error!("Public error: {}", msg);
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
            AppError::Business(msg) => {
                error!("Business error: {}", msg);
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
            AppError::ValidationErrors(errors) => {
                let msg = format!("Validation failed: {}", errors);
                error!("Validation errors: {}", msg);
                (
                    StatusCode::BAD_REQUEST,
                    ApiError {
                        code: 400,
                        message: msg,
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
            AppError::Authentication(msg) => {
                error!("Authentication error: {}", msg);
                (
                    StatusCode::UNAUTHORIZED,
                    ApiError {
                        code: 401,
                        message: msg.clone(),
                        request_id,
                        data: Value::Null,
                    },
                )
            }
            AppError::Authorization(msg) => {
                error!("Authorization error: {}", msg);
                (
                    StatusCode::FORBIDDEN,
                    ApiError {
                        code: 403,
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
            AppError::Salvo(err) => {
                error!("Salvo error: {}", err);
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
            AppError::HttpStatus(status_error) => {
                error!("HTTP status error: {}", status_error);
                (
                    status_error.code,
                    ApiError {
                        code: status_error.code.as_u16() as i32,
                        message: if status_error.brief.is_empty() {
                            "HTTP error".to_string()
                        } else {
                            status_error.brief.clone()
                        },
                        request_id,
                        data: Value::Null,
                    },
                )
            }
            AppError::HttpParse(err) => {
                error!("HTTP parse error: {}", err);
                (
                    StatusCode::BAD_REQUEST,
                    ApiError {
                        code: 400,
                        message: "Invalid request format".to_string(),
                        request_id,
                        data: Value::Null,
                    },
                )
            }
            AppError::Anyhow(err) => {
                error!("Anyhow error: {}", err);
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
}

/// Scribe trait 实现 (融合两个项目的优势)
///
/// 保留 webserver-template 的响应格式，集成 request_id 追踪
impl Scribe for AppError {
    fn render(self, res: &mut Response) {
        // 生成一个随机的request_id
        let request_id = Uuid::new_v4();

        let (status, api_error) = self.to_api_error(request_id);
        res.status_code(status);
        res.render(Json(api_error));
    }
}

/// OpenAPI 文档注册 (基于 salvo-template 的实现)
impl EndpointOutRegister for AppError {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            oapi::Response::new("Internal server error")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::NOT_FOUND.as_str(),
            oapi::Response::new("Not found")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::BAD_REQUEST.as_str(),
            oapi::Response::new("Bad request")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::UNAUTHORIZED.as_str(),
            oapi::Response::new("Unauthorized")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::FORBIDDEN.as_str(),
            oapi::Response::new("Forbidden")
                .add_content("application/json", StatusError::to_schema(components)),
        );
    }
}
