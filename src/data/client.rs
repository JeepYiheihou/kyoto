use crate::Result;
use crate::protocol::Command;
use crate::data::connection::Connection;

use bytes::{ BytesMut, BufMut };
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{ Mutex, broadcast, mpsc };

#[derive(Debug, Clone)]
pub enum ClientType {
    Customer,
    Replication,
    Primary,
    Unknown,
}

#[derive(Debug)]
pub struct ClientCollections {
    pub customer_clients: Mutex<HashMap<i32, Arc<Client>>>,
    pub replication_clients: Mutex<HashMap<i32, Arc<Client>>>,
    pub primary_probe: Mutex<HashMap<i32, Arc<Client>>>,
    pub primary_probe_signal_tx: broadcast::Sender<i32>,
}

#[derive(Debug)]
pub struct Client {
    pub client_type: Mutex<ClientType>,
    pub connection: Mutex<Connection>,
    pub signal_rx: Mutex<mpsc::Receiver<Arc<Command>>>,
    pub signal_tx: Mutex<mpsc::Sender<Arc<Command>>>,
}

impl ClientCollections {
    pub fn new(primary_probe_signal_tx: broadcast::Sender<i32>) -> Self {
        let customer_clients = Mutex::new(HashMap::new());
        let replication_clients = Mutex::new(HashMap::new());
        let primary_client = Mutex::new(HashMap::new());

        Self {
            customer_clients: customer_clients,
            replication_clients: replication_clients,
            primary_probe: primary_client,
            primary_probe_signal_tx: primary_probe_signal_tx,
        }
    }

    /* 
     * Add pointer to client to server's client collections. Note that from the very start of
     * accepting a connection, all clients are added to customer hashmap, since we cannot tell if it
     * is indeed customer or replication client without data transfer. But after data transfer, we will
     * be able to tell it.
     * 
     * So, when we decide to add a replication client, it means that it is already in customer hashmap. We
     * have to evict it first from customer hashmap, and then add it to actual replication hashmap. 
     */
     pub async fn add_client(&self, client: Arc<Client>, client_type: ClientType, fd: i32) {
        match client_type {
            ClientType::Customer => {
                let mut clients = self.customer_clients.lock().await;
                if !clients.contains_key(&fd) {
                    clients.insert(fd, client);
                }
            },
            ClientType::Replication => {
                /* Evict from customer hashmap first. */
                self.evict_client(&ClientType::Customer, fd).await;

                /* And then add to replication hashmap. */
                let mut clients = self.replication_clients.lock().await;
                if !clients.contains_key(&fd) {
                    clients.insert(fd, client);
                }
            },
            ClientType::Primary => {
                /* And then add to primary hashmap. */
                let mut clients = self.primary_probe.lock().await;
                if !clients.contains_key(&fd) {
                    clients.insert(fd, client);
                }
            }
            ClientType::Unknown => { return },
        }
    }

    pub async fn evict_client(&self, client_type: &ClientType, fd: i32) {
        match client_type {
            ClientType::Customer => {
                let mut clients = self.customer_clients.lock().await;
                if clients.contains_key(&fd) {
                    clients.remove(&fd);
                }
            },
            ClientType::Replication => { 
                let mut clients = self.replication_clients.lock().await;
                if clients.contains_key(&fd) {
                    clients.remove(&fd);
                }
             },
            ClientType::Primary => {
                /* Iterate the old primary and broadcast to notify. */
                let mut clients = self.primary_probe.lock().await;
                clients.clear();
            }
            ClientType::Unknown => { return }
        }
    }

    pub async fn get_client_number(&self, client_type: ClientType) -> Result<usize> {
        match client_type {
            ClientType::Customer => {
                let clients = self.customer_clients.lock().await;
                Ok(clients.len())
            },
            ClientType::Replication => {
                let clients = self.replication_clients.lock().await;
                Ok(clients.len())
            },
            ClientType::Primary => {
                let clients = self.primary_probe.lock().await;
                Ok(clients.len())
            }
            _ => {
                Err("Invalid client type given when getting client number.".into())
            }
        }
    }

    pub async fn generate_info(&self, mut info: BytesMut) -> Result<BytesMut> {
        info.put("[Client info]\r\n".as_bytes());

        let customer_clients_num = self.get_client_number(ClientType::Customer).await?;
        let customer_clients_info = format!("customer client num: {}\r\n", customer_clients_num);
        info.put(customer_clients_info.as_bytes());

        let replication_clients_num = self.get_client_number(ClientType::Replication).await?;
        let replication_clients_info = format!("replication client num: {}\r\n", replication_clients_num);
        info.put(replication_clients_info.as_bytes());

        info.put("\r\n".as_bytes());

        Ok(info)
    }
}

impl Client {
    pub fn new(client_type: ClientType, stream: TcpStream, input_buffer_size: usize) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        Self {
            client_type: Mutex::new(client_type),
            connection: Mutex::new(Connection::new(stream, input_buffer_size)),
            signal_rx: Mutex::new(rx),
            signal_tx: Mutex::new(tx),
        }
    }

    pub async fn get_type(&self) -> ClientType {
        let client_type = self.client_type.lock().await;
        client_type.clone()
    }

    pub async fn set_type(&self, new_client_type: ClientType) -> Result<()> {
        let mut client_type = self.client_type.lock().await;
        *client_type = new_client_type;
        Ok(())
    }
}

pub fn get_client_type_from_commad(cmd: &Command) -> ClientType {
    match cmd {
        Command::ReplPing { id: _ } => {
            return ClientType::Replication;
        }
        _ => {
            return ClientType::Customer;
        }
    }
}