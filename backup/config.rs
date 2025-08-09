//! 配置管理模块
//!
//! 使用config-rs库加载分层配置，支持从文件和环境变量加载配置

use serde::Deserialize;

/// 应用配置结构体
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// 服务器配置
    pub server: ServerConfig,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// Redis配置
    pub redis: RedisConfig,
    /// JWT配置
    pub jwt: JwtConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// 主机地址
    pub host: String,
    /// 端口号
    pub port: u16,
}

/// 数据库配置
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库连接URL
    pub url: String,
}

/// Redis配置
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis连接URL
    pub url: String,
}

/// JWT配置
#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    /// JWT密钥
    pub secret: String,
    /// Token过期时间（小时）
    pub expiration_hours: i64,
}

impl Config {
    /// 加载配置
    ///
    /// 配置加载顺序：
    /// 1. config/default.toml 默认配置
    /// 2. config/${RUN_MODE}.toml 环境特定配置（可选）
    /// 3. 环境变量（最高优先级）
    pub fn load() -> Result<Self, config::ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        config.try_deserialize()
    }
}