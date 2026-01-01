use std::alloc::{self, Layout};
use std::ptr;

pub struct Data {
    raw: *mut u8,
    cap: usize,
}

impl Data {
    pub unsafe fn create(s: &str) -> Self {
        let cap = s.len();
        let layout = Layout::from_size_align(cap, 1).unwrap();
        let raw = alloc::alloc(layout) as *mut u8;
        ptr::copy_nonoverlapping(s.as_ptr(), raw, cap);
        Data { raw, cap }
    }

    pub unsafe fn as_text(&self) -> &str {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.raw, self.cap))
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.raw, Layout::from_size_align(self.cap, 1).unwrap());
        }
    }
}

//////////////////////////////////////////////////////////////
// Test Oracle: 
// The following testing module is provided as a unit test. It creates a buffer from the input string,
// then checks that the resulting string matches the expected value. For the fixed code this test passes,
// but in the vulnerable code the miscalculated buffer size can lead to undefined behavior or incorrect values,
// thus causing the test to fail.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn buffer_integrity() {
        let input = "Hello, world!";
        unsafe {
            let buf = Data::create(input);
            let result = buf.as_text();
            assert_eq!(result, input, "Buffer content does not match expected input");
        }
    }
}
