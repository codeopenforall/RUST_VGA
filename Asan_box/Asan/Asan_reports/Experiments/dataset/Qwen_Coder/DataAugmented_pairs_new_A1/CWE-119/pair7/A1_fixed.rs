use std::ptr;
struct Data {
    buffer: [u8; 64],
    magic: u32,
}
impl Data {
    pub fn new() -> Self {
        Self {
            buffer: [0; 64],
            magic: 0xDEADBEEF,
        }
    }
    pub fn operate(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input too large");
        }
        unsafe {
            let dest = self.buffer.as_mut_ptr();
            for i in 0..input.len() {
                *dest.add(i) = input[i];
            }
        }
        Ok(())
    }
    pub fn check(&self) -> bool {
        self.magic == 0xDEADBEEF
    }
}
fn main() {
    let mut obj = Data::new();
    let input = vec![1u8; 100];
    let _ = obj.operate(&input);
    println!("Integrity check: {}", obj.check());
}

//////////////////// Oracle Test ////////////////////
// This oracle function is intended to serve as a single reproducible test.
// It verifies that processing an input that exceeds the internal buffer's boundary
// results in an error. In the vulnerable version, the operation would erroneously succeed
// (and corrupt internal memory), causing the test to fail.
#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn test_oracle() {
        let mut obj = Data::new();
        let input = vec![1u8; 100]; // Oversized input
        let result = obj.operate(&input);
        // The expected correct behavior is for the operation to return an error.
        assert!(result.is_err(), "Operation must reject oversized input");
    }
}
