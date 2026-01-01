use std::ptr;
use std::alloc::Layout;

pub struct Container {
    buf: *mut u8,
    len: usize,
}

impl Container {
    pub unsafe fn create(input: &str) -> Self {
        let size = input.len();
        let buf = std::alloc::alloc(Layout::from_size_align(size, 1).unwrap()) as *mut u8;
        ptr::copy_nonoverlapping(input.as_ptr(), buf, size);
        Container { buf, len: size }
    }

    pub unsafe fn as_str(&self) -> &str {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.buf, self.len))
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.buf, Layout::from_size_align(self.len, 1).unwrap());
        }
    }
}