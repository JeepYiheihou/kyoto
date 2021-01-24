use crate::Result;

use bytes::{ BytesMut };
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufWriter };
use tokio::net::{ TcpListener, TcpStream };
use tracing::{ error };

#[derive(Debug)]
pub struct ConnHandler {
    socket: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl ConnHandler {
    pub fn new(stream: TcpStream) -> Self {
        let socket = BufWriter::new(stream);
        let buffer = BytesMut::with_capacity(4 * 1024);
        ConnHandler { socket: socket,
                      buffer: buffer }
        
    }

    pub async fn handle(&mut self) -> Result<()> {
        loop {
            let res = self.socket.read_buf(&mut self.buffer).await?;
            if res == 0 {
                if self.buffer.is_empty() {
                    return Ok(());
                } else {
                    return Err("connection reset by peer".into());
                }
            };
            self.socket.write(&self.buffer[..res]).await?;
            self.socket.flush().await?;
            self.buffer.clear();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        Server { port: port }
    }

    #[tokio::main]
    pub async fn run(&mut self) -> Result<()> {
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", self.port)).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let mut conn_handler = ConnHandler::new(stream);
                    tokio::spawn(async move {
                        if let Err(err) = conn_handler.handle().await {
                            error!(cause = ?err, "connection error");
                        }
                    });
                },
                Err(err) => {
                    return Err(err.into());
                },
            }
        }
    }
}
