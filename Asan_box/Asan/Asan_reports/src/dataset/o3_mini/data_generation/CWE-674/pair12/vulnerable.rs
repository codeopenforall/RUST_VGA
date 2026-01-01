use std::env;
use std::thread;
use std::panic;
pub struct Engine;
impl Engine {
    pub unsafe fn explore(&self, data: *const u8, len: usize, idx: usize) -> i32 {
        if idx >= len {
            return 0;
        }
        let byte = *data.add(idx);
        if byte == b'(' {
            return 1 + self.explore(data, len, idx + 1);
        } else {
            return 0;
        }
    }
    pub fn journey(&self, input: &[u8]) -> i32 {
        unsafe { self.explore(input.as_ptr(), input.len(), 0) }
    }
}
pub fn evaluate(input: &str) -> Result<i32, &'static str> {
    let engine = Engine;
    let inp = input.as_bytes().to_vec(); 
    let handler = thread::spawn(move || {
        engine.journey(&inp)
    });
    match handler.join() {
        Ok(res) => Ok(res),
        Err(_) => Err("Unexpected recursive overflow occurred"),
    }
}
fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "(".repeat(10000));
    match evaluate(&input) {
        Ok(res) => println!("Computation successful, result: {}", res),
        Err(err) => println!("Computation failed with error: {}", err),
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
