//! 领域服务模块声明

pub mod auth_service;
pub mod user_service;

pub use auth_service::AuthService;
pub use user_service::UserService;