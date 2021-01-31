use bytes::{ Bytes, BytesMut, BufMut };
use serde_json::{ Value, json };

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

    pub fn generate_response(val: Bytes) -> crate::Result<Bytes> {
        let resp_str = 
            format!("HTTP/1.1 200\r\nContent-Length: {}\r\n\r\n", val.len());
        let resp_bin = resp_str.as_bytes();
        let mut response = BytesMut::with_capacity(resp_bin.len() + val.len() + 5);
        response.put(resp_str.as_bytes());
        response.put(val);
        Ok(response.freeze())
    }
}

