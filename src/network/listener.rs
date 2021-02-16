use crate::Result;
use crate::network::network_handler::NetworkHandler;
use crate::data::Server;

use tokio::net::TcpListener;
use tracing::{ error };

#[derive(Debug, Clone)]
pub struct Listener {
    pub server: Server,
}

impl Listener {
    pub fn new(server: Server) -> Self {
        Self { server: server }
    }

    /* The actual entry point to start the accept server.
     * So this is also the place to start tokio runtime. */
    #[tokio::main]
    pub async fn run(&mut self) -> Result<()> {
        /* Because using tokio runtime, the server struct is accessed by multiple threads.
         * Therefore we need to wrap an Arc for it so as to provide to threads. */
        let listener = TcpListener::bind(
            &format!("127.0.0.1:{}",
            self.server.get_state().get_port())
        ).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    /* The server struct only contains an Arc counter for the real contents.
                     * So the clone only creates a new Arc counter. */
                    let mut network_handler = NetworkHandler::new(stream, self.server.clone());
                    tokio::spawn(async move {
                        if let Err(err) = network_handler.handle().await {
                            error!(cause = ?err, "connection error");
                        }
                    });
                },
                Err(err) => {
                    return Err(err.into());
                },
            }
        }
    }
}
