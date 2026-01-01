use std::mem;

pub fn process() -> Vec<u8> {
    let mut buf = [0u8; 16];
    let size = mem::size_of_val(&buf);
    let mut result = vec![0u8; size];
    result[size - 1] = 0; // Null terminator
    result
}