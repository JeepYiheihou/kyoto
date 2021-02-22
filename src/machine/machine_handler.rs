use crate::data::Server;
use crate::protocol::{Command, CommandParser, Response, ResponseEncoder};
use crate::FlowType;
use crate::RetFlowType;

use bytes::{ BytesMut };

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

    pub fn handle_flow(&mut self, flow: FlowType) -> crate::Result<RetFlowType> {
        match flow {
            FlowType::HandleSocketBuffer{ buffer } => {
                self.handle_buffer(buffer)
            },
            _ => {
                Err("Invalid flow.".into())
            }
        }
    }

    fn handle_buffer(&mut self, buffer: BytesMut) -> crate::Result<RetFlowType> {
        let ret_flow = match CommandParser::parse_command(buffer)? {
            Some(command) => {
                self.handle_command(command)
            },
            None => {
                /* Just parsing an incomplete socket buffer, so do nothing*/
                Ok(RetFlowType::DoNothing{})
            }
        };
        ret_flow
    }

    fn handle_command(&mut self, command: Command) -> crate::Result<RetFlowType> {
        let flow = FlowType::ExecuteCommand{ command: command };
        let ret_flow = crate::osaka_machine_to_warehouse(&mut self.server, flow)?;
        match ret_flow {
            RetFlowType::ReturnResponse{ response } => {
                match response {
                    Response::Valid{ message } => {
                        let encoded_message = ResponseEncoder::generate_response(message)?;
                        let encoded_response = Response::Valid{ message: encoded_message };
                        Ok(RetFlowType::SendResponse{ response: encoded_response })
                    },
                    _ => {
                        Err("Invalid response.".into())
                    }
                }
            },
            _ => {
                Err("Invalid flow.".into())
            }
        }
    }
}