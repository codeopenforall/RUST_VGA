struct Config {
    data: Vec<i32>,
}

impl Config {
    pub fn new() -> Self {
        Config { data: vec![1, 2, 3, 4, 5] }
    }

    pub unsafe fn process(&self, index: &str) -> Result<i32, String> {
        let idx: usize = match index.parse() {
            Ok(num) => num,
            Err(_) => return Err("Invalid index".to_string()),
        };

        if idx >= self.data.len() {
            return Err("Index out of bounds".to_string());
        }

        Ok(self.data[idx])
    }
}

//////////////////////////////////////////////////////////////
// Test Oracle:
//
// This unit test attempts to process user input that is out-of-bounds.
// For the fixed code, the call should return an error indicating that the
// index is out of bounds. In contrast, the vulnerable code would perform
// unsafe pointer arithmetic and may yield an unexpected result or undefined behavior.
// This test strictly checks for the error condition.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import necessary definitions from the parent module.
    use super::*;

    #[test]
    fn test_bad_input() {
        let config = Config::new();
        // Provide an input that is clearly out-of-bounds (e.g., "10" for a vector of length 5).
        let result = unsafe { config.process("10") };
        // Expect an error for the out-of-range index.
        assert!(result.is_err(), "Expected an error for out-of-bounds index");
        // Optional: check that the error message is as expected.
        if let Err(e) = result {
            assert_eq!(e, "Index out of bounds".to_string(), "Error message must indicate bounds violation");
        }
    }
}
