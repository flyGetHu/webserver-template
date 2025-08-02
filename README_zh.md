# Rust Web 服务器模板

这是一个基于 `axum` 的高性能、模块化的 Rust Web 服务器模板项目。它旨在为从 Java/Kotlin 等背景迁移过来的开发者提供一个现代化、企业级且易于使用的起点。

## 目录

- [Rust Web 服务器模板](#rust-web-服务器模板)
  - [目录](#目录)
  - [1. 快速开始](#1-快速开始)
  - [2. 设计理念](#2-设计理念)
  - [3. 技术栈](#3-技术栈)
  - [4. 企业级项目结构](#4-企业级项目结构)
    - [模块职责](#模块职责)
    - [配置管理](#配置管理)
  - [5. 核心功能与设计模式](#5-核心功能与设计模式)
    - [5.1. 中间件设计](#51-中间件设计)
    - [5.2. 全局异常处理与统一响应](#52-全局异常处理与统一响应)
    - [5.3. 日志中间件](#53-日志中间件)
    - [5.4. 灵活的身份验证与授权](#54-灵活的身份验证与授权)
    - [5.5. 统一请求验证](#55-统一请求验证)
  - [6. 开发指南](#6-开发指南)
    - [如何添加新的 API 端点](#如何添加新的-api-端点)
  - [7. 集成 Redis](#7-集成-redis)

---

## 1. 快速开始

1.  **安装 Rust**:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **构建项目**:
    ```bash
    cargo build
    ```

3.  **运行项目**:
    ```bash
    cargo run
    ```
    服务将启动在 `.env` 文件中配置的 `SERVER_ADDR`（默认为 `127.0.0.1:3000`）。

4.  **测试 API**:
    -   **健康检查**:
        ```bash
        curl http://127.0.0.1:3000/health
        ```
    -   **示例：创建用户**:
        ```bash
        curl -X POST http://127.0.0.1:3000/api/v1/users \
             -H "Content-Type: application/json" \
             -d '{"username": "gemini", "email": "gemini@google.com", "age": 25}'
        ```
---

## 2. 设计理念

-   **现代化**: 全面拥抱异步 Rust，基于 `tokio` 生态系统构建。
-   **模块化与可扩展**: 采用分层架构（API、Domain、Infrastructure）确保高内聚低耦合，使项目易于维护和扩展。
-   **高性能**: 利用 `hyper` 和 `tokio` 提供顶级的网络性能。
-   **企业级**: 集成企业应用所需的基础架构，包括日志记录、配置、错误处理和强大的身份验证/授权机制。
-   **开发者友好**: 注重开发者体验，提供清晰可重用的模式（例如用于验证的自定义提取器）以减少样板代码并提高生产力。

---

## 3. 技术栈

| 组件                | Crate          | 用途                     | 理由                                                                                               |
| :------------------ | :------------- | :----------------------- | :------------------------------------------------------------------------------------------------- |
| **Web 框架**        | `axum`         | HTTP 路由与处理          | 由 Tokio 团队开发，与生态系统无缝集成，提供优雅且模块化的设计。                                    |
| **异步运行时**      | `tokio`        | 驱动所有异步操作         | 异步 Rust 的事实标准，以稳定性和高性能著称。                                                       |
| **ORM 框架**        | `sqlx`         | 异步 SQL 工具包          | 纯异步且具有编译时 SQL 验证。高性能且类型安全。现代 Rust 应用的首选。                              |
| **序列化**          | `serde`        | 数据格式处理（JSON）     | Rust 社区中的序列化标准，功能强大且性能优异。                                                     |
| **结构化日志**      | `tracing`      | 应用与请求日志           | 专为异步应用设计，提供结构化、基于级别的日志记录，对调试和监控至关重要。                           |
| **HTTP 中间件**     | `tower-http`   | 通用中间件               | 提供 essentials 如 `TraceLayer`（日志记录）和 `CorsLayer`，与 `axum` 完美配合。                    |
| **配置**            | `config-rs`    | 分层配置                 | 合并来自文件（如 TOML）和环境变量的设置，是管理复杂应用配置的理想选择。                            |
| **错误处理**        | `anyhow`       | 简化错误处理             | 提供符合人体工程学且简单的错误处理方式，避免大量样板代码。                                         |
| **验证**            | `validator`    | 数据验证                 | 支持在结构体上进行声明式验证，与 `axum` 的提取器系统平滑集成。                                     |

---

## 4. 企业级项目结构

对于长期协作项目，推荐使用分层架构来明确分离业务逻辑和基础设施，实现"高内聚低耦合"。

```
.
├── .env              # (可选) 本地开发环境覆盖
├── config/
│   ├── default.toml  # 默认配置
│   └── production.toml # 生产环境配置
├── .gitignore
├── Cargo.toml        # 项目依赖和元数据
├── README.md         # 项目文档（此文件）
└── src/
    ├── main.rs       # (二进制 Crate) 入口点：初始化并启动服务器
    └── app/          # (库 Crate) 核心应用逻辑
        ├── lib.rs        # 声明所有模块，提供 `run()` 函数
        ├── config.rs     # 配置加载和管理
        ├── error.rs      # 统一错误处理类型 (AppError)
        ├── state.rs      # 共享应用状态 (AppState)
        │
        ├── api/          # API 层 (Web 接口)
        │   ├── mod.rs
        │   ├── routes.rs   # 路由定义
        │   ├── handlers/   # HTTP 处理函数（按领域分组）
        │   │   ├── mod.rs
        │   │   └── user_handler.rs
        │   └── middleware/ # 自定义中间件（如 auth.rs）
        │       └── mod.rs
        │
        ├── domain/       # 领域层 (核心业务逻辑)
        │   ├── mod.rs
        │   ├── models/     # 领域模型（如 User, Product）
        │   │   ├── mod.rs
        │   │   └── user.rs
        │   └── services/   # 领域服务（封装业务逻辑）
        │       ├── mod.rs
        │       └── user_service.rs
        │
        └── infrastructure/ # 基础设施层
            ├── mod.rs
            └── persistence/  # 持久化（数据库）
                ├── mod.rs
                └── user_repository.rs
```

### 模块职责

-   **`main.rs`**: **入口点**。其唯一职责是调用 `app` 库来配置和启动服务器。
-   **`app/lib.rs`**: **应用核心**。声明所有模块并为 `main.rs` 提供 `run()` 函数。
-   **`app/config.rs`**: **配置**。使用 `config-rs` 从 `config/` 文件和环境变量加载应用配置。
-   **`app/error.rs`**: **错误处理**。定义统一的 `AppError` 类型并实现 `IntoResponse`。
-   **`app/state.rs`**: **状态管理**。定义 `AppState`，包含数据库池（`sqlx::PgPool`）、配置和其他共享资源。
-   **`app/api/`**: **API/表示层**。处理 HTTP 请求和响应。是应用与外部世界的接口。
    -   `routes.rs`: 将 URL 路径映射到特定的 `handler` 函数。
    -   `handlers/`: 包含 `axum` 处理函数。它们的工作是解析请求、调用领域服务并格式化响应。**不应包含业务逻辑**。
-   **`app/domain/`**: **领域层**。包含所有核心业务逻辑和规则，**完全独立于外部关注点**如数据库或 HTTP。
    -   `models/`: 定义领域实体（纯 Rust 结构体）。
    -   `services/`: 通过编排仓库和领域模型来实现业务用例。
-   **`app/infrastructure/`**: **基础设施层**。提供与外部世界交互的具体实现。
    -   `persistence/`: 数据持久化逻辑。使用 `sqlx` 实现仓库特性以与数据库交互。

### 配置管理

`config-rs` crate 启用了一个强大的分层配置系统：

-   **`config/default.toml`**: 存储所有配置项的默认值。应提交到版本控制。
-   **`config/development.toml`**, **`config/production.toml`**: 环境特定的覆盖。可能包含机密信息，应根据需要忽略 git。
-   **环境变量**: 最终的覆盖，非常适合容器化部署（例如 `APP_DATABASE__URL=...`）。这遵循十二要素应用方法论。

---

## 5. 核心功能与设计模式

### 5.1. 中间件设计

所有中间件都应符合 `tower::Layer` 规范以实现无缝集成。注册顺序至关重要，因为它定义了请求-响应流。

**推荐顺序:**

1.  **请求 ID 层** (最外层)
2.  **CORS 层**
3.  **日志层 (`TraceLayer`)**
4.  **身份验证与授权层**
5.  **全局异常层** (最内层)

```rust
// 示例: 在 main.rs 中注册全局中间件
let app = Router::new()
    .route("/", get(handler))
    // ... 其他路由
    .layer(
        ServiceBuilder::new()
            .layer(RequestIdLayer) // 1. 请求 ID
            .layer(CorsLayer::new().allow_origin(Any)) // 2. CORS
            .layer(TraceLayer::new_for_http()) // 3. 日志
            // 4. 认证中间件通常在路由级别应用
            .layer(GlobalExceptionLayer), // 5. 异常处理
    );
```

### 5.2. 全局异常处理与统一响应

这是一套健壮且一致的 API 的基石。所有服务器响应，无论是成功还是失败，都必须遵循统一的结构。

**设计目标:**

-   拦截所有未处理的 `Result::Err`。
-   将业务错误和系统错误都转换为标准 JSON 格式。
-   在所有响应中包含 `request_id` 以实现可追溯性。

**统一响应体 (JSON):**

-   **成功:**
    ```json
    {
      "code": 0,
      "message": "Success",
      "data": { ... },
      "request_id": "uuid-v4-string"
    }
    ```
-   **失败:**
    ```json
    {
      "code": 1001, // 非零业务错误代码
      "message": "用户名或密码无效。",
      "request_id": "uuid-v4-string"
    }
    ```

**实现:**

1.  **定义 `AppError`**: 在 `src/app/error.rs` 中创建 `AppError` 枚举以表示所有可能的业务和系统错误。
2.  **实现 `IntoResponse`**: 为 `AppError` 实现 `axum::response::IntoResponse` 特性。此实现将错误变体映射到适当的 HTTP 状态码和统一的 JSON 响应。
3.  **在处理器中使用**: 简单地从处理器返回 `Result<T, AppError>`。当 `axum` 遇到 `Err` 时，将自动使用 `IntoResponse` 实现。

### 5.3. 日志中间件

结构化、与请求绑定的日志记录对调试和监控至关重要。

**设计目标:**

-   记录每个请求的进入和退出。
-   包含关键信息：`HTTP 方法`、`URI`、`状态码`、`延迟`。
-   关键是将每个日志条目与 `请求 ID` 关联。

**实现:**

-   使用 `tower_http::trace::TraceLayer` 结合 `tracing` crate。
-   配置 `TraceLayer` 在请求开始时创建 `span` 并在响应时记录信息，确保包含 `request_id`。

### 5.4. 灵活的身份验证与授权

不同的端点需要不同的访问控制。此模板提供了一种基于中间件的灵活机制来处理这个问题。

**核心设计: `Auth` 中间件 + `AuthStrategy` 特性**

设计围绕 `AuthStrategy` 特性展开，该特性定义了任何身份验证方法的契约。

1.  **`AuthStrategy` 特性**: 所有身份验证逻辑的核心抽象。
    ```rust
    // 在: src/app/api/middleware/auth.rs
    #[derive(Debug, Clone)]
    pub struct Claims {
        pub user_id: i32,
        pub roles: Vec<String>,
    }

    #[async_trait]
    pub trait AuthStrategy: Clone + Send + Sync + 'static {
        async fn authenticate(&self, req: &mut Request<Body>) -> Result<Claims, AppError>;
    }
    ```

2.  **具体策略 (`JwtStrategy`, `ApiKeyStrategy`)**: 为每种需要的身份验证方法实现 `AuthStrategy` 特性。

3.  **`auth` 中间件**: 一个通用中间件，接受一个策略，执行它，并将结果 `Claims` 插入到请求扩展中。

4.  **应用到路由**: 使用 `axum` 的 `route_layer` 将不同的身份验证策略应用到不同的路由或路由组。
    ```rust
    // 在: src/app/api/routes.rs
    let jwt_strategy = JwtStrategy::new();
    let user_routes = Router::new()
        .route("/me", get(user_handler::get_me))
        .route_layer(from_fn(move |req, next| {
            auth(jwt_strategy.clone(), req, next)
        }));
    ```

5.  **`CurrentUser` 提取器**: 一个自定义提取器，用于在处理器中便捷地访问已验证用户的 `Claims`。
    ```rust
    // 在处理器中:
    pub async fn get_me(
        CurrentUser(claims): CurrentUser, // 如果已验证则提取 claims
    ) -> Result<Json<Value>, AppError> {
        // ... 使用 claims.user_id 的逻辑
        Ok(Json(json!({ "user_id": claims.user_id })))
    }
    ```

### 5.5. 统一请求验证

输入验证是第一道防线。此模板使用基于 `axum` 和 `validator` crate 构建的自定义提取器，以实现干净、声明式的方法。

**设计目标:**

-   **声明式**: 直接在 DTO 结构体上定义验证规则。
-   **自动**: 提取器自动触发验证。
-   **统一响应**: 失败时自动返回带有统一错误格式的 `400 Bad Request`。

**实现:**

1.  **在 DTO 上定义规则**: 使用 `#[derive(Validate)]` 和验证属性在请求体结构体上定义。
    ```rust
    // 在: src/app/api/handlers/user_handler.rs
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Deserialize, Validate)]
    pub struct CreateUserPayload {
        #[validate(length(min = 3), required)]
        pub username: Option<String>,
        #[validate(email, required)]
        pub email: Option<String>,
    }
    ```

2.  **创建 `ValidatedJson` 提取器**: 一个自定义提取器，包装 `axum::Json`，反序列化然后验证。如果验证失败，返回 `AppError::Validation`。
    ```rust
    // 在: src/app/api/extractors.rs
    #[derive(Debug, Clone, Copy, Default)]
    pub struct ValidatedJson<T>(pub T);

    #[async_trait]
    impl<T, S> FromRequest<S> for ValidatedJson<T>
    where
        T: DeserializeOwned + Validate,
        S: Send + Sync,
    {
        type Rejection = AppError;

        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(|e| AppError::Validation(e.to_string()))?;
            
            value.validate().map_err(|e| AppError::Validation(e.to_string()))?;
            
            Ok(ValidatedJson(value))
        }
    }
    ```

3.  **在处理器中使用**: 简单地将 `Json<T>` 替换为 `ValidatedJson<T>`。处理器代码变得更干净，验证自动处理。
    ```rust
    // 在处理器中:
    pub async fn create_user(
        ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
    ) -> Result<Json<Value>, AppError> {
        // 如果执行到这里，payload 是有效的。
        // ... 业务逻辑
        Ok(Json(json!({ "status": "success" })))
    }
    ```

---

## 6. 开发指南

### 如何添加新的 API 端点

按照现有的分层架构添加新功能（例如"产品"功能）。

1.  **领域层 (`/domain`)**:
    -   在 `src/app/domain/models/product.rs` 中定义核心业务模型。
    -   (可选) 如果有复杂的业务逻辑，创建 `src/app/domain/services/product_service.rs`。

2.  **基础设施层 (`/infrastructure`)**:
    -   在 `src/app/infrastructure/persistence/product_repository.rs` 中实现数据库逻辑。定义如何在数据库中创建、读取、更新或删除产品。

3.  **API 层 (`/api`)**:
    -   在 `src/app/api/handlers/product_handler.rs` 中定义请求/响应 DTO（数据传输对象）并实现验证规则。
    -   在同一文件中创建处理函数。处理器应：
        a. 使用提取器（`State`、`Path`、`ValidatedJson`）获取数据。
        b. 调用适当的领域服务或仓库。
        c. 将结果转换为 `Result<Json<...>, AppError>`。
    -   在 `src/app/api/routes.rs` 中连接新路由。

4.  **注册模块**:
    -   确保所有新模块（`product.rs`、`product_handler.rs` 等）都在其父级 `mod.rs` 文件中正确声明。

---

## 7. 集成 Redis

Redis 是一个高性能的内存键值存储，广泛用于缓存、会话管理和实时分析。

### 先决条件

-   已安装 Rust 工具链
-   Docker（用于在本地运行 Redis）

### 1. 添加依赖到 `Cargo.toml`

添加带有 `tokio-comp` 的 `redis` 以兼容我们的异步栈，以及用于连接池的 `bb8-redis`。

```toml
[dependencies]
# ... 其他依赖
redis = { version = "0.23", features = ["tokio-comp"] }
bb8 = "0.8"
bb8-redis = "0.14"
```

### 2. 使用 Docker 运行 Redis

对于本地开发，使用 Docker 运行 Redis 是最简单的方法。

```bash
docker run -d -p 6379:6379 --name my-redis redis
```

### 3. 配置 Redis 连接

通过应用的配置管理 Redis 连接 URL。

**在 `config/default.toml` 中:**
```toml
[redis]
url = "redis://127.0.0.1:6379/"
```

**在 `src/app/config.rs` 中:**
```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    // ... 其他字段
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}
```

### 4. 创建 Redis 连接池

连接池对高效的连接管理至关重要。

**在 `src/app/state.rs` 中:**
```rust
use bb8_redis::{bb8, RedisConnectionManager};

pub type RedisPool = bb8::Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub redis_pool: RedisPool,
}

pub async fn create_redis_pool(redis_url: &str) -> Result<RedisPool, bb8::RunError<redis::RedisError>> {
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    bb8::Pool::builder().build(manager).await
}
```

**在 `src/main.rs` (或 `app/lib.rs`) 中:**
更新应用启动逻辑以创建 Redis 池并将其添加到共享状态。
```rust
// ...
let config = app::config::load_config().await.unwrap();

let db_pool = app::state::create_db_pool(&config.database.url).await.unwrap();
let redis_pool = app::state::create_redis_pool(&config.redis.url).await.unwrap(); // 创建 Redis 池

let app_state = app::state::AppState {
    db_pool,
    redis_pool,
};

let app = Router::new()
    // ... 路由
    .with_state(app_state); // 与处理器共享状态
// ...
```

### 5. 在处理器中使用 Redis

通过 `AppState` 从 `axum` 处理器访问 Redis 连接池。

```rust
use axum::{extract::State, Json};
use redis::AsyncCommands;
use crate::app::state::AppState;
use crate::app::error::AppError;

pub async fn redis_ping(
    State(state): State<AppState>,
) -> Result<Json<String>, AppError> {
    let mut conn = state.redis_pool.get().await.map_err(|_| {
        AppError::Business(500, "获取 Redis 连接失败".to_string())
    })?;

    let reply: String = conn.set("my_key", "hello").await.map_err(|e| {
        tracing::error!("Redis 错误: {}", e);
        AppError::Business(500, "Redis 命令执行失败".to_string())
    })?;

    Ok(Json(reply))
}
```