use crate::data::connection::Connection;

use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    pub connection: Connection,    
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            connection: Connection::new(stream),
        }
    }
}