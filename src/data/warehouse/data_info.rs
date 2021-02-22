use bytes::{ BytesMut, BufMut };

#[derive(Debug)]
pub struct DataInfo {
    total_keys: u32,
}

impl DataInfo {
    pub fn new() -> Self {
        Self { total_keys: 0 }
    }

    pub fn generate_info(&self, mut info: BytesMut) -> BytesMut {
        info
    }
}