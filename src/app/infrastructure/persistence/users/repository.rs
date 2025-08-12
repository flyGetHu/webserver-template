//! 用户数据持久化仓储
//! 
//! 提供用户的所有数据库操作，包括基础CRUD和复杂查询
//! 使用 rbatis htmlsql 实现复杂查询功能

use std::sync::Arc;
use std::collections::HashMap;

use rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::PageRequest;
use rbs::{value, Value};
use serde::{Deserialize, Serialize};

use crate::app::{
    domain::models::{CreateUserDto, User},
    error::AppError,
    infrastructure::pagination::{PaginationParams, PaginatedResponse},
};

// 导入内部xml_repository模块的类型
use super::xml_repository::{
    ComplexUserQuery, UserStatistics, UserWithLoginInfo, UserXmlRepository,
};

/// 用户数据持久化仓储
/// 
/// 提供所有用户相关的数据库操作，包括基础CRUD和复杂查询
/// 内部使用rbatis的derive宏和htmlsql功能
#[derive(Clone)]
pub struct UserRepository {
    rb: Arc<RBatis>,
    xml_repo: UserXmlRepository,
}

impl UserRepository {
    /// 创建新的用户仓储实例
    #[must_use]
    pub fn new(rb: Arc<RBatis>) -> Self {
        let xml_repo = UserXmlRepository::new(rb.clone());
        Self { rb, xml_repo }
    }

