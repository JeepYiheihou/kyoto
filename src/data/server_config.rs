use bytes::{ BytesMut, BufMut };

#[derive(Debug)]
pub struct ServerConfig {
    pub port: u32,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self { port: 9736 }
    }

    pub fn generate_info(&self, mut info: BytesMut) -> BytesMut {
        let port_info = format!("port: {}", self.port);
        info.put(port_info.as_bytes());
        info
    }
}