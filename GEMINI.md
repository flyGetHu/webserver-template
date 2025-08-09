# Rust Web Server Template - GEMINI Context

## Project Overview

This project is a high-performance, modular Rust web server template built with the **Salvo** framework. It's designed as a modern, enterprise-grade starting point for developers, especially those transitioning from Java/Kotlin backgrounds. The architecture emphasizes modularity, scalability, and developer experience.

Key features include:
- A layered architecture (API, Domain, Infrastructure).
- Built-in support for logging, configuration management, global error handling, and unified responses.
- Integrated authentication and authorization mechanisms.
- Request validation using the `validator` crate.
- OpenAPI/Swagger documentation generation.
- Database integration with `sqlx`.
- Redis integration example.

## Core Technologies

- **Language**: Rust
- **Framework**: Salvo
- **Async Runtime**: Tokio
- **Database**: SQLx (MySQL)
- **Caching**: Redis
- **Serialization**: Serde
- **Logging**: Tracing
- **Configuration**: Figment (TOML + Environment Variables)
- **Error Handling**: ThisError, Anyhow
- **Validation**: Validator

## Project Structure

The project follows a modular architecture, grouping code by feature rather than technical layer.

```
.
├── .env                   # (Optional) Local development environment overrides
├── .cursor/               # Cursor IDE rules and configurations
├── config/
│   └── default.toml       # Default configuration
├── Cargo.toml             # Project dependencies and metadata
├── README.md              # Project documentation
└── src/
    ├── main.rs            # (Binary Crate) Entry point
    └── app/               # (Library Crate) Core application logic
        ├── mod.rs             # Declares all modules, provides a `run()` function
        ├── config.rs          # Configuration loading and management
        ├── error.rs           # Unified error handling type (AppError)
        ├── state.rs           # Shared application state (AppState)
        ├── container.rs       # Service container and dependency injection
        ├── api/               # API Infrastructure Layer
        │   ├── mod.rs
        │   ├── routes.rs          # Main route aggregation
        │   ├── response.rs        # Unified response types
        │   ├── docs.rs            # OpenAPI/Swagger documentation
        │   ├── extractors.rs      # Custom extractors for validation
        │   ├── middleware/        # Cross-cutting concerns
        │   │   ├── mod.rs
        │   │   ├── auth.rs
        │   │   ├── request_id.rs
        │   │   ├── request_logger.rs
        │   │   └── global_exception_handler.rs
        │   └── telemetry/         # Request tracing and monitoring
        │       ├── mod.rs
        │       └── request_id_format.rs
        ├── modules/           # Feature Modules (Domain-Driven)
        │   ├── mod.rs
        │   ├── auth/              # Authentication module
        │   ├── users/             # User management module
        │   └── health/            # Health check module
        ├── domain/            # Shared Domain Layer
        │   ├── mod.rs
        │   ├── models/            # Core domain entities
        │   └── services/          # Shared domain services
        └── infrastructure/    # Infrastructure Layer
            ├── mod.rs
            └── persistence/       # Data persistence
```

### Module Responsibilities

- **`main.rs`**: Entry point. Calls the `app` library to start the server.
- **`app/`**: Core library crate.
    - **`mod.rs`**: Exposes modules and the main `run()` function.
    - **`config.rs`**: Loads configuration using `figment`.
    - **`error.rs`**: Defines `AppError` and implements Salvo's `Scribe` trait for unified error responses.
    - **`state.rs`**: Manages shared application state like database and Redis pools.
    - **`container.rs`**: Handles dependency injection.
- **`app/api/`**: Infrastructure for HTTP handling.
    - **`routes.rs`**: Aggregates module routes.
    - **`response.rs`**: Defines `ApiResponse<T>` for standard success responses.
    - **`middleware/`**: Contains cross-cutting concerns like request logging, request ID generation, and global exception handling.
- **`app/modules/`**: Feature modules (e.g., `users`, `auth`, `health`). Each contains its own handlers, models, routes, and services.
- **`app/domain/`**: Shared business logic and core entities.
- **`app/infrastructure/`**: Concrete implementations for external systems (e.g., database repositories).

## Building and Running

1.  **Install Rust**: Ensure you have the Rust toolchain installed. If not, run:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **Build the Project**:
    ```bash
    cargo build
    ```
3.  **Run the Project**:
    ```bash
    cargo run
    ```
    The server will start on the address specified by `listen_addr` in `config/default.toml` (default `127.0.0.1:3000`).

4.  **Test the API**:
    -   Health Check:
        ```bash
        curl http://127.0.0.1:3000/health
        ```
    -   Create a user:
        ```bash
        curl -X POST http://127.0.0.1:3000/api/v1/users \
             -H "Content-Type: application/json" \
             -d '{"username": "gemini", "email": "gemini@example.com", "age": 25}'
        ```

### Configuration

Configuration is managed using `figment`, merging settings from `config/default.toml` and environment variables. Environment variables take precedence and follow the format `SECTION__KEY` (e.g., `DATABASE__URL`).

Key configuration items in `config/default.toml`:
- `listen_addr`: The address the server binds to.
- `database.url`: MySQL connection string.
- `redis.url`: Redis connection string.
- `jwt.secret`: Secret key for JWT signing.
- `log.*`: Logging configuration.

## Development Guide

### Adding a New Feature Module

To add a new module (e.g., `products`):
1.  Create the directory `src/app/modules/products`.
2.  Create the files: `mod.rs`, `handlers.rs`, `models.rs`, `routes.rs`, `services.rs`.
3.  Define your request/response models and validation rules in `models.rs`.
4.  Implement your business logic in `services.rs`.
5.  Write your HTTP handlers in `handlers.rs`, focusing on request parsing and response formatting.
6.  Define the module's routes in `routes.rs`.
7.  Register the new module in `src/app/modules/mod.rs` and add its routes to the aggregator in `src/app/api/routes.rs`.

### Error Handling

The project uses a unified `AppError` enum defined in `src/app/error.rs`. Handlers should return `Result<T, AppError>`. The `Scribe` trait implementation for `AppError` ensures all errors are rendered into a consistent JSON response format, including a `request_id` for traceability.

### Logging

Structured logging is implemented using the `tracing` crate. A `request_logger` middleware in `src/app/api/middleware/request_logger.rs` logs the start and completion of every request, including method, URI, status code, latency, and client IP, all associated with a `request_id`.

### Authentication & Authorization

Authentication is handled by a JWT middleware (`src/app/api/middleware/auth.rs`). Routes requiring authentication can be protected by applying this middleware. User claims are stored in the Salvo `Depot` for access in handlers.

### Request Validation

Input validation is declarative, using the `validator` crate. Define validation rules on your request DTO structs (e.g., in `models.rs`) using attributes like `#[validate(length(...))]`. In handlers, parse the request body and call `.validate()` on the struct. Validation errors are automatically converted to `AppError::Validation` and handled by the global error system.

### Unified API Responses

Success responses use the `ApiResponse<T>` struct defined in `src/app/api/response.rs`. This ensures all successful API calls have a consistent structure including `code`, `message`, `request_id`, and `data`.