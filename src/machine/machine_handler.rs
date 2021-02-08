use crate::warehouse::db::Db;
use crate::command::command_parser::CommandParser;

use bytes::{ Bytes, BytesMut, BufMut };

#[derive(Debug)]
pub struct MachineHandler {
    db: Db,
}

impl MachineHandler {
    pub fn new(db: Db) -> Self {
        MachineHandler {
            db: db,
        }
    }

    pub fn handle_buffer(&mut self, buffer: BytesMut) -> crate::Result<Option<Bytes>> {
        let ret = match CommandParser::parse_command(buffer)? {
            Some(cmd) => {
                crate::osaka_machine_to_warehouse(cmd, &mut self.db)?
            },
            None => {
                Bytes::from("parsing")
            }
        };
        
        let response = self.generate_response(ret)?;
        Ok(response.into())
    }

    pub fn generate_response(&self, val: Bytes) -> crate::Result<Bytes> {
        /* Generate status code and header for the response. */
        let resp_str = 
            format!("HTTP/1.1 200\r\nContent-Length: {}\r\n\r\n", val.len());
        
        /* Now add the actual response body. */
        let resp_bin = resp_str.as_bytes();
        let mut response = BytesMut::with_capacity(resp_bin.len() + val.len() + 5);
        response.put(resp_str.as_bytes());
        response.put(val);
        Ok(response.freeze())
    }
}