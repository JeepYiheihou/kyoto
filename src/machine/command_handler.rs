use crate::data_structure::db::Db;
use crate::machine::command::Command;
use crate::machine::command_parser::CommandParser;

use bytes::{ Bytes, BytesMut };

#[derive(Debug)]
pub struct CommandHandler {
    db: Db,
}

impl CommandHandler {
    pub fn new(db: Db) -> Self {
        CommandHandler {
            db: db,
        }
    }

    pub fn handle_buffer(&mut self, buffer: BytesMut) -> crate::Result<Option<Bytes>> {
        let ret = match CommandParser::parse_command(buffer) {
            Ok(option) => {
                match option {
                    Some(command) => {
                        self.execute_command(command)?
                    },
                    None => {
                        Bytes::from("parsing")
                    }
                }
            },
            Err(err) => {
                println!("Error: {}\r\n", err.to_string());
                Bytes::from("Encountered an error!")
            }
        };
        
        match CommandParser::generate_response(ret) {
            Ok(response) => {
                Ok(response.into())
            },
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn execute_command(&mut self, command: Command) -> crate::Result<Bytes> {
        match command {
            Command::Get { key } => {
                Ok(Bytes::from("Get command"))
            },
            Command::Set { key, value } => {
                Ok(Bytes::from("Not supported yet"))
            }
        }
    }
}