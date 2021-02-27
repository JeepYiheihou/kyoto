/* The Arc layer for server so that it can be shared between theads. */
mod shared;

/* Server configurations and status. */
mod server_state;

/* Data structures supported by kyoto. */
mod warehouse;

/* Expose the Server struct. */
pub type Server = shared::server::Server;