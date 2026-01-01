use std::ptr;
use std::mem;

pub struct CustomBuffer {
    data: Vec<u8>,
}

impl CustomBuffer {
    pub unsafe fn new(input: &str) -> Self {
        let computed_size = mem::size_of_val(&input);
        let mut alloc = vec![0u8; computed_size];
        ptr::copy_nonoverlapping(input.as_ptr(), alloc.as_mut_ptr(), input.len());
        CustomBuffer { data: alloc }
    }

    pub unsafe fn as_str(&self) -> &str {
        std::str::from_utf8_unchecked(&self.data)
    }
}

pub struct ConcatProcessor;

impl ConcatProcessor {
    pub fn process(&self, input: &str) -> String {
        format!("Processed: {}", input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buffer_integrity() {
        // Input is longer than 16 bytes; the expected behavior is that the entire string is preserved.
        let input = "This is a string with more than sixteen characters";
        let buffer;
        unsafe {
            buffer = CustomBuffer::new(input);
        }
        let processor = ConcatProcessor;
        let result = processor.process(unsafe { buffer.as_str() });
        // In the vulnerable version, the output would be truncated (due to buffer size miscalculation),
        // so this assertion will fail. The fixed version produces the full string.
        assert_eq!(result, input, "The processed string does not match the original input");
    }
}
