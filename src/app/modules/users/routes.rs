//! 用户模块路由定义
//!
//! 使用Salvo的树状路由结构组织用户管理相关的端点

use salvo::prelude::*;

use super::handlers::{create_user, delete_user, get_user, list_users, update_user};

/// 创建用户模块的路由
///
/// 使用树状结构组织用户管理相关的端点：
/// - GET /users - 获取用户列表
/// - POST /users - 创建用户
/// - GET /users/{id} - 根据ID获取用户
/// - PATCH /users/{id} - 更新用户
/// - DELETE /users/{id} - 删除用户
#[must_use]
pub fn create_routes() -> Router {
    Router::with_path("users")
        // TODO: 添加认证中间件到需要认证的端点
        // .hoop(auth_middleware)
        .get(list_users)
        .post(create_user)
        .push(
            Router::with_path("{id}")
                .get(get_user)
                .patch(update_user)
                .delete(delete_user),
        )
}
