use std::env;
use std::ptr;
struct Processor;
impl Processor {
    unsafe fn traverse(&self, data: *const u8, len: usize) -> Result<usize, String> {
        if len == 0 {
            return Ok(0);
        }
        let first = ptr::read(data);
        if first == b'(' {
            let inner_depth = self.traverse(data.add(1), len - 1)?;
            if len >= inner_depth + 2 && ptr::read(data.add(inner_depth + 1)) == b')' {
                Ok(inner_depth + 1)
            } else {
                Err("Missing closing parenthesis".to_string())
            }
        } else {
            Err("Unexpected token encountered".to_string())
        }
    }
    pub fn process(&self, input: &str) -> Result<usize, String> {
        if input.len() > 1000 {
            return Err("Recursion limit reached".to_string());
        }
        unsafe { self.traverse(input.as_ptr(), input.len()) }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "(())" };
    let proc = Processor;
    match proc.process(input) {
        Ok(depth) => println!("Computed depth: {}", depth),
        Err(e) => eprintln!("Error: {}", e),
    }
}

////////////////////////////////////////////
// Test Oracle for Recursion Safety
// This test function builds a deeply nested input string (more than MAX_DEPTH)
// and invokes the 'process' method from the Processor.
// For the vulnerable version, the test is expected to panic (or unwind) due to a stack overflow.
// For the fixed version, the function should return an error with the message "Recursion limit reached".
// This test uses std::panic::catch_unwind to capture panics.
////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Processor;
    use std::panic;

    #[test]
    fn recursion_oracle() {
        // Create a deeply nested input string (exceeds the safe recursion limit).
        let depth = 1100;
        let input = "(".repeat(depth) + &")".repeat(depth);
        let proc = Processor;

        let result = panic::catch_unwind(|| proc.process(&input));

        match result {
            // For the fixed version, we expect a graceful error.
            Ok(Ok(err)) => {
                // If the process function erroneously returns Ok(_) when it should error, fail the test.
                panic!("Expected an error, but got a depth value: {}", err);
            },
            Ok(Err(e)) => {
                // Fixed version: should produce an explicit error.
                assert_eq!(e, "Recursion limit reached", "Expected recursion limit error");
            },
            // For the vulnerable version, the function is expected to panic.
            Err(_) => {
                panic!("Function panicked due to uncontrolled recursion");
            }
        }
    }
}
