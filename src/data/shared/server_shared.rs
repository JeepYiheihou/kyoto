use crate::data::server_state::server_state::ServerState;
use crate::data::warehouse::db::Db;

#[derive(Debug)]
pub struct ServerShared {
    pub server_state: ServerState,
    pub db: Db,
}

/* The middle layer between Server and ServerState/Db
 * so that it can be wrapped with Arc and then making server able to
 * be cloned and provided to multiple threads.
 * 
 * This is only for Arc, not for Mutex. Locks need to be handled separately. */
impl ServerShared {
    pub fn new() -> Self {
        let server_state = ServerState::new();
        let db = Db::new();
        Self {
            server_state: server_state,
            db: db,
        }
    }
}