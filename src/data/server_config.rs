use crate::Result;
use crate::data::replication_config::ReplicationConfig;
use crate::data::Params;
use crate::data::constants::DEFAULT_PORT;

use bytes::{ BytesMut, BufMut };

#[derive(Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub replication_config: ReplicationConfig,
}

impl ServerConfig {
    pub fn new(params: &Params) -> Self {

        let port_str = params.port.as_deref().unwrap_or(DEFAULT_PORT);
        let port = port_str.parse::<u16>().unwrap();

        let replication_config = ReplicationConfig::new();
        Self {
            port: port,
            replication_config: replication_config,
        }
    }

    pub fn generate_info(&self, mut info: BytesMut) -> Result<BytesMut> {
        let port_info = format!("port: {}\r\n", self.port);
        info.put(port_info.as_bytes());
        Ok(info)
    }
}