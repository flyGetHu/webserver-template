//! 应用模块
//!
//! 使用模块化架构组织业务逻辑，每个模块包含：
//! - handlers: HTTP请求处理器
//! - models: 数据传输对象和领域模型
//! - routes: 路由定义
//! - services: 业务逻辑服务

pub mod auth;
pub mod health;
pub mod users;
