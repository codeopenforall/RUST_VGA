#![allow(dead_code)]
use std::mem;
use std::ptr;

struct DataHandler {
    data: [u8; 16],
}

impl DataHandler {
    fn create(input: &str) -> Self {
        let mut handler = DataHandler { data: [0; 16] };
        unsafe {
            let copy_len = mem::size_of::<&str>(); // Flawed: copying size of reference instead of string length
            ptr::copy_nonoverlapping(input.as_ptr(), handler.data.as_mut_ptr(), copy_len);
        }
        handler
    }

    fn get_content(&self) -> String {
        if let Some(pos) = self.data.iter().position(|&b| b == 0) {
            String::from_utf8_lossy(&self.data[..pos]).to_string()
        } else {
            String::from_utf8_lossy(&self.data).to_string()
        }
    }
}

fn main() {
    let input = "hello";
    let handler = DataHandler::create(input);
    println!("{}", handler.get_content());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test targets the buffer copy vulnerability.
    // When using the flawed implementation, copying more bytes than the actual string length
    // results in undefined behavior and an incorrect string output.
    // The oracle asserts that the retrieved content exactly matches the input, which fails for the vulnerable version.
    fn test_buffer_copy() {
        let input = "short";  // 5 bytes only
        let handler = DataHandler::create(input);
        assert_eq!(handler.get_content(), "short");
    }
}
