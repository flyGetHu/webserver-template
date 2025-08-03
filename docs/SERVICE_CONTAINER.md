# 服务容器模式 - 统一管理实例初始化

本项目实现了类似于Java依赖注入容器的统一实例管理系统，但更符合Rust的所有权模型。

## 架构概览

```
┌─────────────────────────────────────────┐
│           ServiceContainer              │
│  (统一服务容器，集中管理所有实例)        │
├─────────────────────────────────────────┤
│  - UserRepository                        │
│  - AuthService                          │
│  - UserService                          │
│  - Config                               │
│  - AppState                             │
└─────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────┐
│            路由处理器                    │
│  - create_user                          │
│  - get_user_by_id                       │
│  - ...                                  │
└─────────────────────────────────────────┘
```

## 使用方式

### 1. 服务容器初始化

在 `src/app/mod.rs` 中自动完成：

```rust
// 创建服务容器
let service_container = ServiceContainer::new(Arc::new(config), app_state.clone()).await;
```

### 2. 在处理器中使用服务

```rust
pub async fn create_user(
    State(service_container): State<ServiceContainer>,
    Extension(request_id): Extension<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
) -> Result<ApiResponse<CreateUserResponse>, AppError> {
    // 直接从容器中获取服务实例
    let auth_service = service_container.auth_service();
    let user_service = service_container.user_service();
    
    // 使用服务
    let user = auth_service.register_user(create_user_dto).await?;
    Ok(ApiResponse::new(response, request_id))
}
```

### 3. 添加新服务

#### 步骤1：创建服务结构体
```rust
pub struct NewService {
    user_repository: Arc<UserRepository>,
    config: Arc<Config>,
}

impl NewService {
    pub fn new(user_repository: Arc<UserRepository>, config: Arc<Config>) -> Self {
        Self {
            user_repository,
            config,
        }
    }
}
```

#### 步骤2：更新服务容器
在 `src/app/container.rs` 中：

```rust
pub struct ServiceContainer {
    // ... 现有字段
    new_service: Arc<NewService>,
}

impl ServiceContainer {
    pub async fn new(config: Arc<Config>, app_state: Arc<AppState>) -> Self {
        // ... 现有初始化
        let new_service = Arc::new(NewService::new(
            user_repository.clone(),
            config.clone(),
        ));
        
        Self {
            // ... 现有字段
            new_service,
        }
    }
    
    pub fn new_service(&self) -> Arc<NewService> {
        self.new_service.clone()
    }
}
```

## 架构优势

### 1. 集中管理
- 所有服务实例在一个地方创建和管理
- 避免重复创建实例
- 统一处理依赖关系

### 2. 线程安全
- 使用 `Arc` 实现线程安全的共享所有权
- 符合Rust的所有权系统

### 3. 依赖注入
- 构造函数注入，类型安全
- 编译时检查，无运行时错误

### 4. 生命周期管理
- 实例在应用启动时创建
- 在应用关闭时自动释放

### 5. 易于测试
- 可以轻松创建测试用的服务容器
- 支持mock对象的注入

## 与Java DI容器的区别

| Java/Spring | Rust/ServiceContainer |
|-------------|----------------------|
| `@Singleton` | `Arc<T>` |
| `@Autowired` | 构造函数参数 |
| `@Component` | 结构体实现 |
| 运行时反射 | 编译时检查 |
| 代理模式 | 直接调用 |
| 循环依赖检测 | 编译时错误 |

## 文件结构

```
src/
├── app/
│   ├── container.rs          # 服务容器实现
│   ├── domain/
│   │   └── services/         # 业务服务
│   │       ├── auth_service.rs
│   │       └── user_service.rs
│   └── infrastructure/
│       └── persistence/      # 数据访问层
│           └── user_repository.rs
```

## 最佳实践

1. **服务粒度**：保持服务单一职责，避免过大
2. **依赖方向**：上层服务依赖下层，避免循环依赖
3. **错误处理**：在服务层统一处理业务错误
4. **配置管理**：通过容器传递配置，避免全局变量

## 示例代码

### 获取服务实例
```rust
// 在处理器中
let user_service = service_container.user_service();
let auth_service = service_container.auth_service();
```

### 创建复杂服务
```rust
// 服务可以依赖其他服务
impl OrderService {
    pub fn new(
        order_repository: Arc<OrderRepository>,
        user_service: Arc<UserService>,
        payment_service: Arc<PaymentService>,
    ) -> Self {
        Self {
            order_repository,
            user_service,
            payment_service,
        }
    }
}
```

这个系统提供了Rust生态中类似Java Spring的依赖管理功能，但更加安全、高效，且符合Rust的所有权模型。