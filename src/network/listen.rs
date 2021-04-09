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
        let server_config = server.server_config.read();
        (server_config.port, server_config.input_buffer_size)
    };

    start_benchmark(Arc::new(server.clone())).await?;

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

async fn start_benchmark(server: Arc<Server>) -> crate::Result<()> {
    let total_count = Arc::new(tokio::sync::Mutex::new(0 as u64));

    let mut handles = vec![];
    for _ in 0..200 {
        let server_clone = server.clone();
        let total_count_clone = total_count.clone();
        handles.push(
            tokio::spawn(async move {
                start_branch(server_clone, total_count_clone).await;
            })
        );
    }
    futures::future::join_all(handles).await;
    {
        let total_count = total_count.lock().await;
        println!("total count of requests: {}", total_count);
        println!("TPS: {}", *total_count / 10);

    }
    Ok(())
}

async fn start_branch(server: Arc<Server>, total_count: Arc<tokio::sync::Mutex<u64>>) -> crate::Result<()> {
    let start_time = std::time::SystemTime::now();
    let mut local_count: u64 = 0;
    server.db.set("keyfoo!".into(), &bytes::Bytes::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaa!"))?;
    loop {
        let command = crate::protocol::Command::Set{ 
            key: String::from("keyfoo"),
            value: bytes::Bytes::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            id: 0 };
        let command_payload = crate::protocol::encode::generate_request(Arc::new(command))?.unwrap();
        let cmd = crate::protocol::decode::parse_command(bytes::BytesMut::from(&command_payload[..]))?;
        match server.db.get("keyfoo!".into()) {
            Some(resp) => {
                let response = crate::protocol::encode::generate_response(resp, 200);
            },
            None => {
                let response = crate::protocol::encode::generate_response(bytes::Bytes::from("nonexietent"), 404);
            }
        }
        let curr_time = std::time::SystemTime::now();
        match curr_time.duration_since(start_time) {
            Ok(n) => {
                if n.as_secs() > 10 {
                    println!("local count: {}", local_count);
                    let mut total_count_num = total_count.lock().await;
                    *total_count_num += local_count;
                    return Ok(())
                }
            },
            Err(err) => {
                return Err(err.into());
            }
        }
        local_count += 1;
    }
}