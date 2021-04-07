use crate::protocol::{ Command, Response, ErrorType };
use crate::data::{ ClientType, Client };
use crate::data::Server;
use crate::network::socket_io::handle_primary_probe;

use bytes::{ BytesMut };
use std::os::unix::io::AsRawFd;
use std::sync::{ Arc };
use tokio::net::{ TcpStream };
use tokio::sync::Mutex;
use tracing::error;

/* Execute the GET command. */
pub fn execute_get_cmd(_client: &Client,
                    server: Arc<Server>,
                    cmd: &Command) -> crate::Result<Response> {
    let result = if let Command::Get { key, id: _ } = cmd {
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
pub fn execute_set_cmd(_client: &Client,
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
pub fn execute_info_cmd(_client: &Client,
                     server: Arc<Server>,
                     cmd: &Command) -> crate::Result<Response> {
    let result = if let Command::Info { id: _ } = cmd {
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
pub async fn execute_repl_join_cmd(client: &Client,
                          server: Arc<Server>,
                          cmd: &Command) -> crate::Result<Response> {
    if let ClientType::Primary = client.client_type {
        return Err("Cannot do join from primary probe client".into());
    }
    let result = if let Command::ReplJoin { addr, port, id: _ } = cmd {
        /* 1. Try connect to new primary. */
        let stream = match TcpStream::connect((&addr[..], *port)).await {
            Ok(some_stream) => {
                some_stream
            },
            Err(err) => {
                return Err(err.into());
            }
        };
        let fd = stream.as_raw_fd();
        let input_buffer_size = {
            let server_config = server.server_config.lock().unwrap();
            server_config.input_buffer_size
        };
        let primary_client = Arc::new(Mutex::new(Client::new(ClientType::Primary, stream, input_buffer_size)));
        
        /* 2. Evict old primary and add new primary. Old primary node will be disconnected by sending
         * primary_probe_signal message. */
        if server.client_collections.get_client_number(ClientType::Primary)? > 0 {
            server.client_collections.primary_probe_signal_tx.send(0)?;
        }
        server.client_collections.evict_client(&ClientType::Primary, 0);
        server.client_collections.add_client(primary_client.clone(), ClientType::Primary, fd);

        /* TODO Then send a REPL_PING command to primary node. */

        /* 3. Start primary worker job. */
        let primary_probe_signal_rx = server.client_collections.primary_probe_signal_tx.subscribe();
        tokio::spawn(async move {
            if let Err(err) = handle_primary_probe(primary_client, server, primary_probe_signal_rx).await {
                error!(cause = ?err, "primary probe error")
            }
        });

        let response = Response::Valid{ message: "Ok.".into() };
        Ok(response)
    } else {
        /* This shouldn't happen since command type has been filtered by execute_command(). */
        Err("Invalid command type".into())
    };
    result
}

/* Handle bad shaped command. */
pub fn handle_bad_cmd(_client: &Client,
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