use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum Ast {
    Node(Box<Ast>),
    Leaf(char),
}

struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
        }
    }

    fn parse_expr(&mut self) -> Result<Ast, String> {
        if self.pos >= self.input.len() {
            return Err("unexpected end".into());
        }
        let c = unsafe { *self.input.as_ptr().add(self.pos) };
        self.pos += 1;
        if c == '(' {
            let node = self.parse_expr()?; 
            if self.pos >= self.input.len() {
                return Err("expected ')' but found end".into());
            }
            let c2 = unsafe { *self.input.as_ptr().add(self.pos) };
            if c2 != ')' {
                return Err("expected ')'".into());
            }
            self.pos += 1;
            Ok(Ast::Node(Box::new(node)))
        } else {
            Ok(Ast::Leaf(c))
        }
    }
}

fn execute(input: &str) -> Result<Ast, String> {
    let mut parser = Parser::new(input);
    parser.parse_expr()
}

fn main() {
    let input = "((".to_owned() + &"(".repeat(2000) + "x" + &")".repeat(2000) + ")";
    let result_arc = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result_arc);
    let input_clone = input.clone();
    let handle = thread::spawn(move || {
        let res = execute(&input_clone);
        let mut guard = result_clone.lock().unwrap();
        *guard = Some(res);
    });
    handle.join().unwrap();
    let guard = result_arc.lock().unwrap();
    match &*guard {
        Some(Ok(ast)) => println!("Parsed AST: {:?}", ast),
        Some(Err(e)) => println!("Error during parsing: {}", e),
        None => println!("No result produced"),
    }
}

//////////////////////////////////////////
// Test Oracle for Recursion Vulnerability //
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test uses an extremely nested input to trigger the recursion.
    // For the corrected version it should produce an error with a specific message,
    // while the vulnerable version will panic (stack overflow) and thus fail the test.
    #[test]
    fn test_recursion_limit() {
        // Create a deeply nested input designed to exceed MAX_DEPTH.
        let deep_input = "(".to_owned() + &"(".repeat(1500) + "x" + &")".repeat(1500) + ")";
        let result = panic::catch_unwind(|| execute(&deep_input));
        match result {
            Ok(Ok(_)) => panic!("Expected error due to recursion limit being exceeded, but got successful parse."),
            Ok(Err(e)) => assert_eq!(e, "recursion limit exceeded"),
            Err(_) => panic!("Function panicked, which reflects the vulnerability in the uncontrolled recursion case."),
        }
    }
}
