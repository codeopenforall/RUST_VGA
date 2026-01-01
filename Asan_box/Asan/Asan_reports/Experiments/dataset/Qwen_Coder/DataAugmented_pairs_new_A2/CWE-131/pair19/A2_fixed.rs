use std::ptr;
pub struct Formatter;
impl Formatter {
    pub unsafe fn process(&self, input: &str) -> Vec<u8> {
        let buf_size = input.as_bytes().len() + 1;
        let mut buffer = Vec::with_capacity(buf_size);
        let bytes = input.as_bytes();
        let dst = buffer.as_mut_ptr();
        ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        *dst.add(bytes.len()) = 0;
        buffer.set_len(buf_size);
        buffer
    }
}
fn main() {
    let formatter = Formatter;
    let input = "This is a very long input string that will properly be handled.";
    unsafe {
        let result = formatter.process(input);
        println!("Serialized output (fixed): {:?}", result);
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
