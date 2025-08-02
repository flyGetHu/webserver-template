//! 应用模块声明

pub mod config;
pub mod error;
pub mod state;
pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod lib;

pub use config::Config;
pub use state::AppState;

/// 运行应用的主要函数
///
/// 该函数封装了应用启动的完整逻辑
pub async fn run() -> anyhow::Result<()> {
    lib::run().await
}
