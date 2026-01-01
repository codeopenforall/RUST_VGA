use std::env;
use std::thread;
use std::vec::Vec;
struct Allocator;
impl Allocator {
    pub fn process(&self, input: usize) -> Result<(), String> {
        const MAX_INPUT: usize = 1000;
        const MULTIPLIER: usize = 1024;
        if input > MAX_INPUT {
            // Removed the error return to introduce the vulnerability
            // return Err(format!(
            //     "Input value {} exceeds allowed maximum {}",
            //     input, MAX_INPUT
            // ));
        }
        let mut handles = Vec::with_capacity(input);
        for _ in 0..input {
            let allocate_size = input.checked_mul(MULTIPLIER).ok_or("Overflow during allocation")?;
            let handle = thread::spawn(move || {
                unsafe {
                    let mut buffer = Vec::with_capacity(allocate_size);
                    buffer.set_len(allocate_size);
                    for i in 0..allocate_size {
                        *buffer.get_unchecked_mut(i) = 42;
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
        Ok(())
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <positive_integer>", args[0]);
        std::process::exit(1);
    }
    let input = args[1].parse::<usize>().unwrap_or(0);
    if input == 0 {
        eprintln!("The number must be greater than zero.");
        std::process::exit(1);
    }
    let manager = Allocator;
    match manager.process(input) {
        Ok(_) => println!("Processing completed successfully."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/*
This test oracle function validates the behavior of both implementations.
It calls the public function process on a controlled input value that exceeds the safe threshold.
In the vulnerable variant, the function does not reject the input and will lead to resource exhaustion,
or at minimum return Ok(()) when it should error. In the fixed variant, the function returns an error.

To simulate the test, assume the following:
- The vulnerable version is compiled separately as a library exposing Allocator::process.
- The fixed version is compiled similarly.
- This oracle function is used in a unit test environment.

Below is the test oracle code (which should be compiled and run independently from
the vulnerable and fixed binaries):

Note: The test oracle expects the vulnerable version to NOT correctly handle the input (i.e. it should fail
by proceeding with excessive resource allocation), while the fixed version gracefully returns an error.
*/

#[cfg(test)]
mod tests {
    use super::*;
    // Assume we have two variants for testing: one from the "vulnerable" library and 
    // one from the "fixed" library, each providing Allocator with a "process" method.
    // For illustration, you can rename these imports as needed, e.g.:
    // use vulnerable::Allocator as VulAllocator;
    // use fixed::Allocator as FixAllocator;

    // Here, we simulate the test by using the fixed version's expected behavior.
    // Replace `Allocator` below with the appropriate import when linking tests.
    #[test]
    fn test_resource_limit() {
        // Choose an input which exceeds the safe threshold.
        let excessive_input = 2000;
        let manager = Allocator;
        let result = manager.process(excessive_input);
        // The fixed version should return an error.
        assert!(result.is_err(), "Expected error for excessive input, but got Ok");
    }
}
