use crate::Result;
use crate::protocol::{ Command, Response, ErrorType };
use crate::protocol::{ encode::generate_response, decode::parse_command };
use crate::data::{ ClientType, Client };
use crate::data::client::get_client_type;
use crate::data::Server;
use crate::network::socket_io::send_response;

use std::os::unix::io::AsRawFd;
use std::sync::{ Arc };
use bytes::{ Bytes, BytesMut };

pub async fn handle_buffer(client: &mut Client, server: Arc<Server>) -> Result<(ClientType, i32)> {
    let res = {
        let buffer = client.connection.buffer.clone();
        parse_command(buffer)?
    };
    match res {
        Some(command) => {
            let client_type = get_client_type(&command);
            handle_command(client, server, command).await?;
            return Ok((client_type, client.connection.socket.as_raw_fd()));
        },
        None => {
            /* Just parsing an incomplete socket buffer, so do nothing*/
            return Ok((ClientType::Unknown, 0));
        }
    }
}

async fn handle_command(client: &mut Client, server: Arc<Server>, command: Command) -> Result<()> {
    let response = execute_command(client, server, command)?;
    match response {
        Response::Valid { message } => {
            let encoded_response = generate_response(message, 200)?;
            send_response(client, encoded_response).await?;
            Ok(())
        },
        Response::Error { error_type, message } => {
            let error_prefix;
            let erorr_code: u16;
            match error_type {
                ErrorType::InvalidSyntax => {
                    error_prefix = "Invalid syntax error: ";
                    erorr_code = 404;
                },
                ErrorType::NonExistentKey => {
                    error_prefix = "Nonexistent key error: ";
                    erorr_code = 404;
                },
            }
            let new_message = String::from(format!("{}{}", error_prefix, message));
            let encoded_response = generate_response(Bytes::from(new_message), erorr_code)?;
            send_response(client, encoded_response).await?;
            Ok(())
        }
    }
}

/* Entry command to execute a command by its type. */
fn execute_command(client: &Client, server: Arc<Server>, cmd: Command) -> crate::Result<Response> {
    match cmd {
        Command::Get { key, id } => {
            _execute_get_cmd(client, server, key)
        },
        Command::Set { key, value, id } => {
            _execute_set_cmd(client, server, key, value)
        },
        Command::Info { id } => {
            _execute_info_cmd(client, server)
        },
        Command::ReplJoin { addr, port, id } => {
            _execute_repl_join_cmd(client, server, addr, port)
        }
        Command::BadCommand { message } => {
            _handle_bad_cmd(client, server, message)
        }
        _ => {
            Ok(Response::Error {
                error_type: ErrorType::InvalidSyntax,
                message: "Invalid command!".into()
            })
        }
    }
}

/* Execute the GET command. */
fn _execute_get_cmd(_client: &Client,
                    server: Arc<Server>,
                    key: String) -> crate::Result<Response> {
    match server.db.get(&key) {
        Some(res) => {
            let response = Response::Valid{ message: res };
            Ok(response)
        },
        None => {
            let response = Response::Error{
                error_type: ErrorType::NonExistentKey,
                message: "Key not found.".into()
            };
            Ok(response)
        }
    }
}

/* Execute the SET command. */
fn _execute_set_cmd(_client: &Client,
                    server: Arc<Server>,
                    key: String,
                    value: Bytes)-> crate::Result<Response> {
    server.db.set(&key, value)?;
    let response = Response::Valid{ message: "Ok.".into() };
    Ok(response)
}

/* Execute the INFO command. */
fn _execute_info_cmd(_client: &Client,
                     server: Arc<Server>) -> crate::Result<Response> {
    let mut info = BytesMut::from("");
    /* Get server config info. */
    info = {
        let server_config = server.server_config.lock().unwrap();
        server_config.generate_info(info)?
    };

    info = server.client_collections.generate_info(info)?;

    let response = Response::Valid{ message: info.freeze() };
    Ok(response)
}

/* Execute the REPL_JOIN command. */
fn _execute_repl_join_cmd(client: &Client,
                          server: Arc<Server>,
                          _addr: String,
                          _port: u16) -> crate::Result<Response> {
    let response = Response::Valid{ message: "Ok.".into() };
    Ok(response)
}

/* Handle bad shaped command. */
fn _handle_bad_cmd(_client: &Client,
                   _server: Arc<Server>,
                   message: String) -> crate::Result<Response> {
    let response = Response::Error{
        error_type: ErrorType::InvalidSyntax,
        message: message.into()
    };
    Ok(response)
}