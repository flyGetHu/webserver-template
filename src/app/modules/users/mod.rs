//! 用户模块
//!
//! 处理用户管理相关的所有功能

pub mod handlers;
pub mod models;
pub mod routes;
pub mod services;

pub use routes::create_routes;
