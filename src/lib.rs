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

use machine::MachineHandler;
use data::Server;
use protocol::{ FlowType, RetFlowType };

/* kyoto Error type. */
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/* Result type for kyoto operations.
 * This is defined as a convinience */
pub type Result<T> = std::result::Result<T, Error>;

/* Flow moving from network stage to machine stage. */
pub fn kyoto_network_to_machine(machine_handler: &mut MachineHandler,
                                flow: FlowType) -> crate::Result<RetFlowType> {
    machine_handler.handle_flow(flow)
}

/* Flow moving from machine stage to warehouse stage. */
pub fn kyoto_machine_to_warehouse(server: &mut Server,
                                  flow: FlowType) -> crate::Result<RetFlowType> {
    server.handle_flow(flow)
}