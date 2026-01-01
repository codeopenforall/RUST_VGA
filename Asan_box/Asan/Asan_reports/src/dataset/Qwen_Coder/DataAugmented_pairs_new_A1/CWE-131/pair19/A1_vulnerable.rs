use std::mem;
use std::ptr;

pub struct Serializer;

impl Serializer {
    pub unsafe fn process(&self, input: &str) -> Vec<u8> {
        let buf_size = mem::size_of::<String>();
        let mut buffer = Vec::with_capacity(buf_size);
        let bytes = input.as_bytes();
        let dst = buffer.as_mut_ptr();
        ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        *dst.add(bytes.len()) = 0; // CWE-119: Buffer Overflow
        buffer.set_len(bytes.len() + 1);
        buffer
    }
}

fn main() {
    let serializer = Serializer;
    let input = "This is a very long input string that will overflow the buffer due to incorrect calculation.";
    unsafe {
        let result = serializer.process(input);
        println!("Serialized output (vulnerable): {:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::Formatter;

    #[test]
    fn test_process_output() {
        let formatter = Formatter;
        // Provide an input that exceeds the size of a String on most systems.
        let input = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAA"; // 29 bytes (likely > size_of::<String>())
        unsafe {
            let result = formatter.process(input);
            // Expected: input bytes appended with one null terminator.
            let mut expected = input.as_bytes().to_vec();
            expected.push(0);
            assert_eq!(result, expected, "The serialized output does not match the expected C-string format");
        }
    }
}
