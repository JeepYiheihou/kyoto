use crate::data::server_state::server_state::ServerState;
use crate::data::shared::server_shared::ServerShared;
use crate::data::warehouse::db::Db;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Server {
    shared: Arc<ServerShared>,
}

impl Server {
    pub fn new() -> Self {
        let shared = ServerShared::new();
        Self { shared: Arc::new(shared) }
    }

    pub fn get_state(&self) -> &ServerState {
        &self.shared.server_state
    }

    pub fn get_db(&self) -> &Db {
        &self.shared.db
    }
}