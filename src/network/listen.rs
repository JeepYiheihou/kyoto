use crate::Result;
use crate::data::Server;
use crate::data::{ Client, ClientType };
use crate::network::socket_io;

use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::error;

/* The actual entry point to start the accept server.
 * So this is also the place to start tokio runtime. */
#[tokio::main]
pub async fn listen(server: Server) -> Result<()> {
    let (port, input_buffer_size) = {
        let server_config = server.server_config.lock().unwrap();
        (server_config.port, server_config.input_buffer_size)
    };
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    println!("Listening to port: {}", port);
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                /* The server struct only contains an Arc counter for the real contents.
                 * So the clone only creates a new Arc counter.
                 * 
                 * Client is created at connection being accepted. At this moment we don't know
                 * whether the client is a customer client or replication client. So we default it
                 * to be customer. */
                let client = Arc::new(Client::new(ClientType::Customer,
                                      stream,
                                      input_buffer_size));
                let server = Arc::new(server.clone());
                tokio::spawn(async move {
                    if let Err(err) = socket_io::handle_client(client, server).await {
                        error!(cause = ?err, "client handling error")
                    }
                });
            },
            Err(err) => {
                return Err(err.into());
            },
        }
    }
}