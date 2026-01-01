use std::sync::Arc;
use std::thread;

struct Processor {
    factor: i32,
}

impl Processor {
    fn initialize(factor: i32) -> Self {
        Processor { factor }
    }

    fn compute(&self, divisor_str: &str) -> i32 {
        let divisor: i32 = divisor_str.parse().unwrap_or_default();
        unsafe {
            let ptr = &self.factor as *const i32;
            let value = *ptr;
            value / divisor
        }
    }
}

pub fn execute(divisor_str: &str) -> Result<i32, String> {
    let processor = Processor::initialize(100);
    Ok(processor.compute(divisor_str))
}

fn main() {
    let processor = Arc::new(Processor::initialize(100));
    let arg = std::env::args().nth(1).unwrap_or_default();
    let mut threads = Vec::new();
    for _ in 0..4 {
        let processor = Arc::clone(&processor);
        let arg_clone = arg.clone();
        threads.push(thread::spawn(move || {
            let res = processor.compute(&arg_clone);
            println!("Result: {}", res);
        }));
    }
    for thread in threads {
        thread.join().expect("Thread panicked");
    }
}

//////////////////////////////////////////
// Test Oracle
// This test function serves as a single contract for both versions.
// When executing against the vulnerable version, the test will fail because
// the invalid input ("abc") causes a panic (or unintended behavior).
// When executing against the fixed version, the test passes by properly returning an error.
//////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::panic;
    // The API function 'perform' is expected to be present in both versions.
    use super::perform;

    #[test]
    fn test_oracle() {
        // Test with valid input: expect a successful computation.
        {
            let res = panic::catch_unwind(|| perform("42"));
            assert!(res.is_ok(), "Valid input caused a panic unexpectedly");
            let value = res.unwrap().unwrap();
            assert_eq!(value, 100 / 42, "Incorrect computation for valid input");
        }
        // Test with invalid input (non-numeric).
        // In the fixed version, this should return an error.
        // In the vulnerable version, it is likely to panic or produce unintended behavior.
        {
            let res = panic::catch_unwind(|| perform("abc"));
            if let Ok(result) = res {
                // For the fixed version, result should be Err.
                assert!(
                    result.is_err(),
                    "Expected error for non-numeric input, but got success: {:?}",
                    result.ok()
                );
            } else {
                panic!("Non-numeric input caused a panic, indicating the vulnerability.");
            }
        }
    }
}
