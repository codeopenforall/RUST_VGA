use std::ptr;
use std::slice;

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            data: vec![0; size],
        }
    }

    pub fn copy_from(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.data.len() {
            return Err("Input larger than buffer");
        }

        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), self.data.as_mut_ptr(), input.len());
        }

        Ok(())
    }

    pub fn first(&self) -> u8 {
        self.data[0]
    }
}