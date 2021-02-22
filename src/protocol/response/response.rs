use crate::protocol::response::error_type::ErrorType;

use bytes::Bytes;

#[derive(Debug)]
pub enum Response {
    Valid {
        message: Bytes,
    },
    Error {
        error_type: ErrorType,
        message: Bytes,
    },
}