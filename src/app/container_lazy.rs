//! 懒加载服务容器模块
//!
//! 提供按需初始化的服务容器，避免不必要的资源占用

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::app::{
    config::Config,
    domain::services::{AuthService, UserService},
    infrastructure::persistence::UserRepository,
    state::AppState,
};

/// 懒加载服务工厂
pub trait ServiceFactory: Send + Sync {
    fn create(&self, registry: &LazyServiceRegistry) -> Arc<dyn Any + Send + Sync>;
}

/// 懒加载服务注册表
pub struct LazyServiceRegistry {
    services: HashMap<TypeId, Arc<OnceCell<Arc<dyn Any + Send + Sync>>>>,
    factories: HashMap<TypeId, Box<dyn ServiceFactory>>,
    config: Arc<Config>,
    app_state: Arc<AppState>,
}

impl LazyServiceRegistry {
    pub fn new(config: Arc<Config>, app_state: Arc<AppState>) -> Self {
        let mut registry = Self {
            services: HashMap::new(),
            factories: HashMap::new(),
            config,
            app_state,
        };

        registry.register_default_factories();
        registry
    }

    fn register_default_factories(&mut self) {
        // 注册用户仓库工厂
        self.register_factory::<UserRepository>(Box::new(UserRepositoryFactory));
        
        // 注册认证服务工厂
        self.register_factory::<AuthService>(Box::new(AuthServiceFactory));
        
        // 注册用户服务工厂
        self.register_factory::<UserService>(Box::new(UserServiceFactory));
    }

    pub fn register_factory<T: 'static + Send + Sync>(
        &mut self,
        factory: Box<dyn ServiceFactory<Output = Arc<T>>>,
    ) {
        self.factories.insert(TypeId::of::<T>(), factory);
        
        // 预先创建 OnceCell
        self.services.insert(
            TypeId::of::<T>(),
            Arc::new(OnceCell::new()),
        );
    }

    pub async fn get<T: 'static + Send + Sync>(
        &self,
    ) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        
        let once_cell = self.services.get(&type_id)?;
        
        let service = once_cell
            .get_or_init(|| {
                let factory = self.factories.get(&type_id)?;
                Some(factory.create(self))
            })
            .await;
            
        service.clone().downcast::<T>().ok()
    }

    pub async fn expect<T: 'static + Send + Sync>(&self,
    ) -> Arc<T> {
        self.get::<T>()
            .await
            .expect(&format!("Service {} not found", std::any::type_name::<T>()))
    }

    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    pub fn app_state(&self) -> Arc<AppState> {
        self.app_state.clone()
    }
}

/// 服务扩展特性
pub trait LazyServiceAccess {
    async fn user_repository(&self) -> Arc<UserRepository>;
    async fn auth_service(&self) -> Arc<AuthService>;
    async fn user_service(&self) -> Arc<UserService>;
}

impl LazyServiceAccess for LazyServiceRegistry {
    async fn user_repository(&self) -> Arc<UserRepository> {
        self.expect::<UserRepository>().await
    }

    async fn auth_service(&self) -> Arc<AuthService> {
        self.expect::<AuthService>().await
    }

    async fn user_service(&self) -> Arc<UserService> {
        self.expect::<UserService>().await
    }
}

// 工厂实现
struct UserRepositoryFactory;
impl ServiceFactory for UserRepositoryFactory {
    fn create(&self, registry: &LazyServiceRegistry) -> Arc<dyn Any + Send + Sync> {
        Arc::new(UserRepository::new(Arc::new(registry.app_state.db_pool.clone()))) as Arc<_>
    }
}

struct AuthServiceFactory;
impl ServiceFactory for AuthServiceFactory {
    fn create(&self, registry: &LazyServiceRegistry) -> Arc<dyn Any + Send + Sync> {
        Arc::new(AuthService::new(
            Arc::new(registry.app_state.db_pool.clone()),
            registry.config.clone(),
        )) as Arc<_>
    }
}

struct UserServiceFactory;
impl ServiceFactory for UserServiceFactory {
    fn create(&self, registry: &LazyServiceRegistry) -> Arc<dyn Any + Send + Sync> {
        let user_repo = registry.get::<UserRepository>().await.unwrap();
        Arc::new(UserService::new(user_repo, registry.config.clone())) as Arc<_>
    }
}

// 修复：ServiceFactory trait 应该返回具体的 Arc<T>
trait ServiceFactory {
    fn create(&self, registry: &LazyServiceRegistry) -> Arc<dyn Any + Send + Sync>;
}