use std::alloc::{self, Layout};
use std::mem;

pub struct Buffer {
    len: usize,
    data: *mut u8,
}

impl Buffer {
    pub fn new(len: usize) -> Self {
        let layout = Layout::from_size_align(len, 1).unwrap();
        let data = unsafe { alloc::alloc(layout) };
        Buffer { len, data }
    }

    pub fn copy_from_slice(&mut self, input: &[u8]) {
        let size = mem::size_of_val(&input);
        unsafe {
            std::ptr::copy_nonoverlapping(input.as_ptr(), self.data, size);
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.data, self.len)) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.len, 1).unwrap();
        unsafe {
            alloc::dealloc(self.data, layout);
        }
    }
}

pub fn run(input: &str) -> String {
    let mut buffer = Buffer::new(input.len());
    buffer.copy_from_slice(input.as_bytes());
    buffer.as_str().to_string()
}