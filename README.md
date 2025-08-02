# Rust Web Server Template

This is a high-performance, modular Rust web server template project based on `axum`. It aims to provide a modern, enterprise-grade, and easy-to-use starting point for developers migrating from backgrounds like Java/Kotlin.

## Table of Contents

- [Rust Web Server Template](#rust-web-server-template)
  - [Table of Contents](#table-of-contents)
  - [1. Quick Start](#1-quick-start)
  - [2. Design Philosophy](#2-design-philosophy)
  - [3. Technology Stack](#3-technology-stack)
  - [4. Enterprise-Grade Project Structure](#4-enterprise-grade-project-structure)
    - [Module Responsibilities](#module-responsibilities)
    - [Configuration Management](#configuration-management)
  - [5. Core Features & Design Patterns](#5-core-features--design-patterns)
    - [5.1. Middleware Design](#51-middleware-design)
    - [5.2. Global Exception Handling & Unified Response](#52-global-exception-handling--unified-response)
    - [5.3. Logging Middleware](#53-logging-middleware)
    - [5.4. Flexible Authentication & Authorization](#54-flexible-authentication--authorization)
    - [5.5. Unified Request Validation](#55-unified-request-validation)
  - [6. Development Guide](#6-development-guide)
    - [How to Add a New API Endpoint](#how-to-add-a-new-api-endpoint)
  - [7. Integrating Redis](#7-integrating-redis)

---

## 1. Quick Start

1.  **Install Rust**:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Build the project**:
    ```bash
    cargo build
    ```

3.  **Run the project**:
    ```bash
    cargo run
    ```
    The service will start on the `SERVER_ADDR` configured in the `.env` file (defaults to `127.0.0.1:3000`).

4.  **Test the API**:
    -   **Health Check**:
        ```bash
        curl http://127.0.0.1:3000/health
        ```
    -   **Example: Create a user**:
        ```bash
        curl -X POST http://127.0.0.1:3000/api/v1/users \
             -H "Content-Type: application/json" \
             -d '{"username": "gemini", "email": "gemini@google.com", "age": 25}'
        ```
---

## 2. Design Philosophy

-   **Modern**: Fully embraces asynchronous Rust, built upon the `tokio` ecosystem.
-   **Modular & Scalable**: Adopts a layered architecture (API, Domain, Infrastructure) to ensure high cohesion and low coupling, making the project easy to maintain and extend.
-   **High Performance**: Leverages `hyper` and `tokio` to deliver top-tier networking performance.
-   **Enterprise-Grade**: Integrates essential infrastructure for enterprise applications, including logging, configuration, error handling, and robust authentication/authorization mechanisms.
-   **Developer-Friendly**: Focuses on developer experience by providing clear, reusable patterns (e.g., custom extractors for validation) to minimize boilerplate and improve productivity.

---

## 3. Technology Stack

| Component              | Crate          | Purpose                    | Rationale                                                                                             |
| :--------------------- | :------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| **Web Framework**      | `axum`         | HTTP Routing & Handling    | Developed by the Tokio team, it integrates seamlessly with the ecosystem, offering an elegant and modular design. |
| **Async Runtime**      | `tokio`        | Driving all async operations | The de-facto standard for async Rust, known for its stability and high performance.                   |
| **ORM Framework**      | `sqlx`         | Async SQL Toolkit          | Purely asynchronous with compile-time SQL validation. High performance and type-safe. The top choice for modern Rust apps. |
| **Serialization**      | `serde`        | Data format handling (JSON) | The standard for serialization in the Rust community, powerful and performant.                        |
| **Structured Logging** | `tracing`      | Application & Request Logs | Designed for async apps, providing structured, level-based logging crucial for debugging and monitoring. |
| **HTTP Middleware**    | `tower-http`   | Common Middleware          | Provides essentials like `TraceLayer` (logging) and `CorsLayer`, working perfectly with `axum`.       |
| **Configuration**      | `config-rs`    | Layered Configuration      | Merges settings from files (e.g., TOML) and environment variables, ideal for managing complex app configs. |
| **Error Handling**     | `anyhow`       | Simplified Error Handling  | Provides an ergonomic and simple way to handle errors, avoiding extensive boilerplate code.           |
| **Validation**         | `validator`    | Data Validation            | Enables declarative validation on structs, integrating smoothly with `axum`'s extractor system.       |

---

## 4. Enterprise-Grade Project Structure

For long-term, collaborative projects, a layered architecture is recommended to clearly separate business logic from infrastructure, achieving "high cohesion, low coupling."

```
.
├── .env              # (Optional) Local development environment overrides
├── config/
│   ├── default.toml  # Default configuration
│   └── production.toml # Production environment configuration
├── .gitignore
├── Cargo.toml        # Project dependencies and metadata
├── README.md         # Project documentation (this file)
└── src/
    ├── main.rs       # (Binary Crate) Entry point: initializes and starts the server
    └── app/          # (Library Crate) Core application logic
        ├── lib.rs        # Declares all modules, provides a `run()` function
        ├── config.rs     # Configuration loading and management
        ├── error.rs      # Unified error handling type (AppError)
        ├── state.rs      # Shared application state (AppState)
        │
        ├── api/          # API Layer (Web Interface)
        │   ├── mod.rs
        │   ├── routes.rs   # Route definitions
        │   ├── handlers/   # HTTP handler functions (grouped by domain)
        │   │   ├── mod.rs
        │   │   └── user_handler.rs
        │   └── middleware/ # Custom middleware (e.g., auth.rs)
        │       └── mod.rs
        │
        ├── domain/       # Domain Layer (Core Business Logic)
        │   ├── mod.rs
        │   ├── models/     # Domain models (e.g., User, Product)
        │   │   ├── mod.rs
        │   │   └── user.rs
        │   └── services/   # Domain services (encapsulate business logic)
        │       ├── mod.rs
        │       └── user_service.rs
        │
        └── infrastructure/ # Infrastructure Layer
            ├── mod.rs
            └── persistence/  # Persistence (Database)
                ├── mod.rs
                └── user_repository.rs
```

### Module Responsibilities

-   **`main.rs`**: **Entry Point**. Its single responsibility is to call the `app` library to configure and launch the server.
-   **`app/lib.rs`**: **Application Core**. Declares all modules and provides a `run()` function for `main.rs`.
-   **`app/config.rs`**: **Configuration**. Loads application config from `config/` files and environment variables using `config-rs`.
-   **`app/error.rs`**: **Error Handling**. Defines the unified `AppError` type and implements `IntoResponse`.
-   **`app/state.rs`**: **State Management**. Defines `AppState`, containing the database pool (`sqlx::PgPool`), config, and other shared resources.
-   **`app/api/`**: **API/Presentation Layer**. Handles HTTP requests and responses. It's the application's interface to the outside world.
    -   `routes.rs`: Maps URL paths to specific `handler` functions.
    -   `handlers/`: Contains `axum` handler functions. Their job is to parse requests, call domain services, and format responses. **They should not contain business logic**.
-   **`app/domain/`**: **Domain Layer**. Contains all core business logic and rules, **completely independent of external concerns** like databases or HTTP.
    -   `models/`: Defines domain entities (pure Rust structs).
    -   `services/`: Implements business use cases by orchestrating repositories and domain models.
-   **`app/infrastructure/`**: **Infrastructure Layer**. Provides concrete implementations for interacting with the outside world.
    -   `persistence/`: Data persistence logic. Implements repository traits using `sqlx` to interact with the database.

### Configuration Management

The `config-rs` crate enables a robust, layered configuration system:

-   **`config/default.toml`**: Stores default values for all configuration items. Should be committed to version control.
-   **`config/development.toml`**, **`config/production.toml`**: Environment-specific overrides. May contain secrets and should be git-ignored as needed.
-   **Environment Variables**: The ultimate override, perfect for containerized deployments (e.g., `APP_DATABASE__URL=...`). This follows the Twelve-Factor App methodology.

---

## 5. Core Features & Design Patterns

### 5.1. Middleware Design

All middleware should conform to the `tower::Layer` specification for seamless integration. The registration order is critical as it defines the request-response flow.

**Recommended Order:**

1.  **Request ID Layer** (Outermost)
2.  **CORS Layer**
3.  **Logging Layer (`TraceLayer`)**
4.  **Authentication & Authorization Layer**
5.  **Global Exception Layer** (Innermost)

```rust
// Example: Registering global middleware in main.rs
let app = Router::new()
    .route("/", get(handler))
    // ... other routes
    .layer(
        ServiceBuilder::new()
            .layer(RequestIdLayer) // 1. Request ID
            .layer(CorsLayer::new().allow_origin(Any)) // 2. CORS
            .layer(TraceLayer::new_for_http()) // 3. Logging
            // 4. Auth middleware is typically applied at the route level
            .layer(GlobalExceptionLayer), // 5. Exception Handling
    );
```

### 5.2. Global Exception Handling & Unified Response

This is the cornerstone of a robust and consistent API. All server responses, success or failure, must follow a unified structure.

**Design Goals:**

-   Intercept all unhandled `Result::Err`.
-   Convert both business errors and system errors into a standard JSON format.
-   Include a `request_id` in all responses for traceability.

**Unified Response Body (JSON):**

-   **Success:**
    ```json
    {
      "code": 0,
      "message": "Success",
      "data": { ... },
      "request_id": "uuid-v4-string"
    }
    ```
-   **Failure:**
    ```json
    {
      "code": 1001, // Non-zero business error code
      "message": "Invalid username or password.",
      "request_id": "uuid-v4-string"
    }
    ```

**Implementation:**

1.  **Define `AppError`**: Create an `AppError` enum in `src/app/error.rs` to represent all possible business and system errors.
2.  **Implement `IntoResponse`**: Implement the `axum::response::IntoResponse` trait for `AppError`. This implementation maps error variants to the appropriate HTTP status codes and unified JSON response.
3.  **Use in Handlers**: Simply return `Result<T, AppError>` from handlers. `axum` will automatically use the `IntoResponse` implementation when it encounters an `Err`.

### 5.3. Logging Middleware

Structured, request-bound logging is essential for debugging and monitoring.

**Design Goals:**

-   Log the entry and exit of every request.
-   Include key info: `HTTP Method`, `URI`, `Status Code`, `Latency`.
-   Crucially, associate every log entry with a `Request ID`.

**Implementation:**

-   Use `tower_http::trace::TraceLayer` combined with the `tracing` crate.
-   Configure `TraceLayer` to create a `span` at the beginning of a request and record information upon response, ensuring the `request_id` is included.

### 5.4. Flexible Authentication & Authorization

Different endpoints require different access control. This template provides a flexible, middleware-based mechanism to handle this.

**Core Design: `Auth` Middleware + `AuthStrategy` Trait**

The design revolves around an `AuthStrategy` trait, which defines the contract for any authentication method.

1.  **`AuthStrategy` Trait**: The core abstraction for all authentication logic.
    ```rust
    // In: src/app/api/middleware/auth.rs
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

2.  **Concrete Strategies (`JwtStrategy`, `ApiKeyStrategy`)**: Implement the `AuthStrategy` trait for each authentication method you need.

3.  **`auth` Middleware**: A generic middleware that takes a strategy, executes it, and inserts the resulting `Claims` into request extensions.

4.  **Applying to Routes**: Use `axum`'s `route_layer` to apply different authentication strategies to different routes or route groups.
    ```rust
    // In: src/app/api/routes.rs
    let jwt_strategy = JwtStrategy::new();
    let user_routes = Router::new()
        .route("/me", get(user_handler::get_me))
        .route_layer(from_fn(move |req, next| {
            auth(jwt_strategy.clone(), req, next)
        }));
    ```

5.  **`CurrentUser` Extractor**: A custom extractor to ergonomically access the authenticated user's `Claims` in handlers.
    ```rust
    // In a handler:
    pub async fn get_me(
        CurrentUser(claims): CurrentUser, // Extracts claims if authenticated
    ) -> Result<Json<Value>, AppError> {
        // ... logic using claims.user_id
        Ok(Json(json!({ "user_id": claims.user_id })))
    }
    ```

### 5.5. Unified Request Validation

Input validation is the first line of defense. This template uses a custom extractor built on `axum` and the `validator` crate for a clean, declarative approach.

**Design Goals:**

-   **Declarative**: Define validation rules directly on DTO structs.
-   **Automatic**: Validation is triggered automatically by the extractor.
-   **Unified Response**: Failures automatically return a `400 Bad Request` with a consistent error format.

**Implementation:**

1.  **Define Rules on DTOs**: Use `#[derive(Validate)]` and validation attributes on the request body struct.
    ```rust
    // In: src/app/api/handlers/user_handler.rs
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

2.  **Create `ValidatedJson` Extractor**: A custom extractor that wraps `axum::Json`, deserializes, and then validates. If validation fails, it returns an `AppError::Validation`.
    ```rust
    // In: src/app/api/extractors.rs
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

3.  **Use in Handlers**: Simply replace `Json<T>` with `ValidatedJson<T>`. The handler code becomes cleaner, and validation is handled automatically.
    ```rust
    // In a handler:
    pub async fn create_user(
        ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
    ) -> Result<Json<Value>, AppError> {
        // If execution reaches here, payload is valid.
        // ... business logic
        Ok(Json(json!({ "status": "success" })))
    }
    ```

---

## 6. Development Guide

### How to Add a New API Endpoint

Follow the existing layered architecture to add a new feature (e.g., a "products" feature).

1.  **Domain Layer (`/domain`)**:
    -   Define your core business model in `src/app/domain/models/product.rs`.
    -   (Optional) If there's complex business logic, create a `src/app/domain/services/product_service.rs`.

2.  **Infrastructure Layer (`/infrastructure`)**:
    -   Implement the database logic in `src/app/infrastructure/persistence/product_repository.rs`. Define how to create, read, update, or delete products in the database.

3.  **API Layer (`/api`)**:
    -   Define the request/response DTOs (Data Transfer Objects) and implement validation rules in `src/app/api/handlers/product_handler.rs`.
    -   Create the handler functions in the same file. Handlers should:
        a. Use extractors (`State`, `Path`, `ValidatedJson`) to get data.
        b. Call the appropriate domain service or repository.
        c. Convert the result into a `Result<Json<...>, AppError>`.
    -   Wire up the new routes in `src/app/api/routes.rs`.

4.  **Register Modules**:
    -   Ensure all new modules (`product.rs`, `product_handler.rs`, etc.) are correctly declared in their parent `mod.rs` file.

---

## 7. Integrating Redis

Redis is a high-performance in-memory key-value store, popular for caching, session management, and real-time analytics.

### Prerequisites

-   Rust toolchain installed
-   Docker (for running Redis locally)

### 1. Add Dependencies to `Cargo.toml`

Add `redis` with `tokio-comp` for compatibility with our async stack, and `bb8-redis` for connection pooling.

```toml
[dependencies]
# ... other dependencies
redis = { version = "0.23", features = ["tokio-comp"] }
bb8 = "0.8"
bb8-redis = "0.14"
```

### 2. Run Redis with Docker

For local development, the easiest way to run Redis is with Docker.

```bash
docker run -d -p 6379:6379 --name my-redis redis
```

### 3. Configure Redis Connection

Manage the Redis connection URL through the application's configuration.

**In `config/default.toml`:**
```toml
[redis]
url = "redis://127.0.0.1:6379/"
```

**In `src/app/config.rs`:**
```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    // ... other fields
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}
```

### 4. Create Redis Connection Pool

A connection pool is essential for efficient connection management.

**In `src/app/state.rs`:**
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

**In `src/main.rs` (or `app/lib.rs`):**
Update your application startup logic to create the Redis pool and add it to the shared state.
```rust
// ...
let config = app::config::load_config().await.unwrap();

let db_pool = app::state::create_db_pool(&config.database.url).await.unwrap();
let redis_pool = app::state::create_redis_pool(&config.redis.url).await.unwrap(); // Create Redis pool

let app_state = app::state::AppState {
    db_pool,
    redis_pool,
};

let app = Router::new()
    // ... routes
    .with_state(app_state); // Share state with handlers
// ...
```

### 5. Use Redis in Handlers

Access the Redis connection pool from your `axum` handlers via `AppState`.

```rust
use axum::{extract::State, Json};
use redis::AsyncCommands;
use crate::app::state::AppState;
use crate::app::error::AppError;

pub async fn redis_ping(
    State(state): State<AppState>,
) -> Result<Json<String>, AppError> {
    let mut conn = state.redis_pool.get().await.map_err(|_| {
        AppError::Business(500, "Failed to get Redis connection".to_string())
    })?;

    let reply: String = conn.set("my_key", "hello").await.map_err(|e| {
        tracing::error!("Redis error: {}", e);
        AppError::Business(500, "Redis command failed".to_string())
    })?;

    Ok(Json(reply))
}
```