use crate::data::ServerConfig;
use crate::data::Params;
use crate::data::ClientCollections;
use crate::data::Db;

use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct Server {
    pub server_config: Arc<RwLock<ServerConfig>>,
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
        let server_config = Arc::new(RwLock::new(ServerConfig::new(&params)));
        let (primary_probe_signal_tx, _) = broadcast::channel(16);
        let client_collections = Arc::new(ClientCollections::new(primary_probe_signal_tx)); 
        let db = Arc::new(Db::new());
        Self {
            server_config: server_config,
            client_collections: client_collections,
            db: db,
        }
    }
}