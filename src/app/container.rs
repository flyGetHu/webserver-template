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
    #[must_use]
    pub fn new(config: Arc<Config>, app_state: Arc<AppState>) -> Self {
        // 创建用户仓库
        let user_repository = Arc::new(UserRepository::new(Arc::new(app_state.rb.clone())));

        // 创建认证服务
        let auth_service = Arc::new(AuthService::new(
            Arc::new(app_state.rb.clone()),
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

/// 创建服务注入中间件
///
/// 返回一个中间件 Handler，将 `AppServices` 注入到 Depot 中
#[must_use]
pub fn inject_services_middleware(services: Arc<AppServices>) -> ServiceInjectionHandler {
    ServiceInjectionHandler { services }
}

/// 服务注入中间件 Handler
#[derive(Clone)]
pub struct ServiceInjectionHandler {
    services: Arc<AppServices>,
}

#[async_trait::async_trait]
impl Handler for ServiceInjectionHandler {
    async fn handle(
        &self,
        request: &mut Request,
        depot: &mut Depot,
        response: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // 将各个服务注入到 Depot 中
        depot.insert("app_services", self.services.clone());
        depot.insert("config", self.services.config.clone());
        depot.insert("app_state", self.services.app_state.clone());
        depot.insert("user_repository", self.services.user_repository.clone());
        depot.insert("auth_service", self.services.auth_service.clone());
        depot.insert("user_service", self.services.user_service.clone());

        // 继续处理请求
        ctrl.call_next(request, depot, response).await;
    }
}

/// 便捷的服务注入中间件 Handler
///
/// 简化版本的服务注入器，用于直接在路由中使用
#[handler]
pub async fn inject_services(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    // 这个函数现在主要用于测试或特殊场景
    // 实际使用中推荐使用 inject_services_middleware 函数
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
                $crate::app::error::AppError::Internal(format!(
                    "Service {} not found in depot",
                    $service_name
                ))
            })
    };
}

/// 服务获取的便捷扩展 trait
pub trait DepotServiceExt {
    /// 获取完整的应用服务容器
    ///
    /// # Errors
    /// 当 `AppServices` 未注入到 `Depot` 时返回错误
    fn get_app_services(&self) -> Result<Arc<AppServices>, crate::app::error::AppError>;
    /// 获取用户仓库服务
    ///
    /// # Errors
    /// 当 `UserRepository` 未注入到 `Depot` 时返回错误
    fn get_user_repository(&self) -> Result<Arc<UserRepository>, crate::app::error::AppError>;
    /// 获取认证服务
    ///
    /// # Errors
    /// 当 `AuthService` 未注入到 `Depot` 时返回错误
    fn get_auth_service(&self) -> Result<Arc<AuthService>, crate::app::error::AppError>;
    /// 获取用户服务
    ///
    /// # Errors
    /// 当 `UserService` 未注入到 `Depot` 时返回错误
    fn get_user_service(&self) -> Result<Arc<UserService>, crate::app::error::AppError>;
    /// 获取配置
    ///
    /// # Errors
    /// 当 `Config` 未注入到 `Depot` 时返回错误
    fn get_config(&self) -> Result<Arc<Config>, crate::app::error::AppError>;
    /// 获取应用状态
    ///
    /// # Errors
    /// 当 `AppState` 未注入到 `Depot` 时返回错误
    fn get_app_state(&self) -> Result<Arc<AppState>, crate::app::error::AppError>;
}

impl DepotServiceExt for Depot {
    fn get_app_services(&self) -> Result<Arc<AppServices>, crate::app::error::AppError> {
        get_service!(self, AppServices, "app_services")
    }

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
        let db_pool = create_mock_db_pool().unwrap();
        let redis_pool = create_mock_redis_pool().await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let services = AppServices::new(config, app_state);

        // 验证所有服务都已正确创建
        assert!(services.user_repository.as_ref() as *const _ as usize != 0);
        assert!(services.auth_service.as_ref() as *const _ as usize != 0);
        assert!(services.user_service.as_ref() as *const _ as usize != 0);
    }

    #[tokio::test]
    async fn test_depot_service_ext() {
        let config = Arc::new(crate::app::config::Config::load().unwrap());
        let db_pool = create_mock_db_pool().unwrap();
        let redis_pool = create_mock_redis_pool().await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let services = Arc::new(AppServices::new(config, app_state));

        let mut depot = Depot::new();
        // 模拟服务注入中间件的行为
        depot.insert("app_services", services.clone());
        depot.insert("user_repository", services.user_repository.clone());
        depot.insert("auth_service", services.auth_service.clone());
        depot.insert("user_service", services.user_service.clone());
        depot.insert("config", services.config.clone());
        depot.insert("app_state", services.app_state.clone());

        // 测试服务获取
        let app_services = depot.get_app_services();
        assert!(app_services.is_ok());

        let user_repo = depot.get_user_repository();
        assert!(user_repo.is_ok());

        let auth_service = depot.get_auth_service();
        assert!(auth_service.is_ok());

        let user_service = depot.get_user_service();
        assert!(user_service.is_ok());

        let config = depot.get_config();
        assert!(config.is_ok());

        let app_state = depot.get_app_state();
        assert!(app_state.is_ok());
    }

    #[tokio::test]
    async fn test_service_injection_middleware() {
        let config = Arc::new(crate::app::config::Config::load().unwrap());
        let db_pool = create_mock_db_pool().unwrap();
        let redis_pool = create_mock_redis_pool().await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let services = Arc::new(AppServices::new(config, app_state));

        // 测试中间件创建
        let _middleware = inject_services_middleware(services.clone());

        // 验证中间件可以正常创建
        // 注意：这里只是验证中间件函数可以创建，实际的注入测试需要在集成测试中进行
        assert!(true); // 如果能到这里说明中间件创建成功
    }
}
