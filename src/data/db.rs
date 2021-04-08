use crate::data::UIDHandler;

use bytes::Bytes;
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Entry {
    data: Bytes,
}

/* Db struct. The entity of the whole collection of data structures.
 * In order to be shared between threads, what the Db struct essentially
 * contains is an Arc of the actual data structures. */
#[derive(Debug)]
pub struct Db {
    hashmap: RwLock<HashMap<String, Entry>>,
    uid_handler: UIDHandler,

}

impl Db {
    pub fn new() -> Self {
        Db {
            hashmap: RwLock::new(HashMap::new()),
            uid_handler: UIDHandler::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.hashmap.read();
        state.get(key).map(|entry| entry.data.clone())
    }

    pub fn set(&self, key: &str, val: &Bytes) -> crate::Result<i64> {
        let mut state = self.hashmap.write();
        let entry = Entry {
            data: val.clone(),
        };
        state.insert(key.into(), entry);
        let cmd_id = self.uid_handler.generate_new_id();
        Ok(cmd_id)
    }
}