//! 服务容器模块 - 工程化版本
//!
//! 提供统一的依赖管理和实例生命周期管理
//! 使用类型安全的注册表模式，避免结构体膨胀

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use crate::app::{
    config::Config,
    domain::services::{auth_service::AuthService, user_service::UserService},
    infrastructure::persistence::user_repository::UserRepository,
    state::AppState,
};

/// 服务注册表
/// 
/// 使用类型安全的注册表模式，支持动态服务注册
#[derive(Clone)]
pub struct ServiceRegistry {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    config: Arc<Config>,
    app_state: Arc<AppState>,
}

impl ServiceRegistry {
    /// 创建新的服务注册表
    pub async fn new(config: Arc<Config>, app_state: Arc<AppState>) -> Self {
        let mut registry = Self {
            services: HashMap::new(),
            config,
            app_state,
        };

        // 注册核心服务
        registry.register_default_services().await;
        registry
    }

    /// 注册默认服务
    async fn register_default_services(&mut self) {
        // 注册用户仓库
        let user_repo = Arc::new(UserRepository::new(Arc::new(self.app_state.db_pool.clone())));
        self.register::<UserRepository>(user_repo);

        // 注册认证服务
        let auth_service = Arc::new(AuthService::new(
            Arc::new(self.app_state.db_pool.clone()),
            self.config.clone(),
        ));
        self.register::<AuthService>(auth_service);

        // 注册用户服务
        let user_service = Arc::new(UserService::new(
            self.get::<UserRepository>().unwrap(),
            self.config.clone(),
        ));
        self.register::<UserService>(user_service);
    }

    /// 注册服务实例
    pub fn register<T: 'static + Send + Sync>(&mut self, service: Arc<T>) {
        self.services.insert(TypeId::of::<T>(), service);
    }

    /// 获取服务实例
    pub fn get<T: 'static + Send + Sync>(&self) -> Option<Arc<T>> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|service| service.clone().downcast::<T>().ok())
    }

    /// 获取服务实例（带错误处理）
    pub fn expect<T: 'static + Send + Sync>(&self) -> Arc<T> {
        self.get::<T>().expect(&format!("Service {} not found", std::any::type_name::<T>()))
    }

    /// 获取配置
    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    /// 获取应用状态
    pub fn app_state(&self) -> Arc<AppState> {
        self.app_state.clone()
    }
}

/// 服务扩展特性
/// 
/// 为服务注册表提供便捷的获取方法
pub trait ServiceAccess {
    fn user_repository(&self) -> Arc<UserRepository>;
    fn auth_service(&self) -> Arc<AuthService>;
    fn user_service(&self) -> Arc<UserService>;
}

impl ServiceAccess for ServiceRegistry {
    fn user_repository(&self) -> Arc<UserRepository> {
        self.expect::<UserRepository>()
    }

    fn auth_service(&self) -> Arc<AuthService> {
        self.expect::<AuthService>()
    }

    fn user_service(&self) -> Arc<UserService> {
        self.expect::<UserService>()
    }
}

/// 服务容器构建器
pub struct ServiceContainerBuilder {
    config: Option<Arc<Config>>,
    app_state: Option<Arc<AppState>>,
    custom_services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceContainerBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            app_state: None,
            custom_services: HashMap::new(),
        }
    }

    pub fn config(mut self, config: Arc<Config>) -> Self {
        self.config = Some(config);
        self
    }

    pub fn app_state(mut self, app_state: Arc<AppState>) -> Self {
        self.app_state = Some(app_state);
        self
    }

    pub fn register_service<T: 'static + Send + Sync>(mut self, service: Arc<T>) -> Self {
        self.custom_services.insert(TypeId::of::<T>(), service);
        self
    }

    pub async fn build(self) -> Result<ServiceRegistry, String> {
        let config = self.config.ok_or("Config is required")?;
        let app_state = self.app_state.ok_or("AppState is required")?;

        let mut registry = ServiceRegistry::new(config, app_state).await;

        // 注册自定义服务
        for (type_id, service) in self.custom_services {
            registry.services.insert(type_id, service);
        }

        Ok(registry)
    }
}

impl Default for ServiceContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::AppState;

    #[tokio::test]
    async fn test_service_registry() {
        let config = Arc::new(crate::app::config::Config::load().unwrap());
        let db_pool = state::create_db_pool("mysql://localhost/test").await.unwrap();
        let redis_pool = state::create_redis_pool("redis://localhost/").await.unwrap();
        let app_state = Arc::new(AppState::new(db_pool, redis_pool));

        let registry = ServiceRegistry::new(config, app_state).await;

        let user_repo = registry.get::<UserRepository>();
        assert!(user_repo.is_some());

        let auth_service = registry.auth_service();
        assert!(auth_service.user_repository().is_some());
    }
}