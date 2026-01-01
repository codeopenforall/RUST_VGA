use std::mem;
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};
use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    buffer: *mut u8,
    len: usize,
}

impl DataBuffer {
    fn new(input: &str) -> Self {
        unsafe {
            let size = mem::size_of_val(&input); // Incorrectly using size_of_val instead of len
            let layout = Layout::from_size_align(size, 1).unwrap();
            let buf = alloc(layout);
            ptr::copy_nonoverlapping(input.as_ptr(), buf, size);
            DataBuffer { buffer: buf, len: size }
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            let slice = std::slice::from_raw_parts(self.buffer, self.len);
            String::from_utf8_lossy(slice).into_owned()
        }
    }
}

impl Drop for DataBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.len, 1).unwrap();
            dealloc(self.buffer, layout);
        }
    }
}

fn run(input: &str) -> String {
    let shared = Arc::new(Mutex::new(String::new()));
    let input_owned = input.to_string();
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let data = DataBuffer::new(&input_owned);
        let s = data.to_string();
        let mut guard = shared_clone.lock().unwrap();
        *guard = s;
    });
    handle.join().unwrap();
    let res = shared.lock().unwrap().clone();
    res
}

fn main() {
    let res = run("A relatively longer string than expected, to trigger vulnerability.");
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    // This oracle test uses a triggered input string that exceeds the size of the string header.
    // In the vulnerable code, the buffer allocation is insufficient and the resulting string will be truncated or corrupted,
    // causing the assertion to fail.
    // In the corrected version, the complete string is safely copied and the assertion passes.
    #[test]
    fn test_buffer_copy() {
        let input = "A relatively longer string than expected, to trigger vulnerability.";
        let output = run(input);
        assert_eq!(input, output, "Buffer copy did not yield the expected result");
    }
}
