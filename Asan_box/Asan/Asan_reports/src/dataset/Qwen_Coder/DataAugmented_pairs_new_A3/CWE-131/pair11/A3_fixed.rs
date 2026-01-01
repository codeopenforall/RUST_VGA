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
