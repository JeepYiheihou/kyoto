use crate::data::replication_config::ReplicationConfig;

use bytes::{ BytesMut, BufMut };

#[derive(Debug)]
pub struct ServerConfig {
    pub port: u32,
    pub replication_config: ReplicationConfig,
}

impl ServerConfig {
    pub fn new() -> Self {
        let replication_config = ReplicationConfig::new();
        Self {
            port: 9736,
            replication_config: replication_config,
        }
    }

    pub fn generate_info(&self, mut info: BytesMut) -> BytesMut {
        let port_info = format!("port: {}", self.port);
        info.put(port_info.as_bytes());
        info
    }
}