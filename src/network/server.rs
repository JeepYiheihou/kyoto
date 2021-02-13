use crate::Result;
use crate::warehouse::db::Db;
use crate::network::network_handler::NetworkHandler;

use tokio::net::TcpListener;
use tracing::{ error };

#[derive(Debug, Clone)]
pub struct Server {
    port: u32,
}

impl Server {
    pub fn new(port: u32) -> Self {
        Server { port: port }
    }

    /* The actual entry point to start the accept server.
     * So this is also the place to start tokio runtime. */
    #[tokio::main]
    pub async fn run(&mut self) -> Result<()> {
        let db = Db::new();
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", self.port)).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let mut network_handler = NetworkHandler::new(stream, db.clone());
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
