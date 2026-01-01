use std::panic;

struct MemoryBlock {
    buffer: Box<[i32]>,
}

impl MemoryBlock {
    fn new() -> Self {
        Self {
            buffer: vec![10, 20, 30, 40, 50].into_boxed_slice(),
        }
    }

    fn validated_sum(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        if offset + count > self.buffer.len() {
            return Err("Invalid offset or count");
        }
        unsafe {
            let ptr = self.buffer.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, count);
            Ok(slice.iter().sum())
        }
    }

    fn safe_entry(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        self.validated_sum(offset, count)
    }
}

fn main() {
    let block = MemoryBlock::new();
    let valid = block.safe_entry(1, 3);
    println!("Valid call result: {:?}", valid);
    let invalid = block.safe_entry(2, 4);
    println!("Invalid call result: {:?}", invalid);
}

//////////////////////////// Test Oracle ////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_entry() {
        let block = MemoryBlock::new();
        // Valid parameters should return the correct sum.
        let valid = block.safe_entry(1, 3);
        assert_eq!(valid, Ok(90));

        // Invalid parameters: offset + count exceeds the buffer length.
        // In the fixed version, this should return an error.
        let invalid = block.safe_entry(2, 4);
        assert!(invalid.is_err(), "Expected error for invalid parameters");

        // For vulnerable code, the same test would either not return an error or could panic,
        // causing this test to fail.
    }
}
//////////////////////////// End of Test Oracle ////////////////////////////
