use std::env;
use std::process;
use std::thread;
use std::sync::Arc;

struct Calculator;

impl Calculator {
    fn execute(&self, denominator: i32) -> Result<i32, &'static str> {
        if denominator == 0 {
            Err("Division by zero encountered")
        } else {
            Ok(100 / denominator)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let calc = Calculator;
    let data = Arc::new(input);
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        *data_clone
    });
    let thread_val = handle.join().unwrap();
    match calc.execute(thread_val) {
        Ok(result) => println!("Output: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

////////////////////////////////////////////////////////////
// Test Oracle: Ensures that the zero denominator case
// is handled properly.
//
// This test calls the execute() function with a zero
// denominator. For the vulnerable version, the unsafe
// division will panic, causing the test to fail.
// For the fixed version, the function should return an
// error result, thereby making the test pass.
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn oracle() {
        let calc = Calculator;
        // Execute with a zero divisor.
        let res = panic::catch_unwind(|| {
            calc.execute(0)
        });
        // The fixed version should not panic.
        assert!(res.is_ok(), "Function should not panic on zero input");
        let result = res.unwrap();
        // For a zero denominator, the fixed version returns an error.
        assert!(result.is_err(), "Fixed version should return an error on division by zero");
    }
}
