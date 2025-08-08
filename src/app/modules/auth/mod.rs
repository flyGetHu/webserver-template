//! 认证模块
//!
//! 处理用户认证相关的所有功能，包括注册、登录、注销等

pub mod handlers;
pub mod models;
pub mod routes;
pub mod services;

pub use routes::create_routes;
