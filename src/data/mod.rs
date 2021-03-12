/* Server configurations and status. */
mod db;

/* Data structures supported by kyoto. */
mod server_config;

/* Expose the Server struct. */
pub type Db = db::Db;

/* Expose the ServerConfig struct. */
pub type ServerConfig = server_config::ServerConfig;