    /// 创建新用户
    /// # Errors
    /// 可能返回数据库错误、序列化错误或范围转换错误
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
            value!(serde_json::to_string(&vec!["user".to_string()])
                .map_err(|e| AppError::Internal(format!("serialize roles json failed: {e}")))?),
            value!(true),
        ];
        let exec = self.rb.exec(sql, args).await.map_err(AppError::Database)?;
        let user_id: i64 = exec.last_insert_id.into();
        let user_id = i32::try_from(user_id)
            .map_err(|_| AppError::Internal("last_insert_id out of range".to_string()))?;
        self.find_by_id(user_id).await?.ok_or(AppError::NotFound("User not found".to_string()))
    }

    /// 通过ID查找用户 - 使用RBatis原生语法（简单查询）
    /// # Errors
    /// 数据库错误
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode("SELECT * FROM users WHERE id = ?", vec![value!(id)])
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 通过用户名查找用户 - 使用RBatis原生语法（简单查询）
    /// # Errors
    /// 数据库错误
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode("SELECT * FROM users WHERE username = ?", vec![value!(username)])
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 通过邮箱查找用户 - 使用RBatis原生语法（简单查询）
    /// # Errors
    /// 数据库错误
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode("SELECT * FROM users WHERE email = ?", vec![value!(email)])
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 通过用户名或邮箱查找用户
    /// # Errors
    /// 数据库错误
    pub async fn find_by_username_or_email(&self, username_or_email: &str) -> Result<Option<User>, AppError> {
        let users: Vec<User> = self
            .rb
            .query_decode(
                "SELECT * FROM users WHERE username = ? OR email = ?",
                vec![value!(username_or_email), value!(username_or_email)],
            )
            .await
            .map_err(AppError::Database)?;
        Ok(users.into_iter().next())
    }

    /// 检查用户名是否存在 - 使用RBatis原生语法（简单查询）
    /// # Errors
    /// 数据库错误
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

    /// 检查邮箱是否存在 - 使用RBatis原生语法（简单查询）
    /// # Errors
    /// 数据库错误
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let rows: Vec<i64> = self
            .rb
            .query_decode(
                "SELECT EXISTS(SELECT 1 FROM users WHERE email = ?)",
                vec![value!(email)],
            )
            .await
            .map_err(AppError::Database)?;
        Ok(rows.into_iter().next().unwrap_or(0) != 0)
    }

    /// 获取所有用户（分页）
    /// # Errors
    /// 数据库错误
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

    /// 获取所有用户（标准化分页）
    /// # Errors
    /// 数据库错误
    pub async fn find_all_paginated(&self, params: PaginationParams) -> Result<PaginatedResponse<User>, AppError> {
        let total = self.count_all().await?;
        let offset = params.offset();
        let data = self.find_all(params.per_page, offset).await?;
        
        Ok(PaginatedResponse::new(
            data,
            params.page,
            params.per_page,
            total,
        ))
    }

    /// 获取用户总数 - 使用RBatis原生语法（简单查询）
    /// # Errors
    /// 数据库错误
    pub async fn count_all(&self) -> Result<i64, AppError> {
        let counts: Vec<i64> = self
            .rb
            .query_decode("SELECT COUNT(*) FROM users", vec![])
            .await
            .map_err(AppError::Database)?;
        Ok(counts.into_iter().next().unwrap_or(0))
    }

    /// 简单分页查询所有用户
    /// 复杂查询请使用本类中的其他方法（如 search_users_paginated）
    /// # Errors
    /// 数据库错误
    pub async fn find_users_paginated(
        &self,
        params: &PaginationParams,
    ) -> Result<PaginatedResponse<User>, AppError> {
        let mut params = params.clone();
        params.validate();
        
        let users = self.find_all(params.per_page, params.offset()).await?;
        let total = self.count_all().await?;
        
        Ok(PaginatedResponse::new(users, params.page, params.per_page, total))
    }

    // 复杂查询方法已合并到本类中
    // 该 Repository 现在包含简单和复杂的查询操作


    /// 更新用户状态
    /// # Errors
    /// 数据库错误或未找到
    pub async fn update_user_status(&self, id: i32, is_active: bool) -> Result<User, AppError> {
        self
            .rb
            .exec(
                "UPDATE users SET is_active = ? WHERE id = ?",
                vec![value!(is_active), value!(id)],
            )
            .await
            .map_err(AppError::Database)?;

        self.find_by_id(id).await?.ok_or(AppError::NotFound("User not found".to_string()))
    }

    /// 删除用户
    /// # Errors
    /// 数据库错误
    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        self
            .rb
            .exec("DELETE FROM users WHERE id = ?", vec![value!(id)])
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    // === 以下方法来自原 UserXmlRepository，用于复杂查询 ===

    /// 根据状态查找用户
    /// # Errors
    /// 数据库错误
    pub async fn find_users_by_status(&self, is_active: bool) -> Result<Vec<User>, AppError> {
        let mut _params = HashMap::new();
        _params.insert("is_active".to_string(), rbs::Value::Bool(is_active));

        self.rb.query_decode("SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users WHERE is_active = ? ORDER BY created_at DESC", vec![rbs::Value::Bool(is_active)])
            .await
            .map_err(AppError::Database)
    }

    /// 动态条件分页查询用户
    /// # Errors
    /// 数据库错误
    pub async fn find_users_by_condition_paginated(
        &self,
        pagination: &PaginationParams,
        _query_params: &HashMap<String, Value>,
    ) -> Result<PaginatedResponse<User>, AppError> {
        let mut params = pagination.clone();
        params.validate();

        // 构建查询参数
        let mut _search_params = _query_params.clone();
        _search_params.insert("limit".to_string(), rbs::value!(params.per_page));
        _search_params.insert("offset".to_string(), rbs::value!(params.offset()));

        // 获取总数
        let total: i64 = self
            .rb
            .query_decode("SELECT COUNT(*) FROM users", vec![])
            .await
            .map_err(AppError::Database)?;

        // 获取数据
        let data: Vec<User> = self.rb.query_decode(
            "SELECT id, username, email, password_hash, is_active, created_at, updated_at FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?",
            vec![rbs::Value::I32(pagination.per_page as i32), rbs::Value::I32(((pagination.page - 1) * pagination.per_page) as i32)]
        )
        .await
        .map_err(AppError::Database)?;

        Ok(PaginatedResponse::new(
            data,
            pagination.page,
            pagination.per_page,
            total,
        ))
    }

    /// 搜索用户（分页）
    /// 使用 XML 映射进行模糊搜索
    pub async fn search_users_paginated(
        &self,
        keyword: &str,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<User>, AppError> {
        let offset = (pagination.page - 1) * pagination.per_page;
        let limit = pagination.per_page;
        
        // 调用XML中定义的搜索查询
        self.xml_repo.search_users_paginated_public(keyword, pagination).await
    }

    // === 复杂查询方法（使用 rbatis htmlsql） ===

    /// 复杂条件查询用户（分页）
    /// 
    /// # 参数
    /// * `query` - 复杂查询条件
    /// 
    /// # 返回
    /// 分页的用户列表
    pub async fn find_users_by_complex_condition(
        &self,
        query: &ComplexUserQuery,
    ) -> Result<PaginatedResponse<User>, AppError> {
        // 准备查询参数
        let mut params = HashMap::new();
        
        // 设置可选参数
        if let Some(username) = &query.username {
            params.insert("username".to_string(), Value::String(username.clone()));
        }
        if let Some(email) = &query.email {
            params.insert("email".to_string(), Value::String(email.clone()));
        }
        if let Some(is_active) = query.is_active {
            params.insert("is_active".to_string(), Value::Bool(is_active));
        }
        if let Some(start_date) = &query.start_date {
            params.insert("start_date".to_string(), Value::String(start_date.clone()));
        }
        if let Some(end_date) = &query.end_date {
            params.insert("end_date".to_string(), Value::String(end_date.clone()));
        }
        if let Some(age_min) = query.age_min {
            params.insert("age_min".to_string(), Value::I32(age_min));
        }
        if let Some(age_max) = query.age_max {
            params.insert("age_max".to_string(), Value::I32(age_max));
        }
        if let Some(roles) = &query.roles {
            let role_values: Vec<Value> = roles.iter().map(|r| Value::String(r.clone())).collect();
            params.insert("roles".to_string(), Value::Array(role_values));
        }
        
        // 设置排序和分页参数
        params.insert("sort_by".to_string(), Value::String(query.sort_by.clone().unwrap_or("id".to_string())));
        params.insert("sort_order".to_string(), Value::String(query.sort_order.clone().unwrap_or("ASC".to_string())));
        params.insert("limit".to_string(), Value::I64(query.limit.unwrap_or(10)));
        params.insert("offset".to_string(), Value::I64(query.offset.unwrap_or(0)));
        
        // 调用XML中定义的复杂查询
        self.xml_repo.find_users_by_complex_condition(query).await
    }

    /// 获取用户统计信息
    /// 
    /// # 参数
    /// * `start_date` - 开始日期（可选）
    /// * `end_date` - 结束日期（可选）
    /// 
    /// # 返回
    /// 用户统计信息列表
    pub async fn get_user_statistics(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Vec<UserStatistics>, AppError> {
        let statistics = self.xml_repo.get_user_statistics_public(start_date, end_date)
            .await?;
        
        Ok(statistics)
    }

    /// 获取用户及其登录信息
    /// 
    /// # 参数
    /// * `is_active` - 用户状态（可选）
    /// * `login_start_date` - 登录开始时间（可选）
    /// * `login_end_date` - 登录结束时间（可选）
    /// * `min_login_count` - 最小登录次数（可选）
    /// * `limit` - 限制数量（可选）
    /// * `offset` - 偏移量（可选）
    /// 
    /// # 返回
    /// 用户及其登录信息列表
    pub async fn get_users_with_login_info(
        &self,
        is_active: Option<bool>,
        login_start_date: Option<&str>,
        login_end_date: Option<&str>,
        min_login_count: Option<i64>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<UserWithLoginInfo>, AppError> {
        // 准备查询参数
        let mut params = HashMap::new();
        
        if let Some(active) = is_active {
            params.insert("is_active".to_string(), Value::Bool(active));
        }
        if let Some(start_date) = login_start_date {
            params.insert("login_start_date".to_string(), Value::String(start_date.to_string()));
        }
        if let Some(end_date) = login_end_date {
            params.insert("login_end_date".to_string(), Value::String(end_date.to_string()));
        }
        if let Some(min_count) = min_login_count {
            params.insert("min_login_count".to_string(), Value::I64(min_count));
        }
        
        params.insert("limit".to_string(), Value::I64(limit.unwrap_or(50)));
        params.insert("offset".to_string(), Value::I64(offset.unwrap_or(0)));
        
        // 调用XML中定义的连表查询
        self.xml_repo.get_users_with_login_info_public(
            is_active,
            login_start_date,
            login_end_date,
            min_login_count,
            limit,
            offset,
        ).await
    }

    /// 使用rbatis内置分页功能的示例
    /// 
    /// # 参数
    /// * `page_request` - 分页请求
    /// * `name` - 用户名
    /// * `dt` - 日期时间
    /// 
    /// # 返回
    /// 分页结果
    pub async fn select_page_example(
        &self,
        page_request: &PageRequest,
        name: &str,
        dt: &DateTime,
    ) -> Result<rbatis::Page<User>, AppError> {
        self.xml_repo.select_page_example(page_request, name, dt).await
    }
}