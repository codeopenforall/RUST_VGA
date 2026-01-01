use std::ptr;
struct BufferProcessor {
    buffer: [u8; 10],
}
impl BufferProcessor {
    fn new() -> Self {
        BufferProcessor { buffer: [0; 10] }
    }
    fn update(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input exceeds buffer capacity");
        }
        unsafe {
            let dest = self.buffer.as_mut_ptr();
            ptr::copy_nonoverlapping(input.as_ptr(), dest, input.len());
        }
        Ok(())
    }
    fn read(&self) -> &[u8] {
        &self.buffer
    }
}
fn main() {
    let mut processor = BufferProcessor::new();
    let data = b"HELLO"; 
    match processor.update(data) {
        Ok(()) => println!("Content: {:?}", processor.read()),
        Err(e) => println!("Error: {}", e),
    }
}

/////////////////////////////////////////////////////////////
// The following test oracle function serves as a unit test.
// It verifies that when a too-long input is provided, the safe
// implementation rejects the operation by returning an error,
// whereas the insecure implementation (if unaltered) would
// incorrectly process the input and potentially corrupt memory.
//
// Note: This test oracle is designed to be compiled separately
// and should not be included in the main vulnerable or fixed code.
/////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_memory_safety() {
        let mut instance = BufferProcessor::new();
        let input = b"012345678901"; // 12 bytes, exceeds the 10-byte capacity

        let result = instance.update(input);

        // In the secure implementation, the update call returns an error.
        // In the insecure implementation, it returns Ok(()), indicating a flaw.
        match result {
            Ok(_) => {
                // For an insecure implementation, the internal buffer will be
                // overwritten unsafely. We further validate by comparing the first
                // 10 bytes of the input with what was stored.
                let expected = &input[0..10];
                assert_ne!(
                    instance.read(),
                    expected,
                    "Memory bounds violation: the internal state was updated insecurely."
                );
            }
            Err(_) => {
                // Secure behavior: an error is reported as expected.
                assert!(true);
            }
        }
    }
}
