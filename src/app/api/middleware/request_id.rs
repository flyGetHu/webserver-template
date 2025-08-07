use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{info_span, Instrument};
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn add_request_id(mut req: Request, next: Next) -> Response {
    let id = Uuid::new_v4();
    req.extensions_mut().insert(id);

    let span = info_span!("request", request_id = %id.as_hyphenated().to_string().as_str());
    // 移除了对span.extensions_mut()的调用

    let mut res = next.run(req).instrument(span).await;
    res.headers_mut()
        .insert(REQUEST_ID_HEADER, id.to_string().parse().unwrap());
    res
}
