use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    value: i32,
    next: Option<Box<Data>>,
}
struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser { input: input.as_bytes(), pos: 0 }
    }
    fn current_byte(&self) -> Option<u8> {
        unsafe {
            if self.pos < self.input.len() {
                Some(*self.input.get_unchecked(self.pos))
            } else {
                None
            }
        }
    }
    fn process_nested(&mut self, depth: usize) -> Result<i32, &'static str> {
        if depth > 1000 {
            return Err("Recursion depth limit exceeded");
        }
        let mut acc = 0;
        while let Some(byte) = self.current_byte() {
            self.pos += 1;
            match byte {
                b'(' => {
                    let inner_val = self.process_nested(depth + 1)?;
                    acc += inner_val + 1;
                }
                b')' => {
                    return Ok(acc);
                }
                _ => continue,
            }
        }
        Ok(acc)
    }
}
fn execute(input: &str) -> Result<i32, &'static str> {
    let mut parser = Parser::new(input);
    parser.process_nested(0)
}
fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "( ( ( ( ( ) ) ) ) )".to_string());
    let handle = thread::spawn(move || {
        let _ = execute(&input).unwrap();
    });
    handle.join().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    // Oracle function that verifies the behavior using deeply nested input.
    #[test]
    fn recursion_limit_test() {
        // Create deeply nested input to trigger potential overflow.
        let deep_input = "(".repeat(1100) + &")".repeat(1100);

        // Test the version with uncontrolled recursion; expect a panic/stack overflow.
        let err = std::panic::catch_unwind(|| {
            let _ = execute(&deep_input).unwrap();
        });
        assert!(err.is_err(), "Expected failure in uncontrolled recursion scenario");

        // Test the secure version; it should gracefully error out with a controlled message.
        let result = execute(&deep_input);
        assert!(result.is_err(), "Expected controlled error due to recursion depth limit");
        if let Err(msg) = result {
            assert_eq!(msg, "Recursion depth limit exceeded", "Expected specific error message");
        }
    }
}
