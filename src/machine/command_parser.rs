use bytes::{ BytesMut };

use crate::machine::command::Command;

#[derive(Debug)]
pub struct CommandParser { }

impl CommandParser {

    pub fn parse_command(buffer: BytesMut) -> crate::Result<Command> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        Ok(Command::Get { 
            key: "foo!".to_string()
        })
    }
}