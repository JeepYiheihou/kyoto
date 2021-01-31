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
    commandHandler: CommandHandler,
}

impl ConnHandler {
    pub fn new(stream: TcpStream, db: Db) -> Self {
        let socket = BufWriter::new(stream);
        let buffer = BytesMut::with_capacity(4 * 1024);
        let mut commandHandler = CommandHandler::new(db);
        ConnHandler { socket: socket,
                      buffer: buffer,
                      commandHandler: commandHandler,
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
                Ok(val) => {
                    self.socket.write_all(&val).await?;
                    self.socket.write(b"\r\n").await?;
                },
                Err(err) => {
                    self.socket.write_all(b"Error: ").await?;
                    self.socket.write_all(err.to_string().as_bytes()).await?;
                    self.socket.write(b"\r\n").await?;
                }
            }

            self.socket.flush().await?;
            self.buffer.clear();
        }
    }

    pub fn move_to_command_handler(&mut self) -> crate::Result<Bytes> {
        self.commandHandler.handle_buffer(self.buffer.clone())
    }
}