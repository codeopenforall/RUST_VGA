use std::env;
use std::sync::Arc;
use std::thread;
pub struct Processor {
    data: String,
}
impl Processor {
    pub fn new(data: String) -> Self {
        Processor { data }
    }
    pub fn process(&self, user_len: usize) -> Result<&str, &'static str> {
        if user_len > self.data.len() {
            return Err("Input length exceeds data length");
        }
        let bytes = self.data.as_bytes();
        unsafe {
            Ok(std::str::from_utf8_unchecked(&bytes[..user_len]))
        }
    }
    pub fn run(self, user_len: usize) {
        let shared = Arc::new(self);
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let result = thread_shared.process(user_len).unwrap();
            println!("Processed data: {}", result);
        });
        handle.join().unwrap();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_string> <length>", args[0]);
        std::process::exit(1);
    }
    let input = args[1].clone();
    let user_len: usize = args[2].parse().unwrap();
    let proc_inst = Processor::new(input);
    proc_inst.run(user_len);
}

//////////////////////////////
// Test Oracle Function for verifying the vulnerability fix.
// This test oracle is designed to be compiled separately (as part of a cargo test or similar)
// and linked with either the vulnerable or fixed version individually.
// When compiled with the vulnerable version, the test is expected to catch a panic
// resulting from an out-of-bound slice request.
// When compiled with the fixed version, the test should receive an Err result.

#[cfg(test)]
mod tests {
    use super::Processor;
    use std::panic;

    // The test input is chosen such that "hello" length is 5.
    // An invalid length value (e.g., 10) should trigger different behavior:
    // - In the vulnerable build, calling process(10) should panic at runtime.
    // - In the corrected build, process(10) should return Err("Input length exceeds data length").
    #[test]
    fn oracle() {
        let test_input = "hello".to_string();
        let invalid_length = 10;

        // Test behavior for the current build.
        // Use catch_unwind to detect panic in the vulnerable version.
        let proc_inst = Processor::new(test_input.clone());
        let vulnerable_outcome = panic::catch_unwind(|| {
            // Regardless of the return type, calling process with an invalid length should not succeed.
            let _ = proc_inst.process(invalid_length);
        });
        
        if vulnerable_outcome.is_err() {
            // In the vulnerable version, a panic is expected.
            // So the test should fail.
            panic!("Test failed: Vulnerable version panicked on invalid input.");
        } else {
            // If no panic occurs, then we expect a well-formed error result (from the fixed version).
            let proc_inst_fixed = Processor::new(test_input);
            match proc_inst_fixed.process(invalid_length) {
                Ok(_) => panic!("Test failed: Expected an error for invalid input in fixed version."),
                Err(msg) => assert_eq!(msg, "Input length exceeds data length", "Unexpected error message"),
            }
        }
    }
}
