pub mod request_id_format;

#[derive(Debug, Clone)]
pub struct RequestId(pub uuid::Uuid);
