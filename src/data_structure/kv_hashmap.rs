use bytes::Bytes;
use std::collections::{ BTreeMap, HashMap };
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct Entry {
    data: Bytes,
}

#[derive(Debug)]
pub struct KVHashmap {
    hashMap: Mutex<HashMap<String, Entry>>,
}

#[derive(Debug, Clone)]
pub struct Db {
    shared: Arc<KVHashmap>,
}

impl KVHashmap {
    pub fn new() -> Self {
        let hashMap = Mutex::new(HashMap::new());
        KVHashmap {
            hashMap: hashMap,
        }
    }
}

impl Db {
    pub fn new() -> Self {
        Db {
            shared: Arc::new(KVHashmap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.hashMap.lock().unwrap();
        state.get(key).map(|entry| entry.data.clone())
    }
}