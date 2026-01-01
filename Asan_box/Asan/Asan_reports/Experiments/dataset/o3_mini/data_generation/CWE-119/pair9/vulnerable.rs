#![allow(dead_code)]
use std::ptr;
pub fn do_process(input: &[u8]) -> Result<u32, &'static str> {
    let mut buf = [0u8; 16];
    unsafe {
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), input.len());
    }
    let sum: u32 = buf.iter().map(|&b| b as u32).sum();
    Ok(sum)
}
fn main() {
    let input = vec![1u8; 20]; 
    match do_process(&input) {
        Ok(s) => println!("Sum computed: {}", s),
        Err(e) => println!("Error encountered: {}", e),
    }
}


//////////////////////////////////////////////
// Test Oracle: Verifying Memory Safety
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the processing function; for the vulnerable build this will be the unsafe version,
    // for the corrected build the version with proper bounds checking.
    use super::do_process;

    #[test]
    fn test_memory_bounds() {
        // Triggering input: 20 bytes, which is above the internal buffer capacity of 16 bytes.
        let input = vec![1u8; 20];

        // The correct behavior is to reject the oversized input and return an error.
        let result = do_process(&input);

        // Assert that an error is returned; if we receive a valid sum, the vulnerability is present.
        assert!(result.is_err(), "Expected error on oversized input, but got a valid result");
    }
}
