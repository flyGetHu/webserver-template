//! 分页查询支持模块

use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;

/// 分页请求参数
#[derive(Debug, Deserialize, Clone, Default)]
pub struct PaginationParams {
    /// 页码，从1开始
    #[serde(default = "default_page")]
    pub page: i64,
    /// 每页数量
    #[serde(default = "default_per_page")]
    pub per_page: i64,
    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// 排序字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// 排序方向 (asc/desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// 过滤条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<serde_json::Value>,
}

/// 默认页码
fn default_page() -> i64 {
    1
}

/// 默认每页数量
fn default_per_page() -> i64 {
    20
}

impl PaginationParams {
    /// 计算偏移量
    #[must_use]
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }

    /// 计算下一页
    #[must_use]
    pub fn next_page(&self) -> i64 {
        self.page + 1
    }

    /// 计算上一页
    #[must_use]
    pub fn prev_page(&self) -> Option<i64> {
        if self.page > 1 {
            Some(self.page - 1)
        } else {
            None
        }
    }

    /// 验证并修正分页参数
    pub fn validate(&mut self) {
        if self.page < 1 {
            self.page = 1;
        }
        if self.per_page < 1 {
            self.per_page = 20;
        }
        if self.per_page > 100 {
            self.per_page = 100;
        }
    }

    /// 获取排序字段
    #[must_use]
    pub fn sort_field(&self) -> String {
        self.sort_by.clone().unwrap_or_else(|| "id".to_string())
    }

    /// 获取排序方向
    #[must_use]
    pub fn sort_direction(&self) -> String {
        let order = self.sort_order.as_deref().unwrap_or("desc");
        match order.to_lowercase().as_str() {
            "asc" => "ASC",
            _ => "DESC",
        }.to_string()
    }

    /// 构建排序SQL
    #[must_use]
    pub fn build_order_by(&self) -> String {
        format!("{} {}", self.sort_field(), self.sort_direction())
    }
}

/// 分页响应数据
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct PaginatedResponse<T> {
    /// 数据列表
    pub data: Vec<T>,
    /// 分页信息
    pub pagination: PaginationInfo,
}

/// 分页信息
#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct PaginationInfo {
    /// 当前页码
    pub current_page: i64,
    /// 每页数量
    pub per_page: i64,
    /// 总记录数
    pub total: i64,
    /// 总页数
    pub total_pages: i64,
    /// 是否有下一页
    pub has_next: bool,
    /// 是否有上一页
    pub has_prev: bool,
    /// 下一页页码
    pub next_page: Option<i64>,
    /// 上一页页码
    pub prev_page: Option<i64>,
}

impl PaginationInfo {
    /// 创建分页信息
    #[must_use]
    pub fn new(current_page: i64, per_page: i64, total: i64) -> Self {
        let total_pages = if total == 0 {
            1
        } else {
            (total as f64 / per_page as f64).ceil() as i64
        };

        let has_next = current_page < total_pages;
        let has_prev = current_page > 1;
        let next_page = if has_next { Some(current_page + 1) } else { None };
        let prev_page = if has_prev { Some(current_page - 1) } else { None };

        Self {
            current_page,
            per_page,
            total,
            total_pages,
            has_next,
            has_prev,
            next_page,
            prev_page,
        }
    }
}

impl<T> PaginatedResponse<T> {
    /// 创建分页响应
    #[must_use]
    pub fn new(data: Vec<T>, current_page: i64, per_page: i64, total: i64) -> Self {
        Self {
            data,
            pagination: PaginationInfo::new(current_page, per_page, total),
        }
    }
}

/// 分页查询 trait
#[async_trait::async_trait]
pub trait PaginateQuery<T>: Send + Sync {
    /// 获取总记录数
    async fn count(&self) -> Result<i64, crate::app::error::AppError>;

    /// 获取分页数据
    async fn fetch_page(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<T>, crate::app::error::AppError>;

    /// 执行分页查询
    async fn paginate(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<T>, crate::app::error::AppError> {
        let mut params = params;
        params.validate();

        let total = self.count().await?;
        let offset = params.offset();
        let data = self.fetch_page(offset, params.per_page).await?;

        Ok(PaginatedResponse::new(
            data,
            params.page,
            params.per_page,
            total,
        ))
    }
}

/// 可分页的查询构建器 trait
pub trait PageableQuery {
    /// 添加分页限制
    fn with_pagination(self, params: &PaginationParams) -> Self;
}