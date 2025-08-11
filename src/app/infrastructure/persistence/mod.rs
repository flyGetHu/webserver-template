//! 持久化模块声明
//!
//! 按表分文件夹组织：
//! - users/: 用户表相关的所有持久化操作
//! - 未来可以添加其他表的文件夹，如 products/, orders/ 等

pub mod users;

// 重新导出常用类型
pub use users::UserRepository;

