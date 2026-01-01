use std::thread;
const MAX_DEPTH: usize = 100;
struct Parser<'a> {
    data: &'a [u8],
    pos: usize,
}
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser {
            data: input.as_bytes(),
            pos: 0,
        }
    }
    fn descend(&mut self, depth: usize) -> Result<(), String> {
        if depth > MAX_DEPTH {
            return Err("Maximum recursion depth reached".into());
        }
        if self.pos >= self.data.len() {
            return Ok(());
        }
        let ptr = self.data.as_ptr().wrapping_add(self.pos);
        let ch = unsafe { *ptr as char };
        match ch {
            '(' => {
                self.pos += 1;
                self.descend(depth + 1)?;
                if self.pos < self.data.len() && self.data[self.pos] as char == ')' {
                    self.pos += 1;
                    Ok(())
                } else {
                    Err("Missing closing parenthesis".into())
                }
            }
            _ => {
                self.pos += 1;
                self.descend(depth)
            }
        }
    }
}
fn process(input: &str) -> Result<(), String> {
    let mut parser = Parser::new(input);
    parser.descend(0)
}
fn parser_run(input: &str) -> Result<(), String> {
    process(input)
}
fn main() {
    let input = "(".repeat(1000) + &")".repeat(1000);
    let handle = thread::spawn(move || {
        match parser_run(&input) {
            Ok(()) => println!("Parsing succeeded"),
            Err(e) => println!("Parsing failed: {}", e),
        }
    });
    let _ = handle.join();
}


///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Assume the common parser_run function is available from the module.
    // The same function is used by both versions.
    use crate::parser_run;

    #[test]
    fn recursion_limit_oracle() {
        // Create a deeply nested input that would trigger uncontrolled recursion.
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| {
            parser_run(&deep_input)
        });
        // In the vulnerable version, this input causes a stack overflow (panic),
        // so catch_unwind would capture an error. In the fixed version, the panic
        // is avoided and the parser returns a controlled error.
        assert!(result.is_ok(), "Program panicked due to uncontrolled recursion");
        let parse_result = result.unwrap();
        // The contract is that deeply nested input should result in a controlled error.
        assert!(parse_result.is_err(), "Expected an error due to excessive recursion depth");
        assert_eq!(parse_result.unwrap_err(), "Maximum recursion depth reached");
    }
}
