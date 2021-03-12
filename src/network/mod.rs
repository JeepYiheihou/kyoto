/* Server struct for all shared data. */
pub mod server;

/* Client struct for all client specific data. */
pub mod client;

/* Connection struct for all connection specific data. */
pub mod connection;

/* Listener to accept incoming connections. */
pub mod listener;

/* Network module for network stage functions. */
pub mod socket_io;

/* Expose Server struct. */
pub type Server = server::Server;

/* Expose Client struct. */
pub type Client = client::Client;

/* Expose Listener struct. */
pub type Listener = listener::Listener;