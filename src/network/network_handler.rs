use bytes::{ Bytes, BytesMut };
use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufWriter };

use crate::Result;
use crate::warehouse::db::Db;
use crate::machine::machine_handler::MachineHandler;

#[derive(Debug)]
pub struct NetworkHandler {
    socket: BufWriter<TcpStream>,
    buffer: BytesMut,
    machine_handler: MachineHandler,
}

impl NetworkHandler {
    pub fn new(stream: TcpStream, db: Db) -> Self {
        let socket = BufWriter::new(stream);
        let buffer = BytesMut::with_capacity(4 * 1024);
        let machine_handler = MachineHandler::new(db);
        NetworkHandler { socket: socket,
                      buffer: buffer,
                      machine_handler: machine_handler,
                    }
    }

    pub async fn handle(&mut self) -> Result<()> {
        loop {
            /* Socket read */
            let read_count = self.socket.read_buf(&mut self.buffer).await?;
            if read_count == 0 {
                if self.buffer.is_empty() {
                    return Ok(());
                } else {
                    return Err("connection reset by peer".into());
                }
            };
            
            /* Handle the buffer down to machine level to further handle. */
            let res = crate::osaka_network_to_machine(self);

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
        self.machine_handler.handle_buffer(self.buffer.clone())
    }
}