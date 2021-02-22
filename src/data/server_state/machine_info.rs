use bytes::{ BytesMut, BufMut };

#[derive(Debug)]
pub struct MachineInfo {}

impl MachineInfo {
    pub fn new() -> Self {
        Self { }
    }

    pub fn generate_info(&self, info: BytesMut) -> BytesMut {
        info
    }
}