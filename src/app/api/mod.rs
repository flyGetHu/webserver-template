//! API层模块声明
//!
//! API层负责处理HTTP请求和响应，是应用的入口点

pub mod docs;
pub mod extractors;
pub mod handlers;
pub mod middleware;
pub mod response;
pub mod routes;
pub mod telemetry;
