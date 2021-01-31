use bytes::{ Bytes, BytesMut };
use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufWriter };

use crate::Result;
use crate::data_structure::db::Db;
use crate::machine::command_handler::CommandHandler;

#[derive(Debug)]
pub struct ConnHandler {
    socket: BufWriter<TcpStream>,
    buffer: BytesMut,
    command_handler: CommandHandler,
}

impl ConnHandler {
    pub fn new(stream: TcpStream, db: Db) -> Self {
        let socket = BufWriter::new(stream);
        let buffer = BytesMut::with_capacity(4 * 1024);
        let command_handler = CommandHandler::new(db);
        ConnHandler { socket: socket,
                      buffer: buffer,
                      command_handler: command_handler,
                    }
        
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
            
            let res = crate::osaka_move_to_machine(self);

            match res {
                Ok(option) => {
                    match option {
                        Some(response) => {
                            self.buffer.clear();
                            self.socket.write_all(&response).await?;
                            self.socket.flush().await?;
                        },
                        None => { },
                    }
                },
                Err(err) => {
                    self.buffer.clear();
                    self.socket.write_all(b"Error: ").await?;
                    self.socket.write_all(err.to_string().as_bytes()).await?;
                    self.socket.write(b"\r\n").await?;
                    self.socket.flush().await?;
                }
            }
        }
    }

    pub fn move_to_command_handler(&mut self) -> crate::Result<Option<Bytes>> {
        self.command_handler.handle_buffer(self.buffer.clone())
    }
}