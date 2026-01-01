use std::iter::Peekable;
use std::str::Chars;
use std::sync::Arc;
use std::thread;

struct Parser;
impl Parser {
    fn new() -> Self {
        Parser
    }
    unsafe fn recursive_parse(&self, iter: &mut Peekable<Chars>) -> Result<(), String> {
        while let Some(&c) = iter.peek() {
            if c == '(' {
                iter.next();
                self.recursive_parse(iter)?;
            } else if c == ')' {
                iter.next();
                return Ok(());
            } else {
                iter.next();
            }
        }
        Ok(())
    }
    fn parse(&self, input: &str) -> Result<(), String> {
        let mut iter = input.chars().peekable();
        unsafe {
            self.recursive_parse(&mut iter)
        }
    }
}

fn main() {
    let attacker_input = "(".repeat(1000) + &")".repeat(1000);
    let parser = Parser::new();
    let shared_parser = Arc::new(parser);
    let handles: Vec<_> = (0..4).map(|_| {
        let sp = Arc::clone(&shared_parser);
        let input_clone = attacker_input.clone();
        thread::spawn(move || {
            sp.parse(&input_clone)
        })
    }).collect();
    for handle in handles {
        match handle.join() {
            Ok(Ok(())) => println!("Thread finished execution"),
            Ok(Err(e)) => println!("Error: {}", e),
            Err(_) => println!("Thread panicked"),
        }
    }
}

/*
Test Oracle Function (to be compiled and run separately as part of the unit test suite)
This function verifies that the parser gracefully handles deeply nested input.
It spawns a thread to execute the parse function on malicious input.
For the vulnerable version, the thread is expected to panic (or not return an error in a controlled manner),
whereas for the corrected version, the parser returns an error indicating that the recursion depth limit was exceeded.
*/

#[cfg(test)]
mod oracle {
    use std::thread;
    use super::Parser;

    #[test]
    fn test_recursion_depth_limit() {
        // Create an input with extremely deep nesting
        let input = "(".repeat(600) + &")".repeat(600);
        let parser = Parser::new();

        // Spawn a thread to isolate potential panics
        let handle = thread::spawn(move || {
            parser.parse(&input)
        });

        // Join the thread and inspect the result.
        match handle.join() {
            // For the corrected version, the parser should return an Err with the expected message.
            Ok(result) => {
                assert!(result.is_err(), "Expected an error due to recursion depth limit");
                let err = result.unwrap_err();
                assert!(err.contains("Recursion depth limit exceeded"), "Unexpected error message: {}", err);
            },
            // For the vulnerable version, the thread would likely panic, causing the test to fail.
            Err(_) => panic!("Thread panicked: uncontrolled recursion vulnerability detected"),
        }
    }
}
