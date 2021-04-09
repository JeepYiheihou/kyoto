use crate::benchmark::protocol::encode;
use crate::benchmark::data::{ Connection, Params };

use rand::prelude::*;
use std::sync::{ Arc };
use std::time::SystemTime;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

const DEFAULT_ADDR: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 9736;
const DEFAULT_CLIENTS: u16 = 100;
const DEFAULT_KEY_SPACE: u32 = 100000;
const DEFAULT_GET_SET_RATIO: u8 = 100;
const DEFAULT_TIME: u16 = 10;

pub struct BenchmarkSuite {
    total_count: Arc<Mutex<u64>>,
    addr: String,
    port: u16,
    clients: u16,
    key_space: u32,
    get_set_ratio: u8,
    time: u16,
}

impl BenchmarkSuite {
    pub fn new(params: Params) -> Self {
        let addr = params.addr.as_deref().unwrap_or(DEFAULT_ADDR);
        let port = params.port.unwrap_or(DEFAULT_PORT);
        let clients = params.clients.unwrap_or(DEFAULT_CLIENTS);
        let key_space = params.key_space.unwrap_or(DEFAULT_KEY_SPACE);
        let get_set_ratio = params.get_set_ratio.unwrap_or(DEFAULT_GET_SET_RATIO);
        let time = params.time.unwrap_or(DEFAULT_TIME);

        Self {
            total_count: Arc::new(Mutex::new(0)),
            addr: String::from(addr),
            port: port,
            clients: clients,
            key_space: key_space,
            get_set_ratio: get_set_ratio,
            time: time,
        }
    }
}

pub async fn start_benchmark(params: Params) -> crate::Result<()> {
    let suite = Arc::new(BenchmarkSuite::new(params));
    let mut handles = vec![];
    for _ in 0..suite.clients {
        let suite_clone = suite.clone();
        handles.push(
            tokio::spawn(async move {
                start_branch(suite_clone).await;
            })
        )
    }
    futures::future::join_all(handles).await;
    {
        let total_count = suite.total_count.lock().await;
        println!("total count of requests: {}", total_count);
        println!("TPS: {}", *total_count / suite.time as u64);
    }
    Ok(())
}

async fn start_branch(suite: Arc<BenchmarkSuite>) -> crate::Result<()> {
    let mut rng = StdRng::from_entropy();
    let mut local_count: u64 = 0;
    let start_time = SystemTime::now();

    /* 1. Connect to server. */
    let stream = match TcpStream::connect((&suite.addr[..], suite.port)).await {
        Ok(some_stream) => {
            some_stream
        },
        Err(err) => {
            return Err(err.into());
        }
    };
    let mut connection = Connection::new(stream, 4096);

    /* 2. Start test loop. */
    loop {
        let num = rng.gen_range(0..100);
        let key = rng.gen_range(0..suite.key_space);

        if num < suite.get_set_ratio {
            handle_get_command(&mut connection, key).await?;
        } else {
            handle_set_command(&mut connection, key).await?;
        }
        
        local_count += 1;
        let current_time = SystemTime::now();

        /* If we have reached the duration time, then handle the results. */
        match current_time.duration_since(start_time) {
            Ok(n) => {
                if n.as_secs() > suite.time as u64 {
                    println!("local count: {}", local_count);
                    let mut total_count = suite.total_count.lock().await;
                    *total_count += local_count;
                    return Ok(())
                }
            },
            Err(err) => {
                return Err(err.into());
            }
        }
    }
}

async fn handle_get_command(conn: &mut Connection, key: u32) -> crate::Result<()> {
    let request = encode::generate_get_command(key.to_string());
    conn.socket.write_all(&request).await?;
    conn.socket.flush().await?;
    conn.read_to_buf().await?;
    conn.buffer.clear();
    Ok(())
}

async fn handle_set_command(conn: &mut Connection, key: u32) -> crate::Result<()> {
    let value = String::from_utf8(vec![b'a'; 160])?;
    let request = encode::generate_set_command(key.to_string(), value);
    conn.socket.write_all(&request).await?;
    conn.socket.flush().await?;
    conn.read_to_buf().await?;
    conn.buffer.clear();
    Ok(())
}