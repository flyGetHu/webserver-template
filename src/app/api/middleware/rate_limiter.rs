//! 限流中间件模块
//!
//! 基于salvo-rate-limiter实现IP级别的请求限流

use salvo::prelude::*;
use salvo_rate_limiter::{BasicQuota, FixedGuard, MokaStore, RateLimiter, RemoteIpIssuer};

/// 创建IP限流中间件
///
/// 配置:
/// - 桶容量: 100个请求
/// - 填充速率: 每秒10个请求
pub fn rate_limiter() -> impl Handler {
    RateLimiter::new(
        FixedGuard::new(),
        MokaStore::new(),
        RemoteIpIssuer,
        BasicQuota::per_second(10), // 每秒10个请求
    )
}
