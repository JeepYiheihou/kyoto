/* The Arc layer for server so that it can be shared between theads. */
mod shared;

/* Server configurations and status. */
mod server_state;

/* Data structures supported by osaka. */
mod warehouse;

/* Expose the Server struct. */
pub type Server = shared::server::Server;

/* Expose the CommandExecutor struct. */
pub type CommandExecutor = warehouse::command_executor::CommandExecutor;