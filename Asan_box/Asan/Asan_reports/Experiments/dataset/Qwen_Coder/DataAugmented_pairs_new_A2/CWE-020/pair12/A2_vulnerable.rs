use std::env;
use std::process;

trait Operation {
    fn execute(&self, input: &str) -> Result<u8, &'static str>;
}

struct Handler {
    id: u32,
}

impl Operation for Handler {
    fn execute(&self, input: &str) -> Result<u8, &'static str> {
        let index: usize = input.parse().unwrap();
        let mut data: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // Introducing a CWE-119 vulnerability by removing the bounds check
        unsafe {
            let ptr = data.as_mut_ptr();
            *ptr.add(index) = 99;
        }
        
        let sum: u8 = data.iter().fold(0, |acc, &x| acc.wrapping_add(x));
        Ok(sum)
    }
}

fn run() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "3" };
    let handler = Handler { id: 1 };
    match handler.execute(input) {
        Ok(res) => println!("Result: {}", res),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    run();
}

/*
   Unit Test Oracle: This test function verifies that when a malicious input (e.g., "15") 
   is provided, the corrected version returns an error due to the index being out-of-range.
   In the vulnerable version, the absence of proper checks may result in an incorrect memory write.
   When running the test on the vulnerable version the test will fail, 
   while it will pass when executed on the corrected version.
*/
#[cfg(test)]
mod oracle_test {
    use super::*;

    #[test]
    fn test_invalid_input() {
        // Use an out-of-bound index "15" to trigger the vulnerability condition.
        let handler = Handler { id: 42 };
        let resp = handler.execute("15");
        assert!(resp.is_err(), "Expected error for an out-of-bound index input");
    }
}
