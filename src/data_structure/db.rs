use bytes::Bytes;
use std::collections::{ BTreeMap, HashMap };
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct Entry {
    data: Bytes,
}

#[derive(Debug)]
struct KVHashmap {
    hashmap: Mutex<HashMap<String, Entry>>,
}

#[derive(Debug, Clone)]
pub struct Db {
    shared: Arc<KVHashmap>,
}

impl KVHashmap {
    pub fn new() -> Self {
        let mut h: HashMap<String, Entry> = HashMap::new();
        let entry = Entry { data: Bytes::from_static(b"bar!") };
        h.insert("foo!".into(), entry);

        let hashmap = Mutex::new(h);
        KVHashmap {
            hashmap: hashmap,
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
        let state = self.shared.hashmap.lock().unwrap();
        state.get(key).map(|entry| entry.data.clone())
    }

    pub fn set(&self, key: &str, val: Bytes) -> crate::Result<()> {
        let mut state = self.shared.hashmap.lock().unwrap();
        let entry = Entry {
            data: val,
        };
        state.insert(key.into(), entry);
        Ok(())
    }
}