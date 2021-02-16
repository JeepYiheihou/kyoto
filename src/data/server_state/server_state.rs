use crate::data::server_state::server_config::ServerConfig;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub server_config: ServerConfig,
}

impl ServerState {
    pub fn new() -> Self {
        let server_config = ServerConfig::new();
        Self { 
            server_config: server_config,
        }
    }
}