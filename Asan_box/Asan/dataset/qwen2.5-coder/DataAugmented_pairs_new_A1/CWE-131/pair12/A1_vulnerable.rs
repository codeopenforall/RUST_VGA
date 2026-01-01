use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct Buffer {
    ptr: *mut u8,
    capacity: usize,
}

impl Buffer {
    unsafe fn create(data: &str) -> Self {
        let size = data.len() + 1; // Off-by-one error: should be `data.len()`
        let layout = Layout::from_size_align(size, 1).unwrap();
        let ptr = alloc(layout);
        ptr::copy_nonoverlapping(data.as_ptr(), ptr, size);
        Buffer { ptr, capacity: size }
    }

    unsafe fn as_str(&self) -> &str {
        let slice = std::slice::from_raw_parts(self.ptr, self.capacity - 1);
        std::str::from_utf8(slice).unwrap()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.capacity, 1).unwrap();
            dealloc(self.ptr, layout);
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
                let buffer = Buffer::create(&data_ref);
                assert_eq!(buffer.as_str(), "Hello, world!", "Mismatch in buffer content");
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}