//! 应用状态管理模块
//!
//! 定义应用的共享状态，包括数据库连接池等

use bb8_redis::{bb8, RedisConnectionManager};
use rbatis::RBatis;
use std::sync::Arc;
use tracing::info;

/// Redis连接池类型别名
pub type RedisPool = bb8::Pool<RedisConnectionManager>;

/// 应用状态结构体
///
/// 包含所有需要在处理程序之间共享的状态
#[derive(Clone)]
pub struct AppState {
    /// `RBatis` 数据库实例（共享引用）
    pub rb: Arc<RBatis>,
    /// Redis连接池（共享引用）
    pub redis_pool: Arc<RedisPool>,
}

impl AppState {
    /// 创建新的应用状态实例
    #[must_use]
    pub fn new(rb: RBatis, redis_pool: RedisPool) -> Self {
        Self {
            rb: Arc::new(rb),
            redis_pool: Arc::new(redis_pool),
        }
    }
}

/// 创建数据库连接池
///
/// 根据提供的数据库URL创建连接池
/// # Errors
/// 初始化数据库失败
pub fn create_db_pool(db_url: &str) -> Result<RBatis, rbatis::Error> {
    info!("Creating database pool for {}", db_url);
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, db_url)?;
    info!("Database pool created successfully");
    Ok(rb)
}

/// 创建Redis连接池
///
/// 根据提供的Redis URL创建连接池
/// # Errors
/// 创建连接池失败
pub async fn create_redis_pool(
    redis_url: &str,
) -> Result<RedisPool, bb8::RunError<redis::RedisError>> {
    info!("Creating Redis pool for {}", redis_url);

    let manager = RedisConnectionManager::new(redis_url).map_err(|e| {
        tracing::error!("Failed to create Redis connection manager: {}", e);
        bb8::RunError::User(e)
    })?;

    let pool = bb8::Pool::builder().build(manager).await?;
    info!("Redis pool created successfully");

    Ok(pool)
}

/// 创建模拟数据库连接池（用于测试）
/// # Errors
/// 初始化数据库失败
pub fn create_mock_db_pool() -> Result<RBatis, rbatis::Error> {
    info!("Creating mock database pool for testing");
    let rb = RBatis::new();
    // 使用与配置相同的 MySQL 测试库，或内存 sqlite，如果有需要可改为 rbdc-sqlite
    rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:root123@192.168.100.149:3306/test")?;
    info!("Mock database pool created successfully");
    Ok(rb)
}

/// 创建模拟Redis连接池（用于测试）
/// # Errors
/// 创建连接池失败
pub async fn create_mock_redis_pool() -> Result<RedisPool, bb8::RunError<redis::RedisError>> {
    info!("Creating mock Redis pool for testing");

    // 创建一个模拟的Redis管理器
    let manager = RedisConnectionManager::new("redis://192.168.100.149:6379/")
        .map_err(bb8::RunError::User)?;

    let pool = bb8::Pool::builder().build(manager).await?;
    info!("Mock Redis pool created successfully");

    Ok(pool)
}
