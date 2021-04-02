/* Constants mod for constant values. */
pub mod constants;

/* Server struct for all shared data. */
pub mod server;

/* Params struct for parameters to be fed to server. */
pub mod params;

/* Client struct for all client specific data. */
pub mod client;

/* Connection struct for all connection specific data. */
pub mod connection;

/* Server configurations and status. */
pub mod db;

/* Data structures supported by kyoto. */
pub mod server_config;

/* ReplicationConfig structure for replication data. */
pub mod replication_config;

/* Expose Server struct. */
pub type Server = server::Server;

/* Expose Params struct. */
pub type Params = params::Params;

/* Expose ClientType enum. */
pub type ClientType = client::ClientType;

/* Expose ClientCollections enum */
pub type ClientCollections = client::ClientCollections;

/* Expose Client struct. */
pub type Client = client::Client;

/* Expose the Server struct. */
pub type Db = db::Db;

/* Expose the ServerConfig struct. */
pub type ServerConfig = server_config::ServerConfig;