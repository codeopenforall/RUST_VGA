struct Buffer {
    buffer: Vec<i32>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            buffer: vec![0; size],
        }
    }

    pub fn process(&mut self, value: i32) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(0) = value; // Intentionally using index 0 for demonstration
        }
        assert!(self.buffer[0] < 1000, "Invariant violated: value must be less than 1000");
    }
}

pub fn process(value: i32) {
    let mut buffer = Buffer::new(1);
    buffer.process(value);
}

/*
This test oracle function is designed to confirm the exploitability of the vulnerability in the vulnerable version 
and its remediation in the fixed version. The test calls the process() function with a deliberately invalid input (1500)
that should trigger a panic in the vulnerable variant, while the fixed variant should complete without panicking.

When linked with the vulnerable code, the test should fail because a panic occurs. Conversely, when linked with the fixed 
version, the test should pass.
*/
#[cfg(test)]
mod tests {
    use std::panic;

    // Import the process function.
    use super::process;

    #[test]
    fn oracle_test() {
        let result = panic::catch_unwind(|| {
            process(1500);
        });
        // In a secure implementation, the processing with invalid input should not panic.
        assert!(result.is_ok(), "Test oracle failed: operation panicked due to attacker-controlled input.");
    }
}
