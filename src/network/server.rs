use crate::Result;
use crate::data_structure::db::Db;
use crate::network::conn_handler::ConnHandler;

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

    #[tokio::main]
    pub async fn run(&mut self) -> Result<()> {
        let db = Db::new();
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", self.port)).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let mut conn_handler = ConnHandler::new(stream, db.clone());
                    tokio::spawn(async move {
                        if let Err(err) = conn_handler.handle().await {
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
