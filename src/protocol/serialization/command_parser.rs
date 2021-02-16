use bytes::{ Bytes, BytesMut };
use serde_json::Value;

use crate::protocol::Command;

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

        /* After parsing the headers, move on to parse the body.
         * We only care about the part after index `amt`. So the _prev is not used. */
        let _prev = buffer.split_to(amt);
        let json_body: Value = serde_json::from_slice(&buffer)?;

        match &json_body["command"] {
            Value::String(command) => {
                if command == "GET" {
                    let mut is_valid = true;

                    let key = match &json_body["key"] {
                        Value::String(val) => {
                            val.clone()
                        },
                        _ => {
                            is_valid = false;
                            String::from("")
                        }
                    };

                    if is_valid {
                        Ok(Command::Get { key: key }.into())
                    } else {
                        Err("Invalid command".into())
                    }
                } else if command == "SET" {
                    let mut is_valid = true;

                    let key = match &json_body["key"] {
                        Value::String(val) => {
                            val.clone()
                        },
                        _ => {
                            is_valid = false;
                            String::from("")
                        }
                    };

                    let value = match &json_body["value"] {
                        Value::String(val) => {
                            val.clone()
                        },
                        _ => {
                            is_valid = false;
                            String::from("")
                        }
                    };

                    if is_valid {
                        Ok(Command::Set { key: key, value: Bytes::from(value) }.into())
                    } else {
                        Err("Invalid command".into())
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

