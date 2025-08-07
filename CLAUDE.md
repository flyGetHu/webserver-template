# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a modern, enterprise-grade Rust web server template built with Axum. It follows a layered architecture with clear separation of concerns:
- API Layer (HTTP handlers and routing)
- Domain Layer (business logic and models)
- Infrastructure Layer (database persistence)

Key features include:
- JWT-based authentication with role-based access control
- Unified error handling and response format
- Request validation with automatic validation
- Structured logging with request tracing
- Service registry pattern for dependency injection
- MySQL database with SQLx ORM
- Redis integration for caching
- OpenAPI documentation with Swagger UI
- Configuration management with layered config

## Common Development Commands

### Build the project
```bash
cargo build
```

### Run the project
```bash
cargo run
```

### Run in development mode
```bash
cargo run
```

### Run tests
```bash
cargo test
```

### Check code formatting
```bash
cargo fmt -- --check
```

### Run linting
```bash
cargo clippy
```

### Run clippy with fixes
```bash
cargo clippy --fix
```

### Format code
```bash
cargo fmt
```

## Project Structure

```
src/
├── main.rs              # Entry point
└── app/                 # Core application logic
    ├── config.rs        # Configuration management
    ├── state.rs         # Shared application state
    ├── error.rs         # Unified error handling
    ├── container.rs     # Service registry (dependency injection)
    ├── container_lazy.rs # Lazy-loaded service registry
    ├── api/             # API layer (handlers, middleware, routes)
    │   ├── routes.rs    # Route definitions
    │   ├── response.rs  # Unified response format
    │   ├── extractors.rs # Custom extractors (ValidatedJson, CurrentUser)
    │   ├── docs.rs      # OpenAPI documentation
    │   ├── handlers/    # HTTP handlers
    │   └── middleware/  # Custom middleware
    ├── domain/          # Domain layer (business logic)
    │   ├── models/      # Domain models and DTOs
    │   └── services/    # Business services
    └── infrastructure/  # Infrastructure layer
        └── persistence/ # Database repositories
```

## Key Architectural Patterns

### Service Registry Pattern
The project uses a type-safe service registry pattern for dependency injection, avoiding struct bloat. Services are registered by type and retrieved using `registry.get::<ServiceType>()` or convenience methods like `registry.user_service()`.

### Unified Error Handling
All errors use the `AppError` enum which implements `IntoResponse` to provide consistent error responses.

### Request Validation
The `ValidatedJson<T>` extractor automatically validates request bodies using the `validator` crate.

### Authentication
JWT-based authentication with middleware (`jwt_auth`) and a `CurrentUser` extractor for accessing authenticated user information.

### Configuration
Layered configuration using `config-rs` that loads from files and environment variables.

## Adding New Features

1. **Domain Layer**: Add models in `src/app/domain/models/` and services in `src/app/domain/services/`
2. **Infrastructure Layer**: Add repositories in `src/app/infrastructure/persistence/`
3. **API Layer**: Add handlers in `src/app/api/handlers/` and register routes in `src/app/api/routes.rs`
4. **Service Registration**: Register new services in the `ServiceRegistry::register_default_services()` method
5. **Configuration**: Add new config sections to `Config` struct and `config/default.toml`

## Database Migrations
Database schema changes are managed through SQL migration files in the `migrations/` directory.

## Environment Configuration
The application loads configuration from:
1. `config/default.toml` (default values)
2. Environment-specific files like `config/production.toml` (based on `RUN_MODE` env var)
3. Environment variables prefixed with `APP_` (highest priority)