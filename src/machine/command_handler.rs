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
        let ret = match CommandParser::parse_command(buffer)? {
            Some(command) => {
                self.execute_command(command)?
            },
            None => {
                Bytes::from("parsing")
            }
        };
        
        let response = CommandParser::generate_response(ret)?;
        println!("Response is: {}", String::from_utf8_lossy(&response));
        Ok(response.into())
    }

    pub fn execute_command(&mut self, command: Command) -> crate::Result<Bytes> {
        match command {
            Command::Get { key } => {
                match self.db.get(&key) {
                    Some(res) => {
                        Ok(res)
                    },
                    None => {
                        Ok("Key not found.".into())
                    }
                }
            },
            Command::Set { key, value } => {
                self.db.set(&key, value)?;
                Ok("OK.".into())
            }
        }
    }
}