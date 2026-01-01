use std::env;

struct Config {
    data: Vec<u32>,
}

impl Config {
    fn new() -> Self {
        Self { data: vec![10, 20, 30, 40, 50] }
    }

    unsafe fn process(&self, input: &str) -> Result<u32, String> {
        let idx: usize = input.trim().parse().map_err(|_| "Invalid input: not a number".to_string())?;
        if idx >= self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        let ptr = self.data.as_ptr();
        let value = *ptr.add(idx);
        Ok(value)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "2" };
    let config = Config::new();
    let res = unsafe { config.process(input) };
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
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
