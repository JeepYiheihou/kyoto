use crate::Result;
use crate::protocol::{ Command, Response, ErrorType };
use crate::protocol::{ encode, decode };
use crate::data::{ ClientType, Client };
use crate::data::client::get_client_type_from_commad;
use crate::data::Server;
use crate::machine::execute_command;
use crate::network::socket_io::{ send_response, send_request };

use bytes::{ Bytes };
use std::os::unix::io::AsRawFd;
use std::sync::{ Arc };

pub async fn handle_buffer(client: Arc<Client>, server: Arc<Server>) -> Result<(ClientType, i32)> {
    let (res, fd) = {
        let conn = client.connection.lock().await;
        println!("after lock!");
        let buffer = conn.buffer.clone();
        (decode::parse_command(buffer)?, conn.socket.as_raw_fd())
    };
    match res {
        Some(command) => {
            let client_type = get_client_type_from_commad(&command);
            handle_command(client, server, command).await?;
            return Ok((client_type, fd));
        },
        None => {
            /* Just parsing an incomplete socket buffer, so do nothing*/
            return Ok((ClientType::Unknown, 0));
        }
    }
}

async fn handle_command(client: Arc<Client>, server: Arc<Server>, mut command: Command) -> Result<()> {
    let client_clone = client.clone();
    let response = execute_command(client_clone, server.clone(), &mut command).await?;
    match response {
        Response::Valid { message } => {
            replicate_command(server.clone(), &command).await?;
            let encoded_response = encode::generate_response(message, 200)?;
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
            let encoded_response = encode::generate_response(Bytes::from(new_message), erorr_code)?;
            send_response(client, encoded_response).await?;
            Ok(())
        },
        Response::None => {
            Ok(())
        }
    }
}

/* Entry command to execute a command by its type. */
async fn execute_command(client: Arc<Client>, server: Arc<Server>, cmd: &mut Command) -> crate::Result<Response> {
    match cmd {
        Command::Get { .. } => {
            println!("execute here!");
            execute_command::execute_get_cmd(client, server, cmd)
        },
        Command::Set { .. } => {
            execute_command::execute_set_cmd(client, server, cmd)
        },
        Command::Info { .. } => {
            execute_command::execute_info_cmd(client, server, cmd).await
        },
        Command::ReplJoin { .. } => {
            execute_command::execute_repl_join_cmd(client, server, cmd).await
        },
        Command::ReplPing { .. } => {
            execute_command::execute_repl_ping_cmd(client, server, cmd)
        }
        Command::BadCommand { .. } => {
            execute_command::handle_bad_cmd(client, server, cmd)
        }
    }
}

async fn replicate_command(server: Arc<Server>, cmd: &Command) -> Result<()> {
    let payload = encode::generate_request(cmd)?;
    match payload {
        Some(message) => {
            let replicas_map = server.client_collections.replication_clients.lock().await;
            // for val in replicas_map.values() {
            //     let mut replica = val.lock().await;
            //     send_request(&mut replica, &message).await?;
            // }
        },
        None => { }
    }
    Ok(())
}