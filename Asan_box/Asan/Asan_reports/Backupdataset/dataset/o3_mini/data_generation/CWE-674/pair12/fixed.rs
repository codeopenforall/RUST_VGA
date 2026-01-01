use std::env;
use std::thread;
pub struct Engine;
impl Engine {
    pub unsafe fn traverse(&self, data: *const u8, len: usize, idx: usize, depth: usize, max_depth: usize) -> Result<i32, &'static str> {
        if depth > max_depth {
            return Err("Maximum recursion depth exceeded");
        }
        if idx >= len {
            return Ok(0);
        }
        let byte = *data.add(idx);
        if byte == b'(' {
            let next = self.traverse(data, len, idx + 1, depth + 1, max_depth)?;
            Ok(1 + next)
        } else {
            Ok(0)
        }
    }
    pub fn process(&self, input: &[u8], max_depth: usize) -> Result<i32, &'static str> {
        unsafe { self.traverse(input.as_ptr(), input.len(), 0, 0, max_depth) }
    }
}
pub fn evaluate(input: &str) -> Result<i32, &'static str> {
    let engine = Engine;
    let inp = input.as_bytes().to_vec();
    let max_depth = 1000;
    let handler = thread::spawn(move || {
        engine.process(&inp, max_depth)
    });
    match handler.join() {
        Ok(res) => res,
        Err(_) => Err("Thread panicked unexpectedly"),
    }
}
fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "(".repeat(10000));
    match evaluate(&input) {
        Ok(res) => println!("Processing successful, result: {}", res),
        Err(err) => println!("Processing failed with error: {}", err),
    }
}


//////////////////////////////
// Test Oracle Functionality
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::evaluate;
    use std::panic;

    // This test uses a deeply nested input string designed to trigger recursion limits.
    // For the fixed version, the error "Maximum recursion depth exceeded" should be returned,
    // while the vulnerable version would panic with a stack overflow.
    #[test]
    fn test_deep_input() {
        let deep_input = "(".repeat(5000);
        // Use catch_unwind to intercept panics.
        let result = panic::catch_unwind(|| {
            evaluate(&deep_input)
        });
        
        match result {
            // If the call did not panic, unwrap the result.
            Ok(eval_result) => {
                match eval_result {
                    // For the fixed version, we expect a controlled error with the specific message.
                    Err(err_msg) => assert_eq!(err_msg, "Maximum recursion depth exceeded", "Fixed version should return the recursion depth error"),
                    Ok(val) => panic!("Expected an error due to deep recursion, but got a value: {}", val),
                }
            },
            // If the code panics, that's considered a failure for our regression test.
            Err(_) => panic!("The evaluation panicked, indicating uncontrolled recursion (vulnerable behavior)")
        }
    }
}
