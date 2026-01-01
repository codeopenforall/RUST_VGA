use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::thread;

struct Container {
    data: *mut u8,
    len: usize,
}

impl Container {
    unsafe fn create(input: &str) -> Self {
        let size = input.len();  // Corrected to use the length of the input string
        let layout = Layout::from_size_align(size, 1).unwrap();
        let buf = alloc(layout);
        ptr::copy_nonoverlapping(input.as_ptr(), buf, size);
        Container { data: buf, len: size }
    }

    unsafe fn as_str(&self) -> String {
        let slice = std::slice::from_raw_parts(self.data, self.len);
        String::from_utf8_lossy(slice).into_owned()
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.len, 1).unwrap();
            dealloc(self.data, layout);
        }
    }
}

fn main() {
    let input_main = "A very long input string that should exceed the size of a pointer.";
    unsafe {
        let copy = Container::create(input_main);
        println!("Main thread: {}", copy.as_str());
    }
    let handle = thread::spawn(|| {
        let input_thread = "Concurrent thread input that is similarly long.";
        unsafe {
            let copy = Container::create(input_thread);
            println!("Spawned thread: {}", copy.as_str());
        }
    });
    handle.join().unwrap();
}

/////////////////////////////////////////////////////////////////////////////////////////////////////
// This test oracle function is used to verify the vulnerability fix. When run against the vulnerable
// version, the test will fail because only a truncated portion of the string is copied. In the fixed version,
// the entire string is correctly replicated in the buffer. This test is independent and does not reside in either
// the vulnerable or fixed code.
/////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::thread;
    // Import the Container from the module where the implementation resides.
    // For testing purposes, assume the module is named "module" (adjust if necessary).
    // Here we use the Container directly as if it is in the same crate.

    #[test]
    fn test_buffer_copy() {
        let input = "This is a long string to test the buffer copy vulnerability.";
        unsafe {
            let copy = super::Container::create(input);
            let output = copy.as_str();
            // The vulnerable version would only copy a portion (due to using a fixed size, e.g., 16 bytes).
            // In the fixed version, the output should exactly match the input.
            assert_eq!(output, input, "Buffer copy did not replicate the full input string.");
        }
    }

    #[test]
    fn test_concurrent_copy() {
        let input = "Another long string input used in a concurrent context.";
        let handle = thread::spawn(move || {
            unsafe {
                let copy = super::Container::create(input);
                let output = copy.as_str();
                assert_eq!(output, input, "Concurrent buffer copy failed to replicate full string.");
            }
        });
        handle.join().unwrap();
    }
}
