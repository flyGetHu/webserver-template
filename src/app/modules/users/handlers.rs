//! 用户处理器
//!
//! 处理用户管理相关的HTTP请求

use salvo::prelude::*;
use uuid::Uuid;

use crate::app::{
    api::response::ApiResponse,
    api::util::request_id_or_new,
    error::AppError,
    modules::users::models::{CreateUserRequest, UpdateUserRequest, UserListResponse, UserResponse},
};

/// 获取用户列表处理器
#[endpoint(
    tags("Users"),
    operation_id = "list_users",
    parameters(
        ("page" = i32, Query, description = "Page number", example = 1),
        ("page_size" = i32, Query, description = "Page size", example = 10)
    ),
    responses(
        (status_code = 200, description = "Users retrieved successfully", body = UserListResponse),
        (status_code = 400, description = "Invalid query parameters")
    )
)]
pub async fn list_users(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = request_id_or_new(depot);

    let page = req.query::<i32>("page").unwrap_or(1);
    let page_size = req.query::<i32>("page_size").unwrap_or(10);

    // 临时实现 - 返回模拟用户列表
    let response = UserListResponse {
        users: vec![],
        total: 0,
        page,
        page_size,
    };

    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}

/// 根据ID获取用户处理器
#[endpoint(
    tags("Users"),
    operation_id = "get_user",
    parameters(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status_code = 200, description = "User retrieved successfully", body = UserResponse),
        (status_code = 404, description = "User not found")
    )
)]
pub async fn get_user(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = request_id_or_new(depot);

    let id = req
        .param::<i32>("id")
        .ok_or_else(|| AppError::Validation("Invalid user ID".to_string()))?;

    // 临时实现 - 返回模拟用户
    let response = UserResponse {
        id,
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        age: Some(25),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
    };

    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}

/// 创建用户处理器
#[endpoint(
    tags("Users"),
    operation_id = "create_user",
    request_body = CreateUserRequest,
    responses(
        (status_code = 201, description = "User created successfully", body = UserResponse),
        (status_code = 400, description = "Invalid request data"),
        (status_code = 409, description = "User already exists")
    )
)]
pub async fn create_user(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = request_id_or_new(depot);

    let payload = req
        .parse_json::<CreateUserRequest>()
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // 临时实现 - 返回模拟创建的用户
    let response = UserResponse {
        id: 1,
        username: payload.username,
        email: payload.email,
        age: payload.age,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
    };

    res.status_code(StatusCode::CREATED);
    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}

/// 更新用户处理器
#[endpoint(
    tags("Users"),
    operation_id = "update_user",
    parameters(
        ("id" = i32, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status_code = 200, description = "User updated successfully", body = UserResponse),
        (status_code = 400, description = "Invalid request data"),
        (status_code = 404, description = "User not found")
    )
)]
pub async fn update_user(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = request_id_or_new(depot);

    let id = req
        .param::<i32>("id")
        .ok_or_else(|| AppError::Validation("Invalid user ID".to_string()))?;

    let payload = req
        .parse_json::<UpdateUserRequest>()
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // 临时实现 - 返回模拟更新的用户
    let response = UserResponse {
        id,
        username: payload
            .username
            .unwrap_or_else(|| "updated_user".to_string()),
        email: payload
            .email
            .unwrap_or_else(|| "updated@example.com".to_string()),
        age: payload.age,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
    };

    res.render(Json(ApiResponse::new(response, request_id)));
    Ok(())
}

/// 删除用户处理器
#[endpoint(
    tags("Users"),
    operation_id = "delete_user",
    parameters(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status_code = 204, description = "User deleted successfully"),
        (status_code = 404, description = "User not found")
    )
)]
pub async fn delete_user(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), AppError> {
    let request_id = depot
        .get::<Uuid>("request_id")
        .cloned()
        .unwrap_or_else(|_| Uuid::new_v4());

    let _id = req
        .param::<i32>("id")
        .ok_or_else(|| AppError::Validation("Invalid user ID".to_string()))?;

    // 临时实现 - 模拟删除用户
    // 在实际实现中，这里会调用用户服务删除用户

    res.status_code(StatusCode::NO_CONTENT);
    res.render(Json(ApiResponse::new((), request_id)));
    Ok(())
}
