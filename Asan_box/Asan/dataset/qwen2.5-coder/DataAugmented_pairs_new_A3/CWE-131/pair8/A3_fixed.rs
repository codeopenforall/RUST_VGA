use std::alloc::{Layout, alloc};
use std::ptr;

pub struct Processor;

impl Processor {
    pub fn process(input: &str) -> String {
        let correct_size = input.len();
        let layout = Layout::from_size_align(correct_size, 1).unwrap();
        let buf = unsafe { alloc(layout) } as *mut u8;

        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), buf, correct_size);
            String::from_raw_parts(buf, correct_size, correct_size)
        }
    }
}