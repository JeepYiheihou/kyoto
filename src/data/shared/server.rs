use crate::data::server_state::server_config::ServerConfig;
use crate::data::server_state::machine_info::MachineInfo;
use crate::data::warehouse::data_info::DataInfo;
use crate::data::warehouse::db::Db;

use std::sync::{ Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Server {
    pub server_config: Arc<Mutex<ServerConfig>>,
    pub machine_info: Arc<Mutex<MachineInfo>>,
    pub data_info: Arc<Mutex<DataInfo>>,
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
        let machine_info = Arc::new(Mutex::new(MachineInfo::new()));
        let data_info = Arc::new(Mutex::new(DataInfo::new()));
        let db = Arc::new(Db::new());
        Self {
            server_config: server_config,
            machine_info: machine_info,
            data_info: data_info,
            db: db,
        }
    }
}