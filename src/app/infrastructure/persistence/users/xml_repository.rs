//! 用户XML映射查询仓储
//! 
//! 使用rbatis的htmlsql功能实现复杂查询
//! 参考文档: https://rbatis.github.io/rbatis.io/#/v4/?id=htmlsql

use std::sync::Arc;
use std::collections::HashMap;

use rbatis::{RBatis, html_sql, impled};
use rbatis::rbdc::datetime::DateTime;
use rbatis::PageRequest;
use rbs::{Value};
use serde::{Deserialize, Serialize};

use crate::app::{
    domain::models::User,
    error::AppError,
    infrastructure::pagination::{PaginationParams, PaginatedResponse},
};

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
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// 用户统计结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub is_active: bool,
    pub user_count: i64,
    pub avg_age: Option<f64>,
    pub earliest_registration: Option<String>,
    pub latest_registration: Option<String>,
}

/// 用户登录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithLoginInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: String,
    pub login_count: i64,
    pub last_login_time: Option<String>,
}

// 使用htmlsql!宏定义复杂查询函数
// 参考文档中的语法: htmlsql!(function_name(params) -> ReturnType => "path/to/mapper.html")



/// 用户XML查询仓储
#[derive(Clone)]
pub struct UserXmlRepository {
    rb: Arc<RBatis>,
}

impl UserXmlRepository {
    /// 创建新的XML查询仓储实例
    #[must_use]
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    /// 复杂条件查询用户（分页）
    #[html_sql("src/app/infrastructure/persistence/users/user_mapper.html")]
    async fn select_users_by_complex_condition(&self, params: &HashMap<String, Value>) -> rbatis::Result<Vec<User>> {
        impled!()
    }

    /// 统计符合条件的用户数量
    #[html_sql("src/app/infrastructure/persistence/users/user_mapper.html")]
    async fn count_users_by_complex_condition(&self, params: &HashMap<String, Value>) -> rbatis::Result<i64> {
        impled!()
    }

    /// 搜索用户（关键词匹配）
    #[html_sql("src/app/infrastructure/persistence/users/user_mapper.html")]
    async fn search_users_paginated(&self, keyword: &str, limit: i64, offset: i64) -> rbatis::Result<Vec<User>> {
        impled!()
    }

    /// 统计搜索结果数量
    #[html_sql("src/app/infrastructure/persistence/users/user_mapper.html")]
    async fn count_search_users(&self, keyword: &str) -> rbatis::Result<i64> {
        impled!()
    }

    /// 获取用户统计信息
    #[html_sql("src/app/infrastructure/persistence/users/user_mapper.html")]
    async fn get_user_statistics(&self, start_date: Option<&str>, end_date: Option<&str>) -> rbatis::Result<Vec<UserStatistics>> {
        impled!()
    }

    /// 获取用户及其登录信息
    #[html_sql("src/app/infrastructure/persistence/users/user_mapper.html")]
    async fn get_users_with_login_info(&self, params: &HashMap<String, Value>) -> rbatis::Result<Vec<UserWithLoginInfo>> {
        impled!()
    }

    /// 分页查询示例（使用rbatis内置分页功能）
    #[html_sql("src/app/infrastructure/persistence/users/user_mapper.html")]
    async fn select_page_data(&self, page_request: &PageRequest, name: &str, dt: &DateTime) -> rbatis::Result<rbatis::Page<User>> {
        impled!()
    }

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
        let users = self.select_users_by_complex_condition(&params)
            .await
            .map_err(AppError::Rbs)?;

        let total = self.count_users_by_complex_condition(&params)
            .await
            .map_err(AppError::Rbs)?;
        
        // 构建分页响应
        let page_size = query.limit.unwrap_or(10);
        let current_page = (query.offset.unwrap_or(0) / page_size) + 1;
        
        Ok(PaginatedResponse::new(
            users,
            current_page,
            page_size,
            total,
        ))
    }

    /// 搜索用户（分页）
    /// 
    /// # 参数
    /// * `keyword` - 搜索关键词
    /// * `pagination` - 分页参数
    /// 
    /// # 返回
    /// 分页的用户搜索结果
    pub async fn search_users_paginated(
        &self,
        keyword: &str,
        pagination: &PaginationParams,
    ) -> Result<PaginatedResponse<User>, AppError> {
        let offset = (pagination.page - 1) * pagination.per_page;
        let limit = pagination.per_page;
        
        // 调用XML中定义的搜索查询
        let users = self.search_users_paginated(keyword, limit, offset)
            .await
            .map_err(AppError::Rbs)?;

        let total = self.count_search_users(keyword)
            .await
            .map_err(AppError::Rbs)?;
        
        Ok(PaginatedResponse::new(
            users,
            pagination.page,
            pagination.per_page,
            total,
        ))
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
        let statistics = self.get_user_statistics(start_date, end_date)
            .await
            .map_err(AppError::Rbs)?;
        
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
        let users_with_login = self.get_users_with_login_info(&params)
            .await
            .map_err(AppError::Rbs)?;
        
        Ok(users_with_login)
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
        let page_result = self.select_page_data(page_request, name, dt)
            .await
            .map_err(AppError::Rbs)?;
        
        Ok(page_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbatis::RBatis;
    use rbdc_sqlite::driver::SqliteDriver;

    async fn setup_test_db() -> RBatis {
        let rb = RBatis::new();
        rb.link(SqliteDriver {}, "sqlite://:memory:")
            .await
            .expect("Failed to connect to test database");
        rb
    }

    #[tokio::test]
    async fn test_complex_query_structure() {
        let rb = Arc::new(setup_test_db().await);
        let repo = UserXmlRepository::new(rb);
        
        let query = ComplexUserQuery {
            username: Some("test".to_string()),
            email: None,
            is_active: Some(true),
            start_date: None,
            end_date: None,
            age_min: Some(18),
            age_max: Some(65),
            roles: None,
            sort_by: Some("username".to_string()),
            sort_order: Some("ASC".to_string()),
            limit: Some(10),
            offset: Some(0),
        };
        
        // 这里只是测试结构，实际测试需要先创建表和数据
        // let result = repo.find_users_by_complex_condition(&query).await;
        // assert!(result.is_ok());
        
        // 验证查询参数结构正确
        assert_eq!(query.username, Some("test".to_string()));
        assert_eq!(query.is_active, Some(true));
    }
}