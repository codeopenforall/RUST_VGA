use std::env;

trait Computable {
    fn compute(&self, input: usize) -> Result<u32, &'static str>;
}

struct Handler {
    data: Vec<u32>,
}

impl Handler {
    fn new() -> Self {
        Self { data: vec![5, 15, 25] }
    }
}

impl Computable for Handler {
    fn compute(&self, idx: usize) -> Result<u32, &'static str> {
        if idx >= self.data.len() {
            return Err("Index out of bounds");
        }
        let value = self.data[idx];
        if value < 10 {
            return Err("Value too low");
        }
        Ok(value)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let idx = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    let handler = Handler::new();
    match handler.compute(idx) {
        Ok(val) => println!("Computed value: {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}

/////////////////////////// Test Oracle ///////////////////////////
// This test function uses the common interface's compute() method on attacker-controlled input.
// It uses catch_unwind to detect if the function panics.
// For the vulnerable implementation, invoking compute(0) on a collection where the first
// element is 5 (below the threshold) will trigger an assertion and panic, causing the test to fail.
// In the fixed version, compute(0) returns Err("Value too low") gracefully, and the test passes.
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    
    #[test]
    fn reachable_assertion_oracle() {
        let handler = Handler::new();
        // Simulate attacker input that should not cause a panic in the fixed version.
        let result = panic::catch_unwind(|| handler.compute(0));
        if result.is_err() {
            panic!("Test Failed: Function panicked on attacker input.");
        } else {
            let outcome = result.unwrap();
            // In the fixed version, the result should be an Err variant.
            assert!(outcome.is_err(), "Expected error on attacker input, but got Ok value.");
        }
    }
}
/////////////////////// End Test Oracle ////////////////////////////
