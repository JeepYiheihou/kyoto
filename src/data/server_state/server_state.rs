use crate::data::server_state::server_config::ServerConfig;

use bytes::Bytes;
use std::sync::Mutex;

#[derive(Debug)]
pub struct ServerState {
    pub server_config: Mutex<ServerConfig>,
}

impl ServerState {
    pub fn new() -> Self {
        let server_config_mutex = Mutex::new(ServerConfig::new());
        Self { 
            server_config: server_config_mutex,
        }
    }

    pub fn get_info(&self) -> Option<Bytes> {
        let config = self.server_config.lock().unwrap();
        let info = format!("Port: {}", config.port);
        Some(Bytes::from(info))
    }

    pub fn get_port(&self) -> u32 {
        let config = self.server_config.lock().unwrap();
        config.port
    }
}