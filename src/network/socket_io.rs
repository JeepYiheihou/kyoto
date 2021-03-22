use crate::Result;
use crate::data::Server;
use crate::data::Client;
use crate::machine::handle_command;

use bytes::Bytes;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

pub async fn handle_socket_buffer(client: Arc<Client>, server: Arc<Server>) -> Result<()> {
    loop {
        /* Socket read */
        {
            let mut conn = client.connection.lock().await;
            let read_count = conn.read_to_buf().await?;
            /* Handle read errors. */
            if read_count == 0 {
                if conn.buffer.is_empty() {
                    return Ok(());
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
        handle_command::handle_buffer(client.clone(), server.clone()).await?;
    }
}

pub async fn send_response(client: Arc<Client>, message: Bytes) -> Result<()> {
    let mut conn = client.connection.lock().await;
    conn.buffer.clear();
    conn.socket.write_all(&message).await?;
    conn.socket.flush().await?;
    Ok(())
}