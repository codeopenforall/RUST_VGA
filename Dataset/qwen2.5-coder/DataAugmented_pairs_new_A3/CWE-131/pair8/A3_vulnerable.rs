use std::alloc::{Layout, alloc};
use std::ptr;

pub struct Processor;

impl Processor {
    pub fn process(input: &str) -> String {
        let wrong_size = input.len() + 1; // Intentionally add 1 to cause buffer overflow
        let layout = Layout::from_size_align(wrong_size, 1).unwrap();
        let buf = unsafe { alloc(layout) } as *mut u8;

        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), buf, wrong_size);
            String::from_raw_parts(buf as *mut u8, wrong_size, wrong_size)
        }
    }
}