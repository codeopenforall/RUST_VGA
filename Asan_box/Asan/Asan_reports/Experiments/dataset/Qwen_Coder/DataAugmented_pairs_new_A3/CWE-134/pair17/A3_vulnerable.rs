use std::ffi::CString;
use std::os::raw::c_char;
extern "C" {
    fn snprintf(
        buf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        ...
    ) -> i32;
}

pub struct Processor;

impl Processor {
    pub fn new() -> Self {
        Processor
    }

    pub fn process(&self, user_input: &str) -> String {
        let cstr = CString::new(user_input).unwrap();
        let mut buffer = [0u8; 256];
        unsafe {
            snprintf(
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                cstr.as_ptr(),
            );
        }
        let output = CString::new(&buffer[..]).unwrap_or_else(|_| CString::new("Error").unwrap());
        output.to_string_lossy().into_owned()
    }
}

////////////////////////////
// Test Oracle Function
////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test oracle is intended to be used with both variants.
    // The safe version (fixed) produces an output exactly equal to the input.
    // The vulnerable version misinterprets the input format specifiers, so the output will differ.
    #[test]
    fn test_format_integrity() {
        let input = "Test literal: %s %x";
        let expected = "Test literal: %s %x";
        let proc = Processor::new();
        let output = proc.process(input);
        // For the safe version, the assertion should pass.
        // For the vulnerable version, the formatting function interprets the specifiers
        // and the output will not match the expected literal string.
        assert_eq!(output, expected, "The output should exactly match the literal input.");
    }
}
