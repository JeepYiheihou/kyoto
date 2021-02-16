/* Network module.
 * This is the module to handle network IO. */
pub mod network;

/* Machine module.
 * This is the module to handle machine operations.
 * e.g. command parsing */
pub mod machine;

/* Data module.
 * This is the module for data structures.
 * Commands are actually executed here. Memory IO and disk IO. */
pub mod data;

/* Protocol module.
 * This is the module for command protocol utils.
 * Command table, command parser, etc. This is not a module for any
 * concept "stage", but used in stages. */
pub mod protocol;


use bytes::Bytes;
use network::NetworkHandler;
use protocol::Command;
use data::CommandExecutor;

/* Osaka Error type. */
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/* Result type for osaka operations.
 * This is defined as a convinience */
pub type Result<T> = std::result::Result<T, Error>;

/* Flow to move from network stage to machine stage. */
pub fn osaka_network_to_machine(conn_handler: &mut NetworkHandler) -> crate::Result<Option<Bytes>> {
    conn_handler.move_to_command_handler()
}

/* Flow to move from machine stage to warehouse stage. */
pub fn osaka_machine_to_warehouse(cmd: Command, server: &mut data::Server) -> crate::Result<Bytes> {
    CommandExecutor::execute_command(cmd, server)
}