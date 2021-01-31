pub mod data_structure;

pub mod machine;

pub mod network;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
/* Result type for osaka operations.
 * This is defined as a convinience */
pub type Result<T> = std::result::Result<T, Error>;



pub fn osaka_move_to_machine(conn_handler: &mut network::conn_handler::ConnHandler) -> crate::Result<bytes::Bytes> {
    conn_handler.move_to_command_handler()
}