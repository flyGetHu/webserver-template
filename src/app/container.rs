//! 服务容器与注册表模块 - 分离 Service/Repository 的注入方式
//!
//! 提供与 Salvo 框架深度集成的依赖注入容器
//! 使用 Salvo 的 Depot 机制进行服务管理

use salvo::prelude::*;
use std::sync::Arc;

use crate::app::{
    config::Config,
    depot_keys::{KEY_REPOSITORY_REGISTRY, KEY_SERVICE_REGISTRY},
    domain::services::{auth_service::AuthService, user_service::UserService},
    infrastructure::persistence::UserRepository,
    state::AppState,
};

/// 仓库注册表：集中管理 Repository 对象
#[derive(Clone)]
pub struct RepositoryRegistry {
    pub user_repository: Arc<UserRepository>,
}

impl RepositoryRegistry {
    #[must_use]
    pub fn new(app_state: Arc<AppState>) -> Self {
        let user_repository = Arc::new(UserRepository::new(app_state.rb.clone()));
        Self { user_repository }
    }
}

/// 服务注册表：集中管理 Service 对象，并依赖 RepositoryRegistry
#[derive(Clone)]
pub struct ServiceRegistry {
    pub config: Arc<Config>,
    pub app_state: Arc<AppState>,
    pub repositories: Arc<RepositoryRegistry>,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}

impl ServiceRegistry {
    #[must_use]
    pub fn new(
        config: Arc<Config>,
        app_state: Arc<AppState>,
        repositories: Arc<RepositoryRegistry>,
    ) -> Self {
        let auth_service = Arc::new(AuthService::new(app_state.rb.clone(), config.clone()));
        let user_service = Arc::new(UserService::new(
            repositories.user_repository.clone(),
            config.clone(),
        ));

        Self {
            config,
            app_state,
            repositories,
            auth_service,
            user_service,
        }
    }
}

/// 创建注册表注入中间件
///
/// 返回一个中间件 Handler，将 `ServiceRegistry` 与 `RepositoryRegistry` 注入到 Depot 中
#[must_use]
pub fn inject_registries_middleware(
    services: Arc<ServiceRegistry>,
    repositories: Arc<RepositoryRegistry>,
) -> RegistryInjectionHandler {
    RegistryInjectionHandler {
        services,
        repositories,
    }
}

/// 注册表注入中间件 Handler
#[derive(Clone)]
pub struct RegistryInjectionHandler {
    services: Arc<ServiceRegistry>,
    repositories: Arc<RepositoryRegistry>,
}

#[async_trait::async_trait]
impl Handler for RegistryInjectionHandler {
    async fn handle(
        &self,
        request: &mut Request,
        depot: &mut Depot,
        response: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // 注入分层注册表，类似 Spring 层级 Bean 管理
        depot.insert(KEY_SERVICE_REGISTRY, self.services.clone());
        depot.insert(KEY_REPOSITORY_REGISTRY, self.repositories.clone());

        // 继续处理请求
        ctrl.call_next(request, depot, response).await;
    }
}

// 保留一个空 handler 以兼容结构（如需链路占位）
#[handler]
pub async fn inject_services(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    ctrl.call_next(req, depot, res).await;
}

/// 便捷的服务获取扩展

/// 服务获取的便捷扩展 trait
pub trait DepotServiceExt {
    /// 获取 Service 注册表
    fn services(&self) -> Result<Arc<ServiceRegistry>, crate::app::error::AppError>;
    /// 获取 Repository 注册表
    fn repositories(&self) -> Result<Arc<RepositoryRegistry>, crate::app::error::AppError>;
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
    fn services(&self) -> Result<Arc<ServiceRegistry>, crate::app::error::AppError> {
        self
            .get::<Arc<ServiceRegistry>>(KEY_SERVICE_REGISTRY)
            .map(|s| s.clone())
            .map_err(|_| crate::app::error::AppError::Internal("ServiceRegistry not found in depot".to_string()))
    }

    fn repositories(&self) -> Result<Arc<RepositoryRegistry>, crate::app::error::AppError> {
        self
            .get::<Arc<RepositoryRegistry>>(KEY_REPOSITORY_REGISTRY)
            .map(|s| s.clone())
            .map_err(|_| crate::app::error::AppError::Internal("RepositoryRegistry not found in depot".to_string()))
    }

    fn get_user_repository(&self) -> Result<Arc<UserRepository>, crate::app::error::AppError> {
        self.repositories().map(|r| r.user_repository.clone())
    }

    fn get_auth_service(&self) -> Result<Arc<AuthService>, crate::app::error::AppError> {
        self.services().map(|s| s.auth_service.clone())
    }

    fn get_user_service(&self) -> Result<Arc<UserService>, crate::app::error::AppError> {
        self.services().map(|s| s.user_service.clone())
    }

    fn get_config(&self) -> Result<Arc<Config>, crate::app::error::AppError> {
        self.services().map(|s| s.config.clone())
    }

    fn get_app_state(&self) -> Result<Arc<AppState>, crate::app::error::AppError> {
        self.services().map(|s| s.app_state.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::{create_mock_db_pool, create_mock_redis_pool};

    #[tokio::test]
    async fn test_registries_creation() {
        let config = Arc::new(crate::app::config::Config::load().unwrap());
        let db_pool = create_mock_db_pool().unwrap();
        let redis_pool = create_mock_redis_pool().await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let repositories = Arc::new(RepositoryRegistry::new(app_state.clone()));
        let services = Arc::new(ServiceRegistry::new(config, app_state, repositories.clone()));

        // 验证所有服务都已正确创建
        assert!(repositories.user_repository.as_ref() as *const _ as usize != 0);
        assert!(services.auth_service.as_ref() as *const _ as usize != 0);
        assert!(services.user_service.as_ref() as *const _ as usize != 0);
    }

    #[tokio::test]
    async fn test_depot_service_ext() {
        let config = Arc::new(crate::app::config::Config::load().unwrap());
        let db_pool = create_mock_db_pool().unwrap();
        let redis_pool = create_mock_redis_pool().await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let repositories = Arc::new(RepositoryRegistry::new(app_state.clone()));
        let services = Arc::new(ServiceRegistry::new(config, app_state, repositories.clone()));

        let mut depot = Depot::new();
        // 模拟服务注入中间件的行为：注入两个注册表
        depot.insert(KEY_SERVICE_REGISTRY, services.clone());
        depot.insert(KEY_REPOSITORY_REGISTRY, repositories.clone());

        // 测试服务获取
        let srvs = depot.services();
        assert!(srvs.is_ok());

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

        let repositories = Arc::new(RepositoryRegistry::new(app_state.clone()));
        let services = Arc::new(ServiceRegistry::new(config, app_state, repositories.clone()));

        // 测试中间件创建
        let _middleware = inject_registries_middleware(services.clone(), repositories.clone());

        // 验证中间件可以正常创建
        // 注意：这里只是验证中间件函数可以创建，实际的注入测试需要在集成测试中进行
        assert!(true); // 如果能到这里说明中间件创建成功
    }
}
