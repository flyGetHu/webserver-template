# Rust Web Server Template

This is a high-performance, modular Rust web server template project based on **Salvo**. It aims to provide a modern, enterprise-grade, and easy-to-use starting point for developers migrating from backgrounds like Java/Kotlin.

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
| **Web Framework**      | `salvo`        | HTTP Routing & Handling    | High-performance, modular web framework with excellent middleware support and OpenAPI integration. |
| **Async Runtime**      | `tokio`        | Driving all async operations | The de-facto standard for async Rust, known for its stability and high performance.                   |
| **Database**           | `sqlx`         | Async SQL Toolkit          | Purely asynchronous with compile-time SQL validation. High performance and type-safe. The top choice for modern Rust apps. |
| **Serialization**      | `serde`        | Data format handling (JSON) | The standard for serialization in the Rust community, powerful and performant.                        |
| **Structured Logging** | `tracing`      | Application & Request Logs | Designed for async apps, providing structured, level-based logging crucial for debugging and monitoring. |
| **Configuration**      | `config-rs`    | Layered Configuration      | Merges settings from files (e.g., TOML) and environment variables, ideal for managing complex app configs. |
| **Error Handling**     | `anyhow`       | Simplified Error Handling  | Provides an ergonomic and simple way to handle errors, avoiding extensive boilerplate code.           |
| **Validation**         | `validator`    | Data Validation            | Enables declarative validation on structs, integrating smoothly with Salvo's handler system.       |
| **OpenAPI/Swagger**    | `salvo-oapi`   | API Documentation          | Built-in OpenAPI support for automatic API documentation generation.                                 |

---

## 4. Enterprise-Grade Project Structure

This project adopts a **modular architecture** that combines the best of domain-driven design with Salvo's best practices, achieving "high cohesion, low coupling."

```
.
├── .env              # (Optional) Local development environment overrides
├── .cursor/          # Cursor IDE rules and configurations
│   └── rules/        # Project-specific coding standards
├── config/
│   ├── default.toml  # Default configuration
│   └── production.toml # Production environment configuration
├── .gitignore
├── Cargo.toml        # Project dependencies and metadata
├── README.md         # Project documentation (this file)
└── src/
    ├── main.rs       # (Binary Crate) Entry point: initializes and starts the server
    └── app/          # (Library Crate) Core application logic
        ├── mod.rs        # Declares all modules, provides a `run()` function
        ├── config.rs     # Configuration loading and management
        ├── container.rs  # Service container and dependency injection
        ├── error.rs      # Unified error handling type (AppError)
        ├── state.rs      # Shared application state (AppState)
        │
        ├── api/          # API Infrastructure Layer
        │   ├── mod.rs
        │   ├── routes.rs     # Main route aggregation
        │   ├── response.rs   # Unified response types
        │   ├── docs.rs       # OpenAPI/Swagger documentation
        │   ├── extractors.rs # Custom extractors for validation
        │   ├── middleware/   # Cross-cutting concerns
        │   │   ├── mod.rs
        │   │   ├── auth.rs
        │   │   ├── request_id.rs
        │   │   ├── request_logger.rs
        │   │   └── global_exception_handler.rs
        │   └── telemetry/    # Request tracing and monitoring
        │       ├── mod.rs
        │       └── request_id_format.rs
        │
        ├── modules/      # Feature Modules (Domain-Driven)
        │   ├── mod.rs
        │   ├── auth/         # Authentication module
        │   │   ├── mod.rs
        │   │   ├── handlers.rs   # HTTP handlers
        │   │   ├── models.rs     # DTOs and request/response types
        │   │   ├── routes.rs     # Module-specific routes
        │   │   └── services.rs   # Business logic
        │   ├── users/        # User management module
        │   │   ├── mod.rs
        │   │   ├── handlers.rs
        │   │   ├── models.rs
        │   │   ├── routes.rs
        │   │   └── services.rs
        │   └── health/       # Health check module
        │       ├── mod.rs
        │       └── handlers.rs
        │
        ├── domain/       # Shared Domain Layer
        │   ├── mod.rs
        │   ├── models/       # Core domain entities
        │   │   ├── mod.rs
        │   │   └── user.rs
        │   └── services/     # Shared domain services
        │       ├── mod.rs
        │       ├── auth_service.rs
        │       └── user_service.rs
        │
        └── infrastructure/ # Infrastructure Layer
            ├── mod.rs
            └── persistence/  # Data persistence
                ├── mod.rs
                └── user_repository.rs
```

