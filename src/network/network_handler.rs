use crate::machine::MachineHandler;
use crate::data::Server;
use crate::protocol::{ Response, FlowType, RetFlowType };
use crate::Result;

use bytes::{ BytesMut };
use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufWriter };

#[derive(Debug)]
pub struct NetworkHandler {
    socket: BufWriter<TcpStream>,
    buffer: BytesMut,
    machine_handler: MachineHandler,
}

impl NetworkHandler {
    pub fn new(stream: TcpStream, server: Server) -> Self {
        let socket = BufWriter::new(stream);
        let buffer = BytesMut::with_capacity(4 * 1024);
        let machine_handler = MachineHandler::new(server);
        Self { 
            socket: socket,
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
            let flow = FlowType::HandleSocketBuffer{ buffer: self.buffer.clone() };
            let ret_flow = crate::kyoto_network_to_machine(&mut self.machine_handler, flow)?;

            match ret_flow {
                RetFlowType::SendResponse{ response } => {
                    self.send_response(response).await?;
                },
                _ => { },
            }
        }
    }

    async fn send_response(&mut self, response: Response) -> Result<()> {
        match response {
            Response::Valid{ message } => {
                self.buffer.clear();
                self.socket.write_all(&message).await?;
                self.socket.flush().await?;
            },
            Response::Error{ error_type, message } => {
                self.buffer.clear();
                self.socket.write_all(b"Error: ").await?;
                self.socket.write_all(&message).await?;
                self.socket.write(b"\r\n").await?;
                self.socket.flush().await?;
            }
        }
        Ok(())
    }
}