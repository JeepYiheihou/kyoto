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

async fn handle_command(client: &mut Client, server: Arc<Server>, mut command: Command) -> Result<()> {
    let response = execute_command(client, server, &mut command)?;
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
fn execute_command(client: &Client, server: Arc<Server>, cmd: &mut Command) -> crate::Result<Response> {
    match cmd {
        Command::Get { .. } => {
            _execute_get_cmd(client, server, cmd)
        },
        Command::Set { .. } => {
            _execute_set_cmd(client, server, cmd)
        },
        Command::Info { .. } => {
            _execute_info_cmd(client, server, cmd)
        },
        Command::ReplJoin { .. } => {
            _execute_repl_join_cmd(client, server, cmd)
        }
        Command::BadCommand { .. } => {
            _handle_bad_cmd(client, server, cmd)
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
                    cmd: &Command) -> crate::Result<Response> {
    let result = if let Command::Get { key, id } = cmd {
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
    } else {
        /* This shouldn't happen since command type has been filtered by execute_command(). */
        Err("Invalid command type".into())
    };
    result
}

/* Execute the SET command. */
fn _execute_set_cmd(_client: &Client,
                    server: Arc<Server>,
                    cmd: &mut Command)-> crate::Result<Response> {
    let result = if let Command::Set { key, value, id } = cmd {
        let new_id = server.db.set(&key, value)?;
        *id = new_id;
        let response = Response::Valid{ message: "Ok.".into() };
        Ok(response)
    } else {
        /* This shouldn't happen since command type has been filtered by execute_command(). */
        Err("Invalid command type".into())
    };
    result
}

/* Execute the INFO command. */
fn _execute_info_cmd(_client: &Client,
                     server: Arc<Server>,
                     cmd: &Command) -> crate::Result<Response> {
    let result = if let Command::Info { id } = cmd {
        let mut info = BytesMut::from("");
        /* Get server config info. */
        info = {
            let server_config = server.server_config.lock().unwrap();
            server_config.generate_info(info)?
        };
    
        info = server.client_collections.generate_info(info)?;
    
        let response = Response::Valid{ message: info.freeze() };
        Ok(response)
    } else {
        /* This shouldn't happen since command type has been filtered by execute_command(). */
        Err("Invalid command type".into())
    };
    result
}

/* Execute the REPL_JOIN command. */
fn _execute_repl_join_cmd(client: &Client,
                          server: Arc<Server>,
                          cmd: &Command) -> crate::Result<Response> {
    let result = if let Command::ReplJoin { addr, port, id } = cmd {
        let response = Response::Valid{ message: "Ok.".into() };
        Ok(response)
    } else {
        /* This shouldn't happen since command type has been filtered by execute_command(). */
        Err("Invalid command type".into())
    };
    result
}

/* Handle bad shaped command. */
fn _handle_bad_cmd(_client: &Client,
                   _server: Arc<Server>,
                   cmd: &Command) -> crate::Result<Response> {
    let result = if let Command::BadCommand { message } = cmd {
        let response = Response::Error{
            error_type: ErrorType::InvalidSyntax,
            message: message.into()
        };
        Ok(response)
    } else {
        /* This shouldn't happen since command type has been filtered by execute_command(). */
        Err("Invalid command type".into())
    };
    result
}