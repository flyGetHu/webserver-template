use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tower_http::request_id::{MakeRequestUuid, RequestId};
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

#[derive(Clone)]
struct MakeRequestUuidV4;

impl MakeRequestUuid for MakeRequestUuidV4 {
    fn make_request_id(&mut self, _request: &Request) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();
        Some(RequestId::new(
            request_id.parse().expect("generated UUID is not a valid header value"),
        ))
    }
}

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
