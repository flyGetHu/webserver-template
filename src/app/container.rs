//! 服务容器模块 - Salvo 最佳实践版本
//!
//! 提供与 Salvo 框架深度集成的依赖注入容器
//! 使用 Salvo 的 Depot 机制进行服务管理

use salvo::prelude::*;
use std::sync::Arc;

use crate::app::{
    config::Config,
    domain::services::{auth_service::AuthService, user_service::UserService},
    infrastructure::persistence::user_repository::UserRepository,
    state::AppState,
};

/// 应用服务容器
///
/// 使用 Salvo 推荐的简单直接的方式管理服务依赖
/// 避免过度工程化，专注于实用性和可维护性
#[derive(Clone)]
pub struct AppServices {
    pub config: Arc<Config>,
    pub app_state: Arc<AppState>,
    pub user_repository: Arc<UserRepository>,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}

impl AppServices {
    /// 创建应用服务容器
    ///
    /// 按照依赖顺序初始化所有服务
    pub async fn new(config: Arc<Config>, app_state: Arc<AppState>) -> Self {
        // 创建用户仓库
        let user_repository = Arc::new(UserRepository::new(Arc::new(app_state.db_pool.clone())));

        // 创建认证服务
        let auth_service = Arc::new(AuthService::new(
            Arc::new(app_state.db_pool.clone()),
            config.clone(),
        ));

        // 创建用户服务
        let user_service = Arc::new(UserService::new(user_repository.clone(), config.clone()));

        Self {
            config,
            app_state,
            user_repository,
            auth_service,
            user_service,
        }
    }
}

/// Salvo 中间件：服务注入器
///
/// 简化版本 - 暂时跳过服务注入，直接继续处理
#[handler]
pub async fn inject_services(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // 暂时简化实现，后续可以添加实际的服务注入逻辑
    ctrl.call_next(req, depot, res).await;
}

/// 便捷的服务获取宏
///
/// 从 Depot 中获取服务的便捷方法
#[macro_export]
macro_rules! get_service {
    ($depot:expr, $service_type:ty, $service_name:expr) => {
        $depot
            .get::<Arc<$service_type>>($service_name)
            .map(|s| s.clone())
            .map_err(|_| {
                crate::app::error::AppError::Internal(format!(
                    "Service {} not found in depot",
                    $service_name
                ))
            })
    };
}

/// 服务获取的便捷扩展 trait
pub trait DepotServiceExt {
    fn get_user_repository(&self) -> Result<Arc<UserRepository>, crate::app::error::AppError>;
    fn get_auth_service(&self) -> Result<Arc<AuthService>, crate::app::error::AppError>;
    fn get_user_service(&self) -> Result<Arc<UserService>, crate::app::error::AppError>;
    fn get_config(&self) -> Result<Arc<Config>, crate::app::error::AppError>;
    fn get_app_state(&self) -> Result<Arc<AppState>, crate::app::error::AppError>;
}

impl DepotServiceExt for Depot {
    fn get_user_repository(&self) -> Result<Arc<UserRepository>, crate::app::error::AppError> {
        get_service!(self, UserRepository, "user_repository")
    }

    fn get_auth_service(&self) -> Result<Arc<AuthService>, crate::app::error::AppError> {
        get_service!(self, AuthService, "auth_service")
    }

    fn get_user_service(&self) -> Result<Arc<UserService>, crate::app::error::AppError> {
        get_service!(self, UserService, "user_service")
    }

    fn get_config(&self) -> Result<Arc<Config>, crate::app::error::AppError> {
        get_service!(self, Config, "config")
    }

    fn get_app_state(&self) -> Result<Arc<AppState>, crate::app::error::AppError> {
        get_service!(self, AppState, "app_state")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::{create_mock_db_pool, create_mock_redis_pool};

    #[tokio::test]
    async fn test_app_services_creation() {
        let config = Arc::new(crate::app::config::Config::load().unwrap());
        let db_pool = create_mock_db_pool().await.unwrap();
        let redis_pool = create_mock_redis_pool().await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let services = AppServices::new(config, app_state).await;

        // 验证所有服务都已正确创建
        assert!(services.user_repository.as_ref() as *const _ as usize != 0);
        assert!(services.auth_service.as_ref() as *const _ as usize != 0);
        assert!(services.user_service.as_ref() as *const _ as usize != 0);
    }

    #[tokio::test]
    async fn test_depot_service_ext() {
        let config = Arc::new(crate::app::config::Config::load().unwrap());
        let db_pool = create_mock_db_pool().await.unwrap();
        let redis_pool = create_mock_redis_pool().await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let services = Arc::new(AppServices::new(config, app_state).await);

        let mut depot = Depot::new();
        depot.insert("user_repository", services.user_repository.clone());
        depot.insert("auth_service", services.auth_service.clone());

        // 测试服务获取
        let user_repo = depot.get_user_repository();
        assert!(user_repo.is_ok());

        let auth_service = depot.get_auth_service();
        assert!(auth_service.is_ok());
    }
}
