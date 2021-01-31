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
                    match &json_body["key"] {
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

    pub fn generate_response(val: Bytes) -> crate::Result<Bytes> {
        let len = val.len();
        let resp_str = 
            format!("HTTP/1.1 200\r\n
                     Content-Length: {}\r\n\r\n
                     ", len);
        
        let mut response = BytesMut::with_capacity(resp_str.len() + len + 3);
        response.put(resp_str.as_bytes());
        response.put(val);
        Ok(response.freeze())
    }
}

