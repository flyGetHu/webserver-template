use salvo::prelude::*;
use uuid::Uuid;

use crate::app::depot_keys::KEY_REQUEST_ID;
use crate::app::error::AppError;

/// 便捷函数：从 `Depot` 获取 `request_id`，若不存在则生成新的
#[must_use]
pub fn request_id_or_new(depot: &Depot) -> Uuid {
    depot
        .get::<Uuid>(KEY_REQUEST_ID)
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4())
}

/// 便捷函数：从 `Depot` 获取 `request_id`，若不存在返回错误
pub fn require_request_id(depot: &Depot) -> Result<Uuid, AppError> {
    depot
        .get::<Uuid>(KEY_REQUEST_ID)
        .cloned()
        .map_err(|_| AppError::Internal("request_id not found in depot".to_string()))
}
