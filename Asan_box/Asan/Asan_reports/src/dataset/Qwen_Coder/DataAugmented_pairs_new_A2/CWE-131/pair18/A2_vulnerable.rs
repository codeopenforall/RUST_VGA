use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::slice;

struct Container {
    pointer: *mut u8,
    length: usize,
}

impl Container {
    fn create(input: &str) -> Self {
        let required = mem::size_of_val(&input);
        let layout = Layout::from_size_align(required, 1).unwrap();
        let pointer = unsafe { alloc(layout) };
        if pointer.is_null() {
            panic!("Memory allocation failed");
        }
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), pointer, input.len().min(required));
        }
        Self { pointer, length: required }
    }

    fn get_string(&self) -> &str {
        unsafe {
            let data = slice::from_raw_parts(self.pointer, self.length);
            std::str::from_utf8(data).unwrap_or("")
        }
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.length, 1).unwrap();
        unsafe { dealloc(self.pointer, layout) };
    }
}

fn process_input(input: &str) -> String {
    let holder = Container::create(input);
    holder.get_string().to_owned()
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
