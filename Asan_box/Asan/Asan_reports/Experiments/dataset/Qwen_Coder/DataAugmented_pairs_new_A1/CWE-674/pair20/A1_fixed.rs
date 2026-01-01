use std::env;
use std::process;

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    unsafe fn parse_recursive(&mut self) -> Result<(), String> {
        const MAX_RECURSION_DEPTH: usize = 1000;
        if self.pos >= self.input.len() {
            return Ok(());
        }
        if self.pos > MAX_RECURSION_DEPTH {
            return Err("Recursion too deep".into());
        }
        let current = self.input.as_bytes()[self.pos];
        if current == b'(' {
            self.pos += 1;
            self.parse_recursive()?; 
            if self.pos < self.input.len() && self.input.as_bytes()[self.pos] == b')' {
                self.pos += 1;
                Ok(())
            } else {
                Err("Missing closing ')'".into())
            }
        } else {
            self.pos += 1;
            Ok(())
        }
    }

    pub unsafe fn execute(input: &str) -> Result<(), String> {
        let mut parser = Parser {
            input: input.to_string(),
            pos: 0,
        };
        parser.parse_recursive()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = if args.len() > 1 { &args[1] } else { "(a)" };
    unsafe {
        match Parser::execute(data) {
            Ok(_) => println!("Parsing succeeded."),
            Err(e) => {
                eprintln!("Parsing failed: {}", e);
                process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    
    // This test oracle is designed to be run against both the vulnerable and corrected implementations.
    // It provides a deeply nested input that exceeds the safe recursion limit.
    // For the vulnerable version, the uncontrolled recursion is expected to cause a panic (stack overflow),
    // whereas the fixed version should gracefully return an error ("Recursion too deep").
    #[test]
    fn oracle() {
        // Create a deeply nested input: 1100 pairs of parentheses.
        let nested = "(".repeat(1100) + &")".repeat(1100);
        
        // Catch unwinding to differentiate between a panic and a returned error.
        let result = panic::catch_unwind(|| {
            // Unsafely call the execute API provided by the parser.
            // This function should either return a Result or panic.
            unsafe { super::Parser::execute(&nested) }
        });
        
        match result {
            Ok(res) => {
                // In the fixed version, the recursion depth check should trigger.
                match res {
                    Ok(_) => panic!("Expected failure due to deep recursion, but parsing succeeded."),
                    Err(e) => {
                        assert_eq!(e, "Recursion too deep", "Expected recursion depth limit error.");
                    }
                }
            },
            Err(_) => panic!("Function panicked due to uncontrolled recursion."),
        }
    }
}
