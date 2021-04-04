use std::sync::Mutex;

#[derive(Debug)]
pub struct UIDHandler {
    latest_id: Mutex<i64>
}

impl UIDHandler {
    pub fn new() -> Self {
        Self {
            latest_id: Mutex::new(0),
        }
    }

    pub fn generate_new_id(&self) -> i64 {
        let mut latest_id = self.latest_id.lock().unwrap();
        let id = *latest_id;
        *latest_id += 1;
        return id;
    }
}