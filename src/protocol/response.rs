use bytes::Bytes;

#[derive(Debug)]
pub enum ErrorType {
    InvalidSyntax,
    NonExistentKey,
}

#[derive(Debug)]
pub enum Response {
    /* Some valid response that needs to be sent back to client. */
    Valid {
        message: Bytes,
    },
    /* Some error message that needs to be sent back to client. */
    Error {
        error_type: ErrorType,
        message: String,
    },
    /* Empty reponse, meaning it shouldn't be sent back to client. */
    None,
}