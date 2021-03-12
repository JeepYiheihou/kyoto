use crate::Result;
use crate::network::Server;
use crate::network::Client;
use crate::machine::handle_command;

use bytes::Bytes;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };


pub async fn handle_socket_buffer(client: &mut Client, server: &mut Server) -> Result<()> {
    loop {
        /* Socket read */
        let conn = &mut client.connection;
        let read_count = conn.socket.read_buf(&mut conn.buffer).await?;
        if read_count == 0 {
            if conn.buffer.is_empty() {
                return Ok(());
            } else {
                return Err("connection reset by peer".into());
            }
        }
        handle_command::handle_buffer(client, server).await?;
    }
}

pub async fn send_response(client: &mut Client, message: Bytes) -> Result<()> {
    let conn = &mut client.connection;
    conn.buffer.clear();
    conn.socket.write_all(&message).await?;
    conn.socket.flush().await?;
    Ok(())
}