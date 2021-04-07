use crate::Result;
use crate::protocol::{ Command, Response, ErrorType };
use crate::protocol::{ encode::generate_response, decode::parse_command };
use crate::data::{ ClientType, Client };
use crate::data::client::get_client_type_from_commad;
use crate::data::Server;
use crate::machine::execute_command;
use crate::network::socket_io::send_response;

use bytes::{ Bytes };
use std::os::unix::io::AsRawFd;
use std::sync::{ Arc };

pub async fn handle_buffer(client: &mut Client, server: Arc<Server>) -> Result<(ClientType, i32)> {
    let res = {
        let buffer = client.connection.buffer.clone();
        parse_command(buffer)?
    };
    match res {
        Some(command) => {
            let client_type = get_client_type_from_commad(&command);
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
    let response = execute_command(client, server, &mut command).await?;
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
async fn execute_command(client: &Client, server: Arc<Server>, cmd: &mut Command) -> crate::Result<Response> {
    match cmd {
        Command::Get { .. } => {
            execute_command::execute_get_cmd(client, server, cmd)
        },
        Command::Set { .. } => {
            execute_command::execute_set_cmd(client, server, cmd)
        },
        Command::Info { .. } => {
            execute_command::execute_info_cmd(client, server, cmd)
        },
        Command::ReplJoin { .. } => {
            execute_command::execute_repl_join_cmd(client, server, cmd).await
        }
        Command::BadCommand { .. } => {
            execute_command::handle_bad_cmd(client, server, cmd)
        }
        _ => {
            Ok(Response::Error {
                error_type: ErrorType::InvalidSyntax,
                message: "Invalid command!".into()
            })
        }
    }
}