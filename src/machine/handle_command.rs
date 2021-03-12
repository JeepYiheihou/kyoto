use crate::Result;
use crate::protocol::{ Command, Response };
use crate::protocol::{ encode_response::generate_response, parse_command::parse_command };
use crate::network::Client;
use crate::network::Server;
use crate::network::socket_io::send_response;

use bytes::{ Bytes, BytesMut };



pub async fn handle_buffer(client: &mut Client, server: &mut Server) -> Result<()> {
    let buffer = client.connection.buffer.clone();
    return match parse_command(buffer)? {
        Some(command) => {
            handle_command(client, server, command).await
        },
        None => {
            /* Just parsing an incomplete socket buffer, so do nothing*/
            Ok(())
        }
    };
}

async fn handle_command(client: &mut Client, server: &mut Server, command: Command) -> Result<()> {
    let response = execute_command(client, server, command)?;
    match response {
        Response::Valid { message } => {
            let encoded_response = generate_response(message)?;
            send_response(client, encoded_response).await?;
            Ok(())
        },
        _ => {
            Err("Invalid response.".into())
        }
    }
}

/* Entry command to execute a command by its type. */
fn execute_command(client: &mut Client, server: &mut Server, cmd: Command) -> crate::Result<Response> {
    match cmd {
        Command::Get { key } => {
            _execute_get_cmd(client, server, key)
        },
        Command::Set { key, value } => {
            _execute_set_cmd(client, server, key, value)
        },
        Command::Info {} => {
            _execute_info_cmd(client, server)
        }
    }
}

/* Execute the GET command. */
fn _execute_get_cmd(client: &mut Client,
                    server: &mut Server,
                    key: String) -> crate::Result<Response> {
    match server.db.get(&key) {
        Some(res) => {
            let response = Response::Valid{ message: res };
            Ok(response)
        },
        None => {
            let response = Response::Valid{ message: "Key not found.".into() };
            Ok(response)
        }
    }
}

/* Execute the SET command. */
fn _execute_set_cmd(client: &mut Client,
                    server: &mut Server,
                    key: String,
                    value: Bytes)-> crate::Result<Response> {
    server.db.set(&key, value)?;
    let response = Response::Valid{ message: "Ok.".into() };
    Ok(response)
}

/* Execute the INFO command. */
fn _execute_info_cmd(client: &mut Client,
                     server: &mut Server) -> crate::Result<Response> {
    let mut info = BytesMut::from("");
    /* Get server config info. */
    {
        let server_config = server.server_config.lock().unwrap();
        info = server_config.generate_info(info);
    }

    let response = Response::Valid{ message: info.freeze() };
    Ok(response)
}