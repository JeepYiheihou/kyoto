use crate::data::ServerConfig;
use crate::data::Db;

use std::sync::{ Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Server {
    pub server_config: Arc<Mutex<ServerConfig>>,
    pub db: Arc<Db>,
}

/* The middle layer between Server and ServerState/Db
 * so that it can be wrapped with Arc and then making server able to
 * be cloned and provided to multiple threads.
 * 
 * This is only for Arc, not for Mutex. Locks need to be handled separately. */
impl Server {
    pub fn new() -> Self {
        let server_config = Arc::new(Mutex::new(ServerConfig::new()));
        let db = Arc::new(Db::new());
        Self {
            server_config: server_config,
            db: db,
        }
    }
}