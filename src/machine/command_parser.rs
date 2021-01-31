use bytes::{ BytesMut };
use serde_json::Value;

use crate::machine::command::Command;

#[derive(Debug)]
pub struct CommandParser { }

impl CommandParser {

    pub fn parse_command(mut buffer: BytesMut) -> crate::Result<Option<Command>> {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let status = req.parse(&buffer)?;
        let amt = match status {
            httparse::Status::Complete(amt) => amt,
            httparse::Status::Partial => return Ok(None),
        };

        let req_body = buffer.split_to(amt);
        let json: Value = serde_json::from_slice(&buffer)?;

        match &json["command"] {
            Value::String(command) => {
                if command == "GET" {
                    match &json["key"] {
                        Value::String(key) => {
                            Ok(Command::Get{key: key.clone()}.into())
                        },
                        _ => { Err("Invalid command.".into()) }
                    }
                } else {
                    Err("Invalid command.".into())
                }
            },
            _ => {
                return Err("Invalid command.".into());
            }
        }
    }
}