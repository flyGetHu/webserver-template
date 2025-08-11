//! 用户相关的持久化模块
//!
//! 本模块组织用户数据的持久化操作：
//! - `repository.rs`: 处理所有用户相关的数据库操作（包括基础CRUD和复杂查询）
//! - `user_mapper.html`: rbatis XML 映射文件，定义复杂的 SQL 查询
//! - `xml_repository.rs`: 内部模块，不对外导出

pub mod repository;
mod xml_repository;

pub use repository::*;