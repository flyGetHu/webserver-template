# RBatis 综合使用指南

本文档是项目中 RBatis ORM 框架的完整使用指南，整合了架构设计、最佳实践、具体示例和扩展方法。

## 📋 目录

- [概述](#概述)
- [架构设计](#架构设计)
- [文件组织结构](#文件组织结构)
- [快速开始](#快速开始)
- [基础CRUD操作](#基础crud操作)
- [复杂查询操作](#复杂查询操作)
- [XML SQL完整示例](#xml-sql完整示例)
- [添加新表的步骤](#添加新表的步骤)
- [事务处理](#事务处理)
- [性能优化](#性能优化)
- [调试和日志](#调试和日志)
- [最佳实践](#最佳实践)
- [参考资料](#参考资料)

## 🎯 概述

### 项目中的 RBatis 集成

本项目使用 `rbatis` 作为 ORM 框架，采用 **MyBatis 风格的设计模式**：

- **统一接口设计**：每个表对应一个 Repository，提供所有相关操作
- **简单查询**：使用 RBatis 原生 SQL 和 derive 宏
- **复杂查询**：使用 XML 映射文件和 htmlsql 功能
- **单一出口模式**：每个模块只导出一个 Repository 接口

### 设计理念

- **清晰分层**：Service → Repository → RBatis → Database
- **职责分离**：基础CRUD与复杂查询分离，但统一在一个接口中
- **类型安全**：充分利用 Rust 的类型系统
- **性能优化**：编译时SQL生成 + 动态SQL灵活性

## 🏗️ 架构设计

### 1. 整体架构

```
┌─────────────────────────────────────────┐
│                Service Layer            │
├─────────────────────────────────────────┤
│     Repository (统一接口)               │
│  ┌─────────────────┬─────────────────┐   │
│  │   基础CRUD      │   复杂查询      │   │
│  │  (derive宏)     │  (XML映射)      │   │
│  └─────────────────┴─────────────────┘   │
├─────────────────────────────────────────┤
│            RBatis ORM Framework         │
├─────────────────────────────────────────┤
│  原生 SQL           │  XML 映射文件      │
│  query_decode()     │  mapper.html       │
│  exec()             │  动态 SQL 标签     │
└─────────────────────────────────────────┘
```

### 2. 单一出口设计

每个数据表对应一个文件夹，包含：
- **repository.rs** - 唯一的公共接口
- **xml_repository.rs** - 内部模块（可选）
- **mapper.html** - XML映射文件（可选）
- **mod.rs** - 模块声明

### 3. 技术特性

#### rbatis htmlsql 标准用法

1. **正确的DTD声明**
   ```xml
   <!DOCTYPE mapper PUBLIC "-//rbatis.github.io//DTD Mapper 3.0//EN"
   "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
   ```

2. **htmlsql! 宏使用**
   ```rust
   htmlsql!(select_users_by_complex_condition(rb: &RBatis, query: &ComplexUserQuery) -> Vec<User> => "user_mapper.html");
   ```

3. **动态SQL支持**
   - `<if>` 条件判断
   - `<foreach>` 循环处理
   - `<choose>/<when>/<otherwise>` 多分支选择

## 📁 文件组织结构

### 设计原则

采用**按表分文件夹**的方式组织持久化层代码，每个数据库表对应一个文件夹。

### 目录结构

```
src/app/infrastructure/persistence/
├── mod.rs                           # 模块声明和重新导出
├── RBATIS_COMPREHENSIVE_GUIDE.md    # 本文档
└── users/                           # 用户表相关持久化
    ├── mod.rs                      # 用户模块声明
    ├── repository.rs               # 统一仓库（包含简单和复杂查询）
    ├── xml_repository.rs           # 内部模块（htmlsql宏定义）
    └── user_mapper.html            # XML 映射文件
```

### 模块导出策略

```rust
// users/mod.rs
pub mod repository;
mod xml_repository;  // 内部模块，不导出

pub use repository::UserRepository;
```

## 🚀 快速开始

### 1. 获取数据库连接

在 Service 层中获取 Repository：

```rust
use crate::app::container::ServiceRegistry;

// 在服务中获取 Repository
let user_repo = registry.user_repository();
```

### 2. 基本查询操作

```rust
// 查询单个用户
let user = user_repo.find_by_id(1).await?;

// 检查用户名是否存在
let exists = user_repo.username_exists("admin").await?;

// 分页查询
let users = user_repo.find_all(10, 0).await?;
```

### 3. 创建记录

```rust
use crate::app::domain::models::CreateUserDto;

let create_user = CreateUserDto {
    username: "new_user".to_string(),
    email: "user@example.com".to_string(),
    age: 25,
};

let user = user_repo.create(create_user, "hashed_password".to_string()).await?;
```

### 4. 复杂查询操作

```rust
use crate::app::infrastructure::persistence::users::ComplexUserQuery;

// 动态条件查询
let query = ComplexUserQuery {
    username: Some("john%".to_string()),
    email: Some("%@example.com".to_string()),
    is_active: Some(true),
    age_min: Some(18),
    age_max: Some(65),
    // ...
};

let result = user_repo.find_users_by_complex_condition(&query).await?;
```

## 📊 基础CRUD操作

### 查询模式

#### 单行查询
```rust
pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
    let users: Vec<User> = self
        .rb
        .query_decode("SELECT * FROM users WHERE id = ?", vec![value!(id)])
        .await
        .map_err(AppError::Database)?;
    Ok(users.into_iter().next())
}
```

#### 多行查询
```rust
pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
    let users: Vec<User> = self
        .rb
        .query_decode(
            "SELECT * FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?",
            vec![value!(limit), value!(offset)],
        )
        .await
        .map_err(AppError::Database)?;
    Ok(users)
}
```

### 插入操作

```rust
pub async fn create(&self, user_data: CreateUserDto, password_hash: String) -> Result<User, AppError> {
    let sql = r"
        INSERT INTO users (username, email, password_hash, age, roles, is_active)
        VALUES (?, ?, ?, ?, ?, ?)
    ";
    let args = vec![
        value!(&user_data.username),
        value!(&user_data.email),
        value!(password_hash),
        value!(user_data.age),
        value!(serde_json::to_string(&vec!["user".to_string()])?),
        value!(true),
    ];
    let exec = self.rb.exec(sql, args).await.map_err(AppError::Database)?;
    
    let user_id: i64 = exec.last_insert_id.into();
    let user_id = i32::try_from(user_id)
        .map_err(|_| AppError::Internal("last_insert_id out of range".to_string()))?;
    
    self.find_by_id(user_id).await?.ok_or(AppError::NotFound("User not found".to_string()))
}
```

### 更新操作

```rust
pub async fn update_status(&self, id: i32, is_active: bool) -> Result<(), AppError> {
    self.rb
        .exec(
            "UPDATE users SET is_active = ? WHERE id = ?",
            vec![value!(is_active), value!(id)],
        )
        .await
        .map_err(AppError::Database)?;
    Ok(())
}
```

### 存在性检查

```rust
pub async fn username_exists(&self, username: &str) -> Result<bool, AppError> {
    let rows: Vec<i64> = self
        .rb
        .query_decode(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)",
            vec![value!(username)],
        )
        .await
        .map_err(AppError::Database)?;
    Ok(rows.into_iter().next().unwrap_or(0) != 0)
}
```

## 🔍 复杂查询操作

### XML 映射文件结构

```xml
<!DOCTYPE mapper PUBLIC "-//rbatis.github.io//DTD Mapper 3.0//EN"
"https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <!-- 复杂条件查询用户 -->
    <select id="select_users_by_complex_condition">
        SELECT id, username, email, password_hash, is_active, created_at, updated_at
        FROM users
        WHERE 1=1
        <if test="username != null">
            AND username LIKE CONCAT('%', #{username}, '%')
        </if>
        <if test="email != null">
            AND email LIKE CONCAT('%', #{email}, '%')
        </if>
        <if test="is_active != null">
            AND is_active = #{is_active}
        </if>
        <if test="start_date != null">
            AND created_at >= #{start_date}
        </if>
        <if test="end_date != null">
            AND created_at <= #{end_date}
        </if>
        ORDER BY ${sort_by} ${sort_order}
        LIMIT #{limit} OFFSET #{offset}
    </select>
</mapper>
```

### htmlsql 宏定义

```rust
// xml_repository.rs (内部模块)
use rbatis::htmlsql;
use rbatis::RBatis;

// 定义复杂查询的宏
htmlsql!(select_users_by_complex_condition(rb: &RBatis, query: &ComplexUserQuery) -> Vec<User> => "user_mapper.html");
htmlsql!(count_users_by_complex_condition(rb: &RBatis, query: &ComplexUserQuery) -> i64 => "user_mapper.html");
htmlsql!(get_user_statistics(rb: &RBatis, start_date: Option<&str>, end_date: Option<&str>) -> UserStatistics => "user_mapper.html");
```

### Repository 中的复杂查询方法

```rust
// repository.rs
use super::xml_repository::*;

impl UserRepository {
    /// 复杂条件查询用户（分页）
    pub async fn find_users_by_complex_condition(
        &self,
        query: &ComplexUserQuery,
    ) -> Result<PaginatedResponse<User>, AppError> {
        // 调用XML中定义的复杂查询
        let users = select_users_by_complex_condition(&self.rb, query)
            .await
            .map_err(AppError::Database)?;
        
        // 获取总数
        let total = count_users_by_complex_condition(&self.rb, query)
            .await
            .map_err(AppError::Database)?;
        
        Ok(PaginatedResponse::new(
            users,
            query.page.unwrap_or(1),
            query.page_size.unwrap_or(10),
            total,
        ))
    }
}
```

## 📝 XML SQL完整示例

### 数据结构定义

```rust
use serde::{Deserialize, Serialize};

/// 复杂查询参数结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexUserQuery {
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub age_min: Option<i32>,
    pub age_max: Option<i32>,
    pub roles: Option<Vec<String>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

/// 用户统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub total_users: i64,
    pub active_users: i64,
    pub inactive_users: i64,
    pub avg_age: Option<f64>,
    pub newest_user_date: Option<String>,
    pub oldest_user_date: Option<String>,
}

/// 用户登录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithLoginInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub last_login_at: Option<String>,
    pub login_count: i64,
    pub is_active: bool,
}
```

### 完整的Repository实现示例

```rust
impl UserRepository {
    /// 复杂条件查询用户 - 完整示例
    pub async fn find_users_by_complex_condition(
        &self,
        query: &ComplexUserQuery,
    ) -> Result<PaginatedResponse<User>, AppError> {
        // 步骤1: 调用 XML 中定义的复杂查询
        let users = select_users_by_complex_condition(&self.rb, query)
            .await
            .map_err(|e| {
                log::error!("复杂条件查询用户失败: {}", e);
                AppError::Database(e)
            })?;
        
        // 步骤2: 获取符合条件的总数
        let total = count_users_by_complex_condition(&self.rb, query)
            .await
            .map_err(|e| {
                log::error!("统计复杂条件用户数量失败: {}", e);
                AppError::Database(e)
            })?;
        
        // 步骤3: 构建分页响应
        let page_size = query.page_size.unwrap_or(10);
        let current_page = query.page.unwrap_or(1);
        
        Ok(PaginatedResponse::new(
            users,
            current_page,
            page_size,
            total,
        ))
    }
    
    /// 获取用户统计信息
    pub async fn get_user_statistics(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<UserStatistics, AppError> {
        get_user_statistics(&self.rb, start_date, end_date)
            .await
            .map_err(AppError::Database)
    }
}
```

### Service 层调用示例

```rust
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub async fn search_users_advanced(
        &self,
        username: Option<String>,
        email: Option<String>,
        is_active: Option<bool>,
        page: i64,
        page_size: i64,
    ) -> Result<PaginatedResponse<User>, AppError> {
        // 构建查询参数
        let query = ComplexUserQuery {
            username,
            email,
            is_active,
            start_date: None,
            end_date: None,
            age_min: None,
            age_max: None,
            roles: None,
            sort_by: Some("created_at".to_string()),
            sort_order: Some("DESC".to_string()),
            page: Some(page),
            page_size: Some(page_size),
        };
        
        // 调用 Repository 方法
        self.user_repository.find_users_by_complex_condition(&query).await
    }
}
```

## ➕ 添加新表的步骤

### 步骤 1: 创建表文件夹

例如添加产品表 `products`：

```bash
mkdir src/app/infrastructure/persistence/products
```

### 步骤 2: 创建模块文件

**products/mod.rs**:
```rust
//! 产品表相关的持久化模块

pub mod repository;
mod xml_repository;  // 内部模块（如果需要复杂查询）

pub use repository::ProductRepository;
```

### 步骤 3: 创建 Repository 文件

**products/repository.rs**:
```rust
use std::sync::Arc;
use rbatis::RBatis;
use rbs::value;
use crate::app::{
    domain::models::product::Product,
    error::AppError,
};

#[derive(Clone)]
pub struct ProductRepository {
    rb: Arc<RBatis>,
}

impl ProductRepository {
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    /// 根据ID查找产品
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Product>, AppError> {
        let products: Vec<Product> = self
            .rb
            .query_decode("SELECT * FROM products WHERE id = ?", vec![value!(id)])
            .await
            .map_err(AppError::Database)?;
        Ok(products.into_iter().next())
    }

    /// 创建产品
    pub async fn create(&self, product: CreateProductDto) -> Result<Product, AppError> {
        let sql = r"
            INSERT INTO products (name, price, description, stock_quantity)
            VALUES (?, ?, ?, ?)
        ";
        let args = vec![
            value!(&product.name),
            value!(product.price),
            value!(&product.description),
            value!(product.stock_quantity),
        ];
        
        let exec = self.rb.exec(sql, args).await.map_err(AppError::Database)?;
        let product_id: i64 = exec.last_insert_id.into();
        let product_id = i32::try_from(product_id)
            .map_err(|_| AppError::Internal("last_insert_id out of range".to_string()))?;
        
        self.find_by_id(product_id).await?.ok_or(AppError::NotFound("Product not found".to_string()))
    }

    /// 分页查询产品
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Product>, AppError> {
        let products: Vec<Product> = self
            .rb
            .query_decode(
                "SELECT * FROM products ORDER BY created_at DESC LIMIT ? OFFSET ?",
                vec![value!(limit), value!(offset)],
            )
            .await
            .map_err(AppError::Database)?;
        Ok(products)
    }
}
```

### 步骤 4: 创建 XML 映射文件（可选）

**products/product_mapper.html**:
```xml
<!DOCTYPE mapper PUBLIC "-//rbatis.github.io//DTD Mapper 3.0//EN"
"https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
<mapper>
    <!-- 产品搜索查询 -->
    <select id="search_products">
        SELECT id, name, price, description, stock_quantity, created_at, updated_at
        FROM products
        <where>
            <if test="keyword != null and keyword != ''">
                AND (name LIKE CONCAT('%', #{keyword}, '%')
                OR description LIKE CONCAT('%', #{keyword}, '%'))
            </if>
            <if test="min_price != null">
                AND price >= #{min_price}
            </if>
            <if test="max_price != null">
                AND price <= #{max_price}
            </if>
            <if test="in_stock != null and in_stock == true">
                AND stock_quantity > 0
            </if>
        </where>
        ORDER BY created_at DESC
        LIMIT #{limit} OFFSET #{offset}
    </select>
</mapper>
```

### 步骤 5: 更新主模块

在 **persistence/mod.rs** 中添加：
```rust
pub mod products;

// 重新导出
pub use products::ProductRepository;
```

### 步骤 6: 更新服务容器

在 **container.rs** 中注册新的 Repository：
```rust
use crate::app::infrastructure::persistence::ProductRepository;

#[derive(Clone)]
pub struct RepositoryRegistry {
    pub user_repository: Arc<UserRepository>,
    pub product_repository: Arc<ProductRepository>,
    // ...
}

impl RepositoryRegistry {
    pub fn new(app_state: Arc<AppState>) -> Self {
        let user_repository = Arc::new(UserRepository::new(app_state.rb.clone()));
        let product_repository = Arc::new(ProductRepository::new(app_state.rb.clone()));
        
        Self {
            user_repository,
            product_repository,
        }
    }
}
```

## 🔄 事务处理

### 基本事务操作

```rust
use rbatis::executor::Executor;

pub async fn create_user_with_profile(
    &self,
    user_data: CreateUserDto,
    password_hash: String,
    profile_data: CreateUserProfileDto
) -> Result<User, AppError> {
    let tx = self.rb.acquire_begin().await.map_err(AppError::Database)?;
    
    // 创建用户
    let user = self.create_in_tx(&tx, user_data, password_hash).await?;
    
    // 创建用户资料
    let profile_sql = r"
        INSERT INTO user_profiles (user_id, bio, avatar_url)
        VALUES (?, ?, ?)
    ";
    tx.exec(
        profile_sql,
        vec![
            value!(user.id),
            value!(&profile_data.bio),
            value!(&profile_data.avatar_url),
        ],
    ).await.map_err(AppError::Database)?;
    
    tx.commit().await.map_err(AppError::Database)?;
    Ok(user)
}

async fn create_in_tx(
    &self,
    tx: &dyn Executor,
    user_data: CreateUserDto,
    password_hash: String
) -> Result<User, AppError> {
    let sql = r"
        INSERT INTO users (username, email, password_hash, age, roles, is_active)
        VALUES (?, ?, ?, ?, ?, ?)
    ";
    let args = vec![
        value!(&user_data.username),
        value!(&user_data.email),
        value!(password_hash),
        value!(user_data.age),
        value!(serde_json::to_string(&vec!["user".to_string()])?),
        value!(true),
    ];
    
    let exec = tx.exec(sql, args).await.map_err(AppError::Database)?;
    let user_id: i64 = exec.last_insert_id.into();
    let user_id = i32::try_from(user_id)
        .map_err(|_| AppError::Internal("last_insert_id out of range".to_string()))?;
    
    self.find_by_id(user_id).await?.ok_or(AppError::NotFound("User not found".to_string()))
}
```

## ⚡ 性能优化

### 1. 索引优化

确保查询字段有索引：

```sql
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_is_active ON users(is_active);
```

### 2. 查询优化

```rust
// 只查询需要的字段
pub async fn find_user_summary(&self, id: i32) -> Result<Option<UserSummary>, AppError> {
    let sql = r"
        SELECT id, username, email, created_at 
        FROM users 
        WHERE id = ?
    ";
    let users: Vec<UserSummary> = self
        .rb
        .query_decode(sql, vec![value!(id)])
        .await
        .map_err(AppError::Database)?;
    Ok(users.into_iter().next())
}
```

### 3. 批量操作

```rust
pub async fn batch_create_users(&self, users: Vec<CreateUserDto>) -> Result<Vec<i64>, AppError> {
    let sql = r"
        INSERT INTO users (username, email, password_hash, age, roles, is_active)
        VALUES (?, ?, ?, ?, ?, ?)
    ";
    
    let mut insert_ids = Vec::new();
    
    for user_data in users {
        let args = vec![
            value!(&user_data.username),
            value!(&user_data.email),
            value!("hashed_password"), // 实际应用中应该为每个用户生成不同的哈希
            value!(user_data.age),
            value!(serde_json::to_string(&vec!["user".to_string()])?),
            value!(true),
        ];
        
        let exec = self.rb.exec(sql, args).await.map_err(AppError::Database)?;
        insert_ids.push(exec.last_insert_id.into());
    }
    
    Ok(insert_ids)
}
```

### 4. 连接池配置

```rust
// 在配置文件中优化连接池
[database]
max_connections = 10
min_connections = 5
connect_timeout = 30
idle_timeout = 600
max_lifetime = 1800
```

## 🐛 调试和日志

### 1. 启用 SQL 日志

在配置文件中设置：

```toml
[database]
enable_log = true
log_level = "debug"
```

### 2. 查看执行的 SQL

```rust
// 在 Repository 方法中添加调试日志
pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
    log::debug!("Querying user with id: {}", id);
    let sql = "SELECT * FROM users WHERE id = ?";
    log::debug!("Executing SQL: {}", sql);
    
    let users: Vec<User> = self
        .rb
        .query_decode(sql, vec![value!(id)])
        .await
        .map_err(AppError::Database)?;
    
    log::debug!("Found {} users", users.len());
    Ok(users.into_iter().next())
}
```

### 3. 错误处理和日志

```rust
.await.map_err(|e| {
    log::error!("Database query failed: {}", e);
    log::error!("SQL: {}", sql);
    log::error!("Args: {:?}", args);
    AppError::Database(e)
})?
```

## 🎯 最佳实践

### 1. 架构设计原则

- **单一职责**：每个Repository专注于一个表的操作
- **统一接口**：每个模块只导出一个Repository
- **内部模块化**：复杂查询逻辑封装在内部模块中
- **类型安全**：充分利用Rust的类型系统

### 2. 查询策略

- **简单查询**：使用RBatis原生方法，性能更好
- **复杂查询**：使用XML映射，灵活性更高
- **动态查询**：合理使用条件标签，避免SQL注入
- **分页查询**：统一使用分页响应结构

### 3. 代码组织

- **模块分离**：基础CRUD与复杂查询分离
- **统一错误处理**：使用AppError进行错误转换
- **日志记录**：关键操作添加日志
- **参数验证**：在调用前验证必要参数

### 4. 性能优化

- **索引使用**：为常用查询字段创建索引
- **查询优化**：只查询需要的字段
- **批量操作**：合理使用批量插入/更新
- **连接池**：合理配置数据库连接池

### 5. 安全考虑

- **参数绑定**：使用参数绑定防止SQL注入
- **权限控制**：在Service层进行权限检查
- **数据验证**：输入数据的格式和范围验证
- **敏感信息**：避免在日志中记录敏感信息

## 📚 参考资料

### 官方文档

- [RBatis 官方文档](https://rbatis.github.io/rbatis.io/)
- [RBatis htmlsql 文档](https://rbatis.github.io/rbatis.io/#/en/htmlsql)
- [RBatis GitHub 仓库](https://github.com/rbatis/rbatis)

### 项目文档

- [用户模块 README](./users/README.md) - 用户持久化层详细说明
- [XML 映射示例](./users/user_mapper.html) - 标准XML映射文件
- [复杂查询示例](./users/xml_repository.rs) - htmlsql宏定义

### 相关技术

- [Serde 序列化](https://serde.rs/) - 数据序列化和反序列化
- [Tokio 异步运行时](https://tokio.rs/) - 异步编程框架
- [Log 日志框架](https://docs.rs/log/) - 日志记录

---

**注意**：本文档会随着项目的发展持续更新，请关注最新版本。如有问题或建议，请提交Issue或Pull Request。