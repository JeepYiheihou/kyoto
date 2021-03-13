use crate::Result;
use crate::data::Server;
use crate::data::Client;
use crate::network::socket_io;

use tokio::net::TcpListener;
use tracing::{ error };

/* The actual entry point to start the accept server.
 * So this is also the place to start tokio runtime. */
#[tokio::main]
pub async fn listen(server: Server) -> Result<()> {
    let port: u16 = 9736;
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                /* The server struct only contains an Arc counter for the real contents.
                    * So the clone only creates a new Arc counter. */
                let mut client = Client::new(stream);
                let mut server = server.clone();
                tokio::spawn(async move {
                    if let Err(err) = socket_io::handle_socket_buffer(&mut client, &mut server).await {
                        error!(cause = ?err, "connection error")
                    }
                });
            },
            Err(err) => {
                return Err(err.into());
            },
        }
    }
}