# 工程化服务注册表设计 - 解决结构体膨胀问题

## 问题分析

原`ServiceContainer`设计存在以下问题：
1. **结构体膨胀**：每增加一个服务就需要在结构体中添加一个字段
2. **维护困难**：需要手动更新构造函数和服务获取方法
3. **缺乏扩展性**：难以支持动态服务注册和懒加载

## 解决方案

### 1. 类型安全的注册表模式

使用`TypeId`和`HashMap`实现类型安全的动态服务注册：

```rust
pub struct ServiceRegistry {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    config: Arc<Config>,
    app_state: Arc<AppState>,
}
```

### 2. 优势对比

| 原设计 | 新设计 |
|--------|--------|
| 结构体字段膨胀 | 动态注册，无字段限制 |
| 手动维护构造函数 | 自动类型安全注册 |
| 编译时固定服务 | 支持运行时动态扩展 |
| 立即初始化所有服务 | 支持懒加载 |

## 使用方式

### 服务注册

```rust
// 注册服务
registry.register::<UserRepository>(user_repo);

// 获取服务
let user_repo = registry.get::<UserRepository>().unwrap();
```

### 便捷访问

通过`ServiceAccess`trait提供类型安全的便捷访问：

```rust
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
```

### 在处理器中使用

```rust
pub async fn create_user(
    State(service_registry): State<ServiceRegistry>,
    Extension(request_id): Extension<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
) -> Result<ApiResponse<CreateUserResponse>, AppError> {
    let auth_service = service_registry.auth_service();
    // ... 使用服务
}
```

## 扩展性设计

### 添加新服务

1. **创建服务结构体**：

```rust
pub struct EmailService {
    config: Arc<Config>,
    user_service: Arc<UserService>,
}

impl EmailService {
    pub fn new(config: Arc<Config>, user_service: Arc<UserService>) -> Self {
        Self { config, user_service }
    }
}
```

2. **注册服务**：

```rust
impl ServiceRegistry {
    pub fn register_default_services(&mut self) {
        // ... 现有注册
        
        let email_service = Arc::new(EmailService::new(
            self.config.clone(),
            self.user_service(),
        ));
        self.register::<EmailService>(email_service);
    }
}
```

3. **更新ServiceAccess trait**：

```rust
pub trait ServiceAccess {
    fn email_service(&self) -> Arc<EmailService>;
}

impl ServiceAccess for ServiceRegistry {
    fn email_service(&self) -> Arc<EmailService> {
        self.expect::<EmailService>()
    }
}
```

## 懒加载支持

提供了`LazyServiceRegistry`实现，支持按需初始化：

```rust
pub struct LazyServiceRegistry {
    services: HashMap<TypeId, Arc<OnceCell<Arc<dyn Any + Send + Sync>>>>,
    factories: HashMap<TypeId, Box<dyn ServiceFactory>>,
    // ...
}
```

## 构建器模式

支持流畅的API构建：

```rust
let registry = ServiceContainerBuilder::new()
    .config(config)
    .app_state(app_state)
    .register_service(custom_service)
    .build()
    .await?;
```

## 测试支持

便于测试的mock服务注入：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_service_registry() {
        let registry = ServiceRegistry::new(config, app_state).await;
        
        let user_repo = registry.get::<UserRepository>();
        assert!(user_repo.is_some());
    }
}
```

## 性能考虑

- **零成本抽象**：使用静态分发和编译时优化
- **线程安全**：所有服务都是`Send + Sync`
- **内存效率**：共享所有权，避免重复创建

## 总结

新的服务注册表设计解决了结构体膨胀问题，提供了：

1. **无限扩展性**：可以添加任意数量的服务
2. **类型安全**：编译时检查服务类型
3. **零成本抽象**：运行时性能无损耗
4. **易于测试**：支持mock服务注入
5. **懒加载支持**：按需初始化服务

这个设计既保持了Rust的类型安全特性，又提供了类似Java IoC容器的灵活性，但更加轻量和高效。