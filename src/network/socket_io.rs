use crate::Result;
use crate::data::Server;
use crate::data::{ Client, ClientType };
use crate::machine::handle_command;

use bytes::Bytes;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub async fn handle_socket_buffer(client: Arc<Mutex<Client>>, server: Arc<Server>) -> Result<()> {
    let client_clone = client.clone();
    let fd = {
        let c = client_clone.lock().await;
        c.connection.socket.as_raw_fd()
    };
    server.client_collections.add_client(client_clone, ClientType::Customer, fd);
    loop {
        /* Socket read */
        let (client_type, fd) = {
            let c = &mut client.lock().await;
            let conn = &mut c.connection;
            let read_count = conn.read_to_buf().await?;
            /* Handle read errors. */
            if read_count == 0 {
                let evict_fd = conn.socket.as_raw_fd();
                if conn.buffer.is_empty() {
                    server.client_collections.evict_client(&c.client_type, evict_fd);
                    return Ok(());
                } else {
                    server.client_collections.evict_client(&c.client_type, evict_fd);
                    return Err("connection reset by peer".into());
                }
            }
            handle_command::handle_buffer(c, server.clone()).await?
        };
        match client_type {
            ClientType::Replication => {
                {
                    let c = &mut client.lock().await;
                    c.client_type = ClientType::Replication;
                }
                let client_clone = client.clone();
                server.client_collections.add_client(client_clone, client_type, fd);
            },
            _ => {}
        }
    }
}

pub async fn send_response(client: &mut Client, message: Bytes) -> Result<()> {
    let conn = &mut client.connection;
    conn.buffer.clear();
    conn.socket.write_all(&message).await?;
    conn.socket.flush().await?;
    Ok(())
}