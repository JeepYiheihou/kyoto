#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub port: u32,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self { port: 9736 }
    }
}