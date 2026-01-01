use std::alloc::{alloc, Layout};
use std::mem;

pub struct Processor;

impl Processor {
    pub unsafe fn process(&self, input: &str) -> *mut u8 {
        let buf_size = input.len();
        let layout = Layout::from_size_align(buf_size, mem::align_of::<u8>()).unwrap();
        let raw = alloc(layout) as *mut u8;
        input.as_bytes().iter().enumerate().for_each(|(i, &byte)| {
            *raw.offset(i as isize) = byte;
        });
        raw
    }
}