### Module Responsibilities

#### Core Application
-   **`main.rs`**: **Entry Point**. Its single responsibility is to call the `app` library to configure and launch the server.
-   **`app/mod.rs`**: **Application Core**. Declares all modules and provides a `run()` function for `main.rs`.
-   **`app/config.rs`**: **Configuration**. Loads application config from `config/` files and environment variables using `config-rs`.
-   **`app/error.rs`**: **Error Handling**. Defines the unified `AppError` type and implements Salvo's `Scribe` trait.
-   **`app/state.rs`**: **State Management**. Defines `AppState`, containing the database pool, Redis pool, and other shared resources.
-   **`app/container.rs`**: **Service Container**. Manages dependency injection using Salvo's Depot mechanism for clean service management.

#### API Infrastructure Layer (`app/api/`)
Provides cross-cutting concerns and infrastructure for HTTP handling:
-   **`routes.rs`**: Aggregates all module routes into the main application router.
-   **`response.rs`**: Defines unified response types for consistent API responses.
-   **`docs.rs`**: OpenAPI/Swagger documentation configuration.
-   **`extractors.rs`**: Custom extractors for validation and data parsing.
-   **`middleware/`**: Cross-cutting concerns like authentication, logging, and error handling.
-   **`telemetry/`**: Request tracing and monitoring utilities.

#### Feature Modules (`app/modules/`)
Each module represents a business domain with complete encapsulation:
-   **`handlers.rs`**: Salvo handlers that parse requests, call services, and format responses. **Should not contain business logic**.
-   **`models.rs`**: DTOs, request/response types with validation rules using `validator`.
-   **`routes.rs`**: Module-specific route definitions that wire handlers to URLs.
-   **`services.rs`**: Business logic implementation specific to this module.

#### Shared Domain Layer (`app/domain/`)
Contains shared business logic and entities:
-   **`models/`**: Core domain entities (pure Rust structs) shared across modules.
-   **`services/`**: Shared domain services that implement complex business use cases.

#### Infrastructure Layer (`app/infrastructure/`)
Provides concrete implementations for external integrations:
-   **`persistence/`**: Data persistence logic using `sqlx` for database interactions.

### Configuration Management

The `config-rs` crate enables a robust, layered configuration system:

-   **`config/default.toml`**: Stores default values for all configuration items. Should be committed to version control.
-   **`config/development.toml`**, **`config/production.toml`**: Environment-specific overrides. May contain secrets and should be git-ignored as needed.
-   **Environment Variables**: The ultimate override, perfect for containerized deployments (e.g., `APP_DATABASE__URL=...`). This follows the Twelve-Factor App methodology.

---

## 5. Core Features & Design Patterns

### 5.1. Modular Architecture

This project adopts a **modular architecture** where each business domain is encapsulated in its own module. This approach provides:

- **Domain Isolation**: Each module contains its own handlers, models, routes, and services
- **Scalability**: Easy to add new features without affecting existing code
- **Team Collaboration**: Different teams can work on different modules independently
- **Maintainability**: Clear boundaries make the codebase easier to understand and maintain

### 5.2. Salvo Middleware Design

Salvo uses a powerful middleware system based on handlers. The registration order is critical as it defines the request-response flow.

**Recommended Order:**

1.  **Request ID Middleware** (Outermost)
2.  **Service Injection Middleware**
3.  **Request Logging Middleware**
4.  **Authentication & Authorization Middleware** (Route-specific)
5.  **Global Exception Handler** (Innermost)

```rust
// Example: Registering global middleware
let router = create_routes()
    .hoop(create_request_id_middleware()) // 1. Request ID
    .hoop(request_id_handler) // 2. Request ID integration with tracing
    .hoop(inject_services) // 3. Service injection
    .hoop(request_logger) // 4. Request logging
    .hoop(global_exception_handler); // 5. Exception handling
```

### 5.3. Global Exception Handling & Unified Response

This is the cornerstone of a robust and consistent API. All server responses, success or failure, must follow a unified structure.

**Design Goals:**

