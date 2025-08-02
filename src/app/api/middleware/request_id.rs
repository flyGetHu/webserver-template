use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn add_request_id(
    mut req: Request,
    next: Next,
) -> Response {
    let id = Uuid::new_v4();
    req.extensions_mut().insert(id);
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert(REQUEST_ID_HEADER, id.to_string().parse().unwrap());
    res
}
