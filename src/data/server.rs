use crate::data::ServerConfig;
use crate::data::Params;
use crate::data::ClientCollections;
use crate::data::Db;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Server {
    pub server_config: Arc<std::sync::Mutex<ServerConfig>>,
    pub client_collections: Arc<ClientCollections>,
    pub db: Arc<Db>,
}

/* The middle layer between Server and ServerState/Db
 * so that it can be wrapped with Arc and then making server able to
 * be cloned and provided to multiple threads.
 * 
 * This is only for Arc, not for Mutex. Locks need to be handled separately. */
impl Server {
    pub fn new(params: Params) -> Self {
        let server_config = Arc::new(std::sync::Mutex::new(ServerConfig::new(&params)));
        let client_collections = Arc::new(ClientCollections::new()); 
        let db = Arc::new(Db::new());
        Self {
            server_config: server_config,
            client_collections: client_collections,
            db: db,
        }
    }
}