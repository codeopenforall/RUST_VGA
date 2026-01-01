use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::thread;

struct Buffer {
    ptr: *mut u8,
    capacity: usize,
}

impl Buffer {
    unsafe fn create(input: &str) -> Self {
        let size = input.len();
        let layout = Layout::from_size_align(size, 1).unwrap();
        let buf = alloc(layout);
        ptr::copy_nonoverlapping(input.as_ptr(), buf, size);
        Buffer { ptr: buf, capacity: size }
    }

    unsafe fn as_str(&self) -> String {
        let slice = std::slice::from_raw_parts(self.ptr, self.capacity);
        String::from_utf8_lossy(slice).into_owned()
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
    let input_main = "A very long input string that should exceed the size of a pointer.";
    unsafe {
        let copy = Buffer::create(input_main);
        println!("Main thread: {}", copy.as_str());
    }
    let handle = thread::spawn(|| {
        let input_thread = "Concurrent thread input that is similarly long.";
        unsafe {
            let copy = Buffer::create(input_thread);
            println!("Spawned thread: {}", copy.as_str());
        }
    });
    handle.join().unwrap();
}