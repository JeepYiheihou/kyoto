use bytes::BytesMut;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Connection {
    pub socket: TcpStream,
    pub buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let buffer = BytesMut::with_capacity(4 * 1024);
        Self {
            socket: stream,
            buffer: buffer,
        }
    }
}