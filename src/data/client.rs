use crate::data::connection::Connection;

use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Client {
    pub connection: Mutex<Connection>,    
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            connection: Mutex::new(Connection::new(stream)),
        }
    }
}