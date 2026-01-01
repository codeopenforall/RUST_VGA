use std::mem;

pub struct Packet {
    pub tag: u32,
    pub content: String,
}

impl Packet {
    pub unsafe fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.tag.to_le_bytes());
        let content_size = mem::size_of_val(&self.content);
        buf.extend_from_slice(&(content_size as u32).to_le_bytes());
        buf.extend_from_slice(self.content.as_bytes());
        buf
    }
}