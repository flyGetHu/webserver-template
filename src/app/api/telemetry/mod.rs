pub mod request_id_format;
pub use request_id_format::RequestIdFormat;

#[derive(Debug, Clone)]
pub struct RequestId(pub uuid::Uuid);
