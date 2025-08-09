//! 应用状态管理模块
//!
//! 定义应用的共享状态，包括数据库连接池等

use bb8_redis::{bb8, RedisConnectionManager};
use sqlx::MySqlPool;
use tracing::info;

/// Redis连接池类型别名
pub type RedisPool = bb8::Pool<RedisConnectionManager>;

/// 应用状态结构体
///
/// 包含所有需要在处理程序之间共享的状态
#[derive(Clone)]
pub struct AppState {
    /// MySQL数据库连接池
    pub db_pool: MySqlPool,
    /// Redis连接池
    pub redis_pool: RedisPool,
}

impl AppState {
    /// 创建新的应用状态实例
    pub fn new(db_pool: MySqlPool, redis_pool: RedisPool) -> Self {
        Self {
            db_pool,
            redis_pool,
        }
    }
}

/// 创建数据库连接池
///
/// 根据提供的数据库URL创建连接池
pub async fn create_db_pool(db_url: &str) -> Result<MySqlPool, sqlx::Error> {
    info!("Creating database pool for {}", db_url);

    let pool = MySqlPool::connect(db_url).await?;
    info!("Database pool created successfully");

    Ok(pool)
}

/// 创建Redis连接池
///
/// 根据提供的Redis URL创建连接池
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
pub async fn create_mock_db_pool() -> Result<MySqlPool, sqlx::Error> {
    info!("Creating mock database pool for testing");

    // 使用内存SQLite数据库作为模拟
    let pool = MySqlPool::connect("mysql://root:root123@192.168.100.149:3306/test").await?;
    info!("Mock database pool created successfully");

    Ok(pool)
}

/// 创建模拟Redis连接池（用于测试）
pub async fn create_mock_redis_pool() -> Result<RedisPool, bb8::RunError<redis::RedisError>> {
    info!("Creating mock Redis pool for testing");

    // 创建一个模拟的Redis管理器
    let manager = RedisConnectionManager::new("redis://192.168.100.149:6379/")
        .map_err(bb8::RunError::User)?;

    let pool = bb8::Pool::builder().build(manager).await?;
    info!("Mock Redis pool created successfully");

    Ok(pool)
}
