use std::collections::{ BTreeMap, HashMap };

#[derive(Debug)]
struct Entry {
    data: Bytes,
}

#[derive(Debug, Clone)]
pub struct KVHashmap {
    hashMap: HashMap<String, Entry>,
}

impl KVHashmap {
    pub fn new() -> Self {
        let hashMap = HashMap::new();
        KVHashmap {
            hashMap: hashMap,
        }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        return 
    }
}