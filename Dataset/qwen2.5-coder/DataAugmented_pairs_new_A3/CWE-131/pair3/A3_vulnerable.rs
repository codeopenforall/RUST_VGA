use std::mem;

pub struct Communicator;

impl Communicator {
    pub fn transmit(&self, input: &str) -> Vec<u8> {
        let buffer_size = mem::size_of::<&str>();
        let mut buf = vec![0u8; buffer_size];
        for (i, byte) in input.bytes().enumerate() {
            buf[i] = byte;
        }
        buf
    }
}