-   Intercept all unhandled `Result::Err` from handlers.
-   Convert both business errors and system errors into a standard JSON format.
-   Include a `request_id` in all responses for traceability.
-   Integrate seamlessly with Salvo's error handling system.

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
2.  **Implement `Scribe`**: Implement Salvo's `Scribe` trait for `AppError`. This trait defines how errors are rendered into HTTP responses.
3.  **Use in Handlers**: Simply return `Result<T, AppError>` from handlers. Salvo will automatically use the `Scribe` implementation when it encounters an `Err`.
4.  **Global Exception Middleware**: Use middleware to catch any unhandled errors and ensure consistent error formatting.

### 5.4. Logging Middleware

Structured, request-bound logging is essential for debugging and monitoring.

**Design Goals:**

-   Log the entry and exit of every request.
-   Include key info: `HTTP Method`, `URI`, `Status Code`, `Latency`, `Client IP`.
-   Crucially, associate every log entry with a `Request ID`.
-   Integrate with Salvo's request/response lifecycle.

**Implementation:**

-   Use custom middleware combined with the `tracing` crate.
-   Create spans at the beginning of requests and record information upon response.
-   Leverage Salvo's `Depot` to store and retrieve request-scoped data like request IDs.

### 5.5. Flexible Authentication & Authorization

Different endpoints require different access control. This template provides a flexible, middleware-based mechanism using Salvo's powerful middleware system.

**Core Design: Salvo Middleware + Claims**

The design leverages Salvo's `Depot` for storing authentication state and middleware for enforcement.

1.  **`Claims` Structure**: Represents authenticated user information.
    ```rust
    // In: src/app/api/middleware/auth.rs
    #[derive(Debug, Clone)]
    pub struct Claims {
        pub user_id: i32,
        pub username: String,
        pub email: String,
        pub roles: Vec<String>,
    }
    ```

2.  **JWT Authentication Middleware**: Validates JWT tokens and extracts user claims.
    ```rust
    #[handler]
    pub async fn jwt_auth(
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // Extract and validate JWT token
        // Store claims in depot for handlers to access
        if let Some(claims) = validate_jwt_token(req) {
            depot.insert("current_user", claims);
            ctrl.call_next(req, depot, res).await;
        } else {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(json!({"error": "Unauthorized"})));
        }
    }
    ```

3.  **Applying to Routes**: Use Salvo's `.hoop()` method to apply authentication to specific routes.
    ```rust
    // In module routes
    Router::with_path("users")
        .hoop(jwt_auth) // Apply authentication
        .get(list_users)
        .post(create_user)
    ```

4.  **`CurrentUser` Extractor**: A custom extractor to access authenticated user information.
    ```rust
    // In a handler:
    pub async fn get_me(depot: &mut Depot) -> Result<Json<UserResponse>, AppError> {
        let current_user = depot.get::<Claims>("current_user")
            .ok_or(AppError::Unauthorized("Not authenticated".to_string()))?;
        
        // ... logic using current_user
        Ok(Json(user_response))
    }
    ```

### 5.6. Unified Request Validation

Input validation is the first line of defense. This template uses the `validator` crate integrated with Salvo's request parsing for a clean, declarative approach.

**Design Goals:**

-   **Declarative**: Define validation rules directly on DTO structs using attributes.
-   **Automatic**: Validation is triggered automatically during request parsing.
-   **Unified Response**: Failures automatically return a `400 Bad Request` with a consistent error format.
-   **OpenAPI Integration**: Validation rules are automatically reflected in OpenAPI documentation.

**Implementation:**

1.  **Define Rules on DTOs**: Use `#[derive(Validate)]` and validation attributes on request structs.
    ```rust
    // In: src/app/modules/users/models.rs
    use serde::Deserialize;
    use salvo::oapi::ToSchema;
    use validator::Validate;

    #[derive(Deserialize, Validate, ToSchema)]
    pub struct CreateUserRequest {
        #[validate(length(min = 3, max = 50))]
        pub username: String,
        
        #[validate(email)]
        pub email: String,
        
        #[validate(range(min = 18, max = 120))]
        pub age: Option<i32>,
    }
    ```

2.  **Validation in Handlers**: Use Salvo's built-in JSON parsing with manual validation.
    ```rust
    // In a handler:
    pub async fn create_user(
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
    ) -> Result<(), AppError> {
        let payload = req
            .parse_json::<CreateUserRequest>()
            .await
            .map_err(|e| AppError::Validation(e.to_string()))?;
        
        // Validate the payload
        payload.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        
        // If execution reaches here, payload is valid
        // ... business logic
        Ok(())
    }
    ```

