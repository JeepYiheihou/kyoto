use crate::Result;
use crate::data::replication_config::ReplicationConfig;
use crate::data::Params;
use crate::data::constants::{ DEFAULT_PORT, DEFAULT_INPUT_BUFFER_SIZE };

use bytes::{ BytesMut, BufMut };

#[derive(Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub input_buffer_size: usize,
    pub replication_config: ReplicationConfig,
}

impl ServerConfig {
    pub fn new(params: &Params) -> Self {

        let port_str = params.port.as_deref().unwrap_or(DEFAULT_PORT);
        let port = port_str.parse::<u16>().unwrap();

        let input_buffer_size_str = params.input_buffer_size.as_deref().unwrap_or(DEFAULT_INPUT_BUFFER_SIZE);
        let input_buffer_size = input_buffer_size_str.parse::<usize>().unwrap();

        let replication_config = ReplicationConfig::new();
        Self {
            port: port,
            input_buffer_size: input_buffer_size,
            replication_config: replication_config,
        }
    }

    pub fn generate_info(&self, mut info: BytesMut) -> Result<BytesMut> {
        info.put("[Server config info]\r\n".as_bytes());

        let port_info = format!("port: {}\r\n", self.port);
        info.put(port_info.as_bytes());

        let input_buffer_size_info = format!("input_buffer_size: {}\r\n", self.input_buffer_size);
        info.put(input_buffer_size_info.as_bytes());

        info.put("\r\n".as_bytes());

        Ok(info)
    }
}