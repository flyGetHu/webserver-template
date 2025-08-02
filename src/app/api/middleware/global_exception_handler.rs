use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::app::error::{ApiError, AppError};

pub async fn global_exception_handler(req: Request, next: Next) -> Response {
    let request_id = *req.extensions().get::<Uuid>().unwrap();
    let mut res = next.run(req).await;

    if let Some(err) = res.extensions_mut().remove::<AppError>() {
        let (status_code, api_error) = err.to_api_error(request_id);
        return (status_code, Json(api_error)).into_response();
    }

    res
}
