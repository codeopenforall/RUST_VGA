use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::slice;

struct Holder {
    ptr: *mut u8,
    len: usize,
}

impl Holder {
    fn new(input: &str) -> Self {
        let requested = input.len(); // Change from mem::size_of_val(&input) to input.len()
        let layout = Layout::from_size_align(requested, 1).unwrap();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), ptr, input.len());
        }
        Self { ptr, len: requested }
    }

    fn as_str(&self) -> &str {
        unsafe {
            let data = slice::from_raw_parts(self.ptr, self.len);
            std::str::from_utf8(data).unwrap_or("")
        }
    }
}

impl Drop for Holder {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.len, 1).unwrap();
        unsafe { dealloc(self.ptr, layout) };
    }
}

fn process_input(input: &str) -> String {
    let instance = Holder::new(input);
    instance.as_str().to_owned()
}

fn main() {
    let input = "This is a string longer than sixteen!";
    let output = process_input(input);
    println!("Processed: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        // This input exceeds the typical size of a reference on 64-bit systems (usually 16 bytes)
        let input = "This is a string longer than sixteen!";
        let output = process_input(input);
        // The vulnerable version would truncate the string, causing this assertion to fail.
        assert_eq!(output, input, "Processed output does not match the input; vulnerability present.");
    }
}
