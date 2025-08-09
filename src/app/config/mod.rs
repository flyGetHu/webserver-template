use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

pub mod log_config;
pub use log_config::LogConfig;

pub mod db_config;
pub use db_config::{DatabaseConfig, RedisConfig};

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub log: LogConfig,
    pub tls: Option<TlsConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(default = "default_jwt_expiry")]
    pub expiry: i64, // 单位：秒
}

#[derive(Deserialize, Clone, Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

impl Config {
    pub fn load() -> Result<Self, figment::Error> {
        // 获取运行模式
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        // 使用 Figment 进行分层配置加载
        let mut config = Figment::new()
            // 1. 默认配置文件
            .merge(Toml::file("config/default.toml"))
            // 2. 环境特定配置文件 (可选)
            .merge(Toml::file(format!("config/{run_mode}.toml")).nested())
            // 3. 环境变量覆盖 (最高优先级)
            .merge(Env::prefixed("APP_").global());

        // 4. 特殊环境变量处理
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            config = config.merge(("database.url", database_url));
        }
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            config = config.merge(("redis.url", redis_url));
        }

        config.extract()
    }
}

// Default functions
fn default_listen_addr() -> String {
    "127.0.0.1:3000".into()
}

fn default_host() -> String {
    "127.0.0.1".into()
}

fn default_port() -> u16 {
    3000
}

fn default_jwt_expiry() -> i64 {
    86400 // 24小时，单位：秒
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: default_listen_addr(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            jwt: JwtConfig {
                secret: "your-secret-key".to_string(),
                expiry: default_jwt_expiry(),
            },
            log: LogConfig::default(),
            tls: None,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}
