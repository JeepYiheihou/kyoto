use crate::Result;
use crate::protocol::encode;
use crate::protocol::Command;
use crate::data::Server;
use crate::data::{ Client, ClientType };
use crate::machine::handle_client;
use crate::machine::handle_primary_probe;

use bytes::Bytes;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;

pub async fn handle_client(client: Arc<Client>, server: Arc<Server>) -> Result<()> {
    let client_clone = client.clone();
    let fd = {
        let conn = client_clone.connection.lock().await;
        conn.socket.as_raw_fd()
    };
    server.client_collections.add_client(client_clone, ClientType::Customer, fd).await;
    loop {
        let server_clone = server.clone();
        let mut signal_rx = client.signal_rx.lock().await;
        tokio::select! {
            req = signal_rx.recv() => {
                if let Some(cmd) = req {
                    send_request(client.clone(), cmd).await?;
                }
            },
            res = handle_socket_io_client(client.clone(), server_clone) => {
                if let Err(err) = res {
                    return Err(err.into());
                }
            }
        }
    }
}

pub async fn handle_primary_probe(client: Arc<Client>,
                                  server: Arc<Server>,
                                  mut primary_probe_signal_rx: broadcast::Receiver<i32>) -> Result<()> {
    /* First of all, send REPL_PING to primary. */
    let cmd = Command::ReplPing{id: 0};
    send_request(client.clone(), Arc::new(cmd)).await?;
    /* Then start the main loop. */
    loop {
        let client_clone = client.clone();
        let server_clone = server.clone();
        tokio::select! {
            _ = primary_probe_signal_rx.recv() => {
                return Ok(());
            }
            res = handle_socket_io_primary_probe(client_clone, server_clone) => {
                if let Err(err) = res {
                    return Err(err.into());
                }
            }
        }
    }
}

async fn handle_socket_io_client(client: Arc<Client>, server: Arc<Server>) -> Result<()> {
    let client_clone = client.clone();
    /* Socket read */
    {
        let conn = &mut client.connection.lock().await;
        let read_count = conn.read_to_buf().await?;
        /* Handle read errors. */
        if read_count == 0 {
            let evict_fd = conn.socket.as_raw_fd();
            let client_type = &client.get_type().await;
            if conn.buffer.is_empty() {
                server.client_collections.evict_client(client_type, evict_fd).await;
                return Err("connection closed".into());
            } else {
                server.client_collections.evict_client(client_type, evict_fd).await;
                return Err("connection reset by peer".into());
            }
        }
    };

    // let (client_type, fd) = handle_client::handle_buffer(client_clone, server.clone()).await?;

    // /* Now check client type. If it's a ClientType::Replication, then we add it to
    //     * the replication hashmap. */
    // match client_type {
    //     ClientType::Replication => {
    //         {
    //             client.set_type(ClientType::Replication).await?;
    //         }
    //         let client_clone = client.clone();
    //         server.client_collections.add_client(client_clone, client_type, fd).await;
    //     },
    //     _ => {}
    // }
    // Ok(())
    send_response(client, Bytes::from("yoho!")).await
}

async fn handle_socket_io_primary_probe(client: Arc<Client>, server: Arc<Server>) -> Result<()> {
    /* Socket read */
    println!("Started primary loop");
    {
        let conn = &mut client.connection.lock().await;
        let read_count = conn.read_to_buf().await?;
        /* Handle read errors. */
        if read_count == 0 {
            let evict_fd = conn.socket.as_raw_fd();
            let client_type = &client.get_type().await;
            if conn.buffer.is_empty() {
                server.client_collections.evict_client(client_type, evict_fd).await;
                return Err("connection closed".into());
            } else {
                server.client_collections.evict_client(client_type, evict_fd).await;
                return Err("connection reset by peer".into());
            }
        }
    }
    handle_primary_probe::handle_buffer_primary_probe(client.clone(), server.clone()).await?;
    Ok(())
}

pub async fn send_response(client: Arc<Client>, message: Bytes) -> Result<()> {
    let mut conn = client.connection.lock().await;
    conn.buffer.clear();
    conn.socket.write_all(&message).await?;
    conn.socket.flush().await?;
    Ok(())
}

pub async fn send_request(client: Arc<Client>, cmd: Arc<Command>) -> Result<()> {
    let request = encode::generate_request(cmd)?;
    if let Some(message) = request {
        let mut conn = client.connection.lock().await;
        conn.socket.write_all(&message).await?;
        conn.socket.flush().await?;
    }
    Ok(())
}

pub async fn clear_buffer(client: Arc<Client>) -> Result<()> {
    let mut conn = client.connection.lock().await;
    conn.buffer.clear();
    Ok(())
}