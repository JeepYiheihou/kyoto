use crate::Result;

use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct Connection {
    pub socket: TcpStream,
    pub buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream, input_buffer_size: usize) -> Self {
        let buffer = BytesMut::with_capacity(input_buffer_size);
        Self {
            socket: stream,
            buffer: buffer,
        }
    }

    pub async fn read_to_buf(&mut self) -> Result<usize> {
        let count = self.socket.read_buf(&mut self.buffer).await?;
        Ok(count)
    }
}