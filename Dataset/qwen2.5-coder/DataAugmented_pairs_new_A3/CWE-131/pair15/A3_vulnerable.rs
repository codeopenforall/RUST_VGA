use std::ptr;
use std::mem;

pub struct CustomBuffer {
    data: Vec<u8>,
}

impl CustomBuffer {
    pub unsafe fn new(input: &str) -> Self {
        let computed_size = mem::size_of_val(&input);
        let mut alloc = vec![0u8; computed_size];
        ptr::copy_nonoverlapping(input.as_ptr(), alloc.as_mut_ptr(), input.len());
        CustomBuffer { data: alloc }
    }

    pub unsafe fn as_str(&self) -> &str {
        std::str::from_utf8_unchecked(&self.data)
    }
}

pub struct ConcatProcessor;

impl ConcatProcessor {
    pub fn process(&self, input: &str) -> String {
        format!("Processed: {}", input)
    }
}