3.  **Custom Extractor (Optional)**: For more ergonomic usage, create a custom validated extractor.
    ```rust
    // In: src/app/api/extractors.rs
    pub struct ValidatedJson<T>(pub T);

    impl<T> ValidatedJson<T>
    where
        T: DeserializeOwned + Validate,
    {
        pub async fn extract(req: &mut Request) -> Result<Self, AppError> {
            let value = req
                .parse_json::<T>()
                .await
                .map_err(|e| AppError::Validation(e.to_string()))?;
            
            value.validate()
                .map_err(|e| AppError::Validation(e.to_string()))?;
            
            Ok(ValidatedJson(value))
        }
    }
    ```

---

## 6. Development Guide

### How to Add a New Feature Module

Follow the modular architecture to add a new feature (e.g., a "products" feature).

1.  **Create Module Directory**:
    ```bash
    mkdir src/app/modules/products
    ```

2.  **Create Module Files**:
    ```bash
    # Core module files
    touch src/app/modules/products/mod.rs
    touch src/app/modules/products/handlers.rs
    touch src/app/modules/products/models.rs
    touch src/app/modules/products/routes.rs
    touch src/app/modules/products/services.rs
    ```

3.  **Define Models** (`src/app/modules/products/models.rs`):
    ```rust
    use serde::{Deserialize, Serialize};
    use salvo::oapi::ToSchema;
    use validator::Validate;

    #[derive(Deserialize, Validate, ToSchema)]
    pub struct CreateProductRequest {
        #[validate(length(min = 1, max = 100))]
        pub name: String,
        
        #[validate(length(max = 500))]
        pub description: Option<String>,
        
        #[validate(range(min = 0.01))]
        pub price: f64,
    }

    #[derive(Serialize, ToSchema)]
    pub struct ProductResponse {
        pub id: i32,
        pub name: String,
        pub description: Option<String>,
        pub price: f64,
        pub created_at: String,
    }
    ```

4.  **Implement Handlers** (`src/app/modules/products/handlers.rs`):
    ```rust
    use salvo::prelude::*;
    use crate::app::{
        api::response::ApiResponse,
        error::AppError,
        modules::products::models::*,
    };

    #[handler]
    pub async fn create_product(
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
    ) -> Result<(), AppError> {
        let request_id = depot.get::<String>("request_id").cloned()
            .unwrap_or_else(|| "unknown".to_string());

        let payload = req
            .parse_json::<CreateProductRequest>()
            .await
            .map_err(|e| AppError::Validation(e.to_string()))?;

        payload.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // TODO: Implement business logic
        let response = ProductResponse {
            id: 1,
            name: payload.name,
            description: payload.description,
            price: payload.price,
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        res.render(Json(ApiResponse::new(response, request_id)));
        Ok(())
    }
    ```

5.  **Define Routes** (`src/app/modules/products/routes.rs`):
    ```rust
    use salvo::prelude::*;
    use super::handlers::{create_product};

    pub fn create_routes() -> Router {
        Router::with_path("products")
            .post(create_product)
    }
    ```

6.  **Register Module** (`src/app/modules/mod.rs`):
    ```rust
    pub mod auth;
    pub mod health;
    pub mod users;
    pub mod products; // Add new module

    // Update create_routes function
    pub fn create_routes() -> Router {
        Router::with_path("api/v1")
            .push(auth::create_routes())
            .push(users::create_routes())
            .push(products::create_routes()) // Add new routes
            .push(health::create_routes())
    }
    ```

7.  **Add Domain Logic** (if needed):
    -   For shared domain entities: `src/app/domain/models/product.rs`
    -   For complex business logic: `src/app/domain/services/product_service.rs`
    -   For data persistence: `src/app/infrastructure/persistence/product_repository.rs`

### Service Container Integration

To integrate with the service container for dependency injection:

1.  **Update Container** (`src/app/container.rs`):
    ```rust
    pub struct AppServices {
        // ... existing services
        pub product_service: Arc<ProductService>,
    }
    ```

2.  **Use in Handlers**:
    ```rust
    // Get service from depot (when properly integrated)
    let product_service = depot.get_product_service()?;
    let result = product_service.create_product(payload).await?;
    ```

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