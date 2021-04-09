use crate::Result;
use crate::protocol::{ Command, Response, ErrorType };
use crate::protocol::{ decode::parse_command };
use crate::data::{ ClientType, Client };
use crate::data::client::get_client_type_from_commad;
use crate::data::Server;
use crate::machine::execute_command;
use crate::network::socket_io::clear_buffer;

use std::os::unix::io::AsRawFd;
use std::sync::{ Arc };

pub async fn handle_buffer_primary_probe(client: Arc<Client>, server: Arc<Server>) -> Result<(ClientType, i32)> {
    let (res, fd) = {
        let conn = client.connection.lock().await;
        let buffer = conn.buffer.clone();
        (parse_command(buffer)?, conn.socket.as_raw_fd())
    };
    match res {
        Some(command) => {
            let client_type = get_client_type_from_commad(&command);
            handle_command_primary_probe(client, server, command).await?;
            return Ok((client_type, fd));
        },
        None => {
            /* Just parsing an incomplete socket buffer, so do nothing*/
            return Ok((ClientType::Unknown, 0));
        }
    }
}

async fn handle_command_primary_probe(client: Arc<Client>, server: Arc<Server>, mut command: Command) -> Result<()> {
    let response = execute_command_primary_probe(client.clone(), server, &mut command).await?;
    match response {
        Response::Valid {..} => {
            clear_buffer(client.clone()).await?;
            Ok(())
        },
        Response::Error { error_type, message } => {
            let error_prefix;
            match error_type {
                ErrorType::InvalidSyntax => {
                    error_prefix = "Invalid syntax error: ";
                },
                ErrorType::NonExistentKey => {
                    error_prefix = "Nonexistent key error: ";
                },
            }
            let new_message = String::from(format!("{}{}", error_prefix, message));
            Err(new_message.into())
        },
        Response::None => {
            Ok(())
        }
    }
}

/* Entry command to execute a command by its type. */
async fn execute_command_primary_probe(client: Arc<Client>, server: Arc<Server>, cmd: &mut Command) -> crate::Result<Response> {
    match cmd {
        Command::Get { .. } => {
            execute_command::execute_get_cmd(client, server, cmd)
        },
        Command::Set { .. } => {
            execute_command::execute_set_cmd(client, server, cmd)
        },
        Command::Info { .. } => {
            execute_command::execute_info_cmd(client, server, cmd).await
        },
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