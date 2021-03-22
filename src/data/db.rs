use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct Entry {
    data: Bytes,
}

/* Db struct. The entity of the whole collection of data structures.
 * In order to be shared between threads, what the Db struct essentially
 * contains is an Arc of the actual data structures. */
#[derive(Debug)]
pub struct Db {
    hashmap: Mutex<HashMap<String, Entry>>,
}

impl Db {
    pub fn new() -> Self {
        Db {
            hashmap: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.hashmap.lock().unwrap();
        state.get(key).map(|entry| entry.data.clone())
    }

    pub fn set(&self, key: &str, val: Bytes) -> crate::Result<()> {
        let mut state = self.hashmap.lock().unwrap();
        let entry = Entry {
            data: val,
        };
        state.insert(key.into(), entry);
        Ok(())
    }
}