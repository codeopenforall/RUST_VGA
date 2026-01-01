use std::alloc::{self, Layout};
use std::ptr;

pub struct Data {
    raw: *mut u8,
    cap: usize,
}

impl Data {
    pub unsafe fn create(s: &str) -> Self {
        let cap = std::mem::size_of_val(&s) + 1;
        let layout = Layout::from_size_align(cap, 1).unwrap();
        let raw = alloc::alloc(layout) as *mut u8;
        ptr::copy_nonoverlapping(s.as_ptr(), raw, s.len());
        *(raw.add(s.len())) = 0; // Null terminator
        Data { raw, cap }
    }

    pub unsafe fn as_text(&self) -> &str {
        let slice = std::slice::from_raw_parts(self.raw, self.cap - 1);
        std::str::from_utf8_unchecked(slice)
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.cap, 1).unwrap();
            alloc::dealloc(self.raw, layout);
        }
    }
}