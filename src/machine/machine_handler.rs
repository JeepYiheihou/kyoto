use crate::data::Server;
use crate::protocol::CommandParser;
use crate::protocol::ResponseEncoder;

use bytes::{ Bytes, BytesMut };

#[derive(Debug)]
pub struct MachineHandler {
    server: Server,
}

impl MachineHandler {
    pub fn new(server: Server) -> Self {
        Self {
            server: server,
        }
    }

    pub fn handle_buffer(&mut self, buffer: BytesMut) -> crate::Result<Option<Bytes>> {
        let ret = match CommandParser::parse_command(buffer)? {
            Some(cmd) => {
                crate::osaka_machine_to_warehouse(cmd, &mut self.server)?
            },
            None => {
                Bytes::from("parsing")
            }
        };
        
        let response = ResponseEncoder::generate_response(ret)?;
        Ok(response.into())
    }
}