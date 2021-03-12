use bytes::Bytes;

#[derive(Debug)]
pub enum ErrorType {
    InvalidSyntax,
    NonExistentKey,
}

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