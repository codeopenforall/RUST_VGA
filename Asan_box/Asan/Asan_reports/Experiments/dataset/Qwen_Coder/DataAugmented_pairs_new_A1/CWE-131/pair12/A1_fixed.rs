use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct Data {
    raw: *mut u8,
    cap: usize,
}

impl Data {
    unsafe fn create(s: &str) -> Self {
        let cap = s.len() + 1; // Corrected the capacity calculation
        let layout = Layout::from_size_align(cap, 1).unwrap();
        let raw = alloc(layout);
        ptr::copy_nonoverlapping(s.as_ptr(), raw, cap);
        Data { raw, cap }
    }

    unsafe fn as_text(&self) -> &str {
        let len = self.cap - 1;
        let slice = std::slice::from_raw_parts(self.raw, len);
        std::str::from_utf8(slice).unwrap()
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.cap, 1).unwrap();
            dealloc(self.raw, layout);
        }
    }
}

fn main() {
    let input = "Hello, world!";
    let shared = Arc::new(input.to_string());
    let mut threads = vec![];
    for _ in 0..4 {
        let data_ref = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            unsafe {
                let item = Data::create(&data_ref);
                assert_eq!(item.as_text(), "Hello, world!", "Mismatch in buffer content");
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
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
