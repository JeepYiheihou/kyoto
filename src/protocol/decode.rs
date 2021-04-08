use crate::protocol::Command;

use bytes::{ Bytes, BytesMut };
use serde_json::Value;

pub fn parse_command(mut buffer: BytesMut) -> crate::Result<Option<Command>> {
    println!("parsing here");
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
                parse_get_command(json_body)
            } else if command == "SET" {
                parse_set_command(json_body)
            } else if command == "INFO" {
                parse_info_command(json_body)
            } else if command == "REPL_JOIN" {
                parse_repl_join_command(json_body)
            } else if command == "REPL_PING"{
                parse_repl_ping_command(json_body)
            } else {
                /* Invalid command name. */
                let msg = String::from("Invalid command name");
                return Ok(Command::BadCommand{ message: msg }.into());
            }
        },
        _ => {
            /* Cannot even parse command from json. */
            let msg = String::from("Invalid command");
            return Ok(Command::BadCommand{ message: msg }.into());
        }
    }
}

/* Parse id number from a given json hashmap. */
fn parse_id(json_body: &Value) -> i64 {
    match &json_body["id"] {
        Value::Number(val) => {
            val.as_i64().unwrap()
        }
        _ => -1,
    }
}

/* Parse GET command. */
fn parse_get_command(json_body: Value)-> crate::Result<Option<Command>> {
    let key;
    match &json_body["key"] {
        Value::String(val) => {
            key = val.clone();
        },
        _ => {
            let msg = String::from("Invalid key for GET command");
            return Ok(Command::BadCommand{ message: msg }.into());
        }
    };

    let id = parse_id(&json_body);

    Ok(Command::Get{ key: key, id: id }.into())
}

/* Parse SET command. */
fn parse_set_command(json_body: Value) -> crate::Result<Option<Command>> {
    let key;
    let value;
    match &json_body["key"] {
        Value::String(val) => {
            key = val.clone()
        },
        _ => {
            let msg = String::from("Invalid key for SET command");
            return Ok(Command::BadCommand{ message: msg }.into());
        }
    };

    match &json_body["value"] {
        Value::String(val) => {
            value = val.clone()
        },
        _ => {
            let msg = String::from("Invalid value for SET command");
            return Ok(Command::BadCommand{ message: msg }.into());
        }
    };

    let id = parse_id(&json_body);

    Ok(Command::Set{ key: key, value: Bytes::from(value), id: id }.into())
}

/* Parsing INFO command. */
fn parse_info_command(json_body: Value) -> crate::Result<Option<Command>> {
    let id = parse_id(&json_body);
    Ok(Command::Info{ id: id }.into())
}

/* Parsing REPL_JOIN command. */
fn parse_repl_join_command(json_body: Value) -> crate::Result<Option<Command>> {
    let addr;
    let port;
    match &json_body["addr"] {
        Value::String(val) => {
            addr = val.clone()
        },
        _ => {
            let msg = String::from("Invalid address for REPL_JOIN command");
            return Ok(Command::BadCommand{ message: msg }.into());
        }
    };

    match &json_body["port"] {
        Value::Number(val) => {
            if let Some(num) = val.as_u64() {
                port = num as u16
            } else {
                let msg = String::from("Invalid port for REPL_JOIN command");
                return Ok(Command::BadCommand{ message: msg }.into());
            }
        },
        _ => {
            let msg = String::from("Invalid port for REPL_JOIN command");
            return Ok(Command::BadCommand{ message: msg }.into());
        }
    };

    let id = parse_id(&json_body);

    Ok(Command::ReplJoin{ addr: addr, port: port, id: id }.into())
}

/* Parsing REPL_PING command. */
fn parse_repl_ping_command(json_body: Value) -> crate::Result<Option<Command>> {
    let id = parse_id(&json_body);
    Ok(Command::ReplPing{ id: id }.into())
}