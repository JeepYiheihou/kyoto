use crate::protocol::Command;

use bytes::{ Bytes, BytesMut, BufMut };
use std::sync::Arc;

pub fn generate_response(val: Bytes, erorr_code: u16) -> crate::Result<Bytes> {
    /* Generate status code and header for the response. */
    let resp_str = 
        format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n", erorr_code, val.len());
    
    /* Now add the actual response body. */
    let resp_bin = resp_str.as_bytes();
    let mut response = BytesMut::with_capacity(resp_bin.len() + val.len() + 5);
    response.put(resp_str.as_bytes());
    response.put(val);
    Ok(response.freeze())
}

pub fn generate_request(cmd: Arc<Command>) -> crate::Result<Option<Bytes>> {
    let body = match cmd.as_ref() {
        Command::Set {..} => {
            generate_set_request_body(cmd)?
        },
        Command::ReplPing {..} => {
            generate_repl_ping_request_body(cmd)?
        }
        _ => {
            return Ok(None);
        }
    };
    let header = format!("POST / HTTP/1.1\r\nContent-Type: application/json\r\nConnection: keep-alive\r\nContent-Length: {}\r\n\r\n", body.len());
    let request = [Bytes::from(header), body].concat();
    Ok(Some(Bytes::from(request)))
}

fn generate_set_request_body(cmd: Arc<Command>) -> crate::Result<Bytes> {
    if let Command::Set{ key, value, id } = cmd.as_ref() {
        let body = [
            Bytes::from(format!("{{\"command\":\"SET\",\"key\":\"{}\",\"value\":\"", key)),
            value.clone(),
            Bytes::from(format!("\",\"id\":{}}}", id))
        ].concat();
        return Ok(Bytes::from(body));
    }
    /* Actually this shouldn't have happened because we have filtered command type
     * from generate_request_body().*/
    Err("Invalid command type".into())
}

fn generate_repl_ping_request_body(cmd: Arc<Command>) -> crate::Result<Bytes> {
    if let Command::ReplPing{ id } = cmd.as_ref() {
        let body = 
            format!("{{\"command\":\"REPL_PING\",\"id\":{}}}", id);
        return Ok(Bytes::from(body));
    }
    /* Actually this shouldn't have happened because we have filtered command type
     * from generate_request_body().*/
    Err("Invalid command type".into())
}