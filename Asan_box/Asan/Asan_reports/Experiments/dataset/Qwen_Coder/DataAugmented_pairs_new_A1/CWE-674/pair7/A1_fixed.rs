#![allow(dead_code)]
use std::env;
use std::vec::Vec;

struct Node {
    children: Vec<Node>,
}

impl Node {
    unsafe fn new() -> Self {
        Node { children: Vec::new() }
    }
}

struct Parser {
    input: Vec<char>,
    pos: usize,
    max_depth: usize,
    current_depth: usize,
}

impl Parser {
    fn new(input_str: &str) -> Self {
        Parser {
            input: input_str.chars().collect(),
            pos: 0,
            max_depth: 1000, // Set a reasonable limit for recursion depth
            current_depth: 0,
        }
    }

    fn parse_expr(&mut self) -> Result<Node, &'static str> {
        self.parse_nested()
    }

    fn parse_nested(&mut self) -> Result<Node, &'static str> {
        if self.pos >= self.input.len() {
            return Err("Unexpected end of input");
        }

        if self.current_depth >= self.max_depth {
            return Err("Maximum recursion depth exceeded");
        }

        let mut node;
        unsafe {
            node = Node::new();
        }

        if self.input[self.pos] == '(' {
            self.pos += 1;
            self.current_depth += 1;

            while self.pos < self.input.len() && self.input[self.pos] == '(' {
                node.children.push(self.parse_nested()?);
            }

            if self.pos < self.input.len() && self.input[self.pos] == ')' {
                self.pos += 1;
            } else {
                return Err("Missing closing parenthesis");
            }

            self.current_depth -= 1;
        }

        Ok(node)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input>", args[0]);
        return;
    }
    let mut parser = Parser::new(&args[1]);
    match parser.parse_expr() {
        Ok(_node) => println!("Parsed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}

///////////////////////////////
// Test Oracle Code
///////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    // Depending on which version is being tested, the parser's behavior differs.
    // For the vulnerable version, deep nesting is expected to cause a stack overflow
    // (i.e. a panic), while for the fixed version, the error "Maximum recursion depth exceeded"
    // should be returned.
    //
    // To keep the test deterministic, we use a fixed deep nested input.
    //
    // IMPORTANT: This test oracle is separate from the library/application code.
    use crate::Parser;

    #[test]
    fn oracle_test() {
        // Create an input with over 1100 nested '(' then matching ')' to trigger deep recursion.
        let deep_input = format!("{}{}", "(".repeat(1100), ")".repeat(1100));

        // Execute the parsing inside a catch_unwind to capture potential panics.
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut parser = Parser::new(&deep_input);
            parser.parse_expr()
        }));

        match result {
            // For the fixed version, we expect a graceful error.
            Ok(Err(e)) => {
                assert_eq!(e, "Maximum recursion depth exceeded", "Expected maximum depth error");
            },
            // For the vulnerable version, a panic might occur due to stack overflow.
            Ok(Ok(_node)) => panic!("Expected an error due to deep recursion, but parsing succeeded."),
            Err(_) => panic!("Parsing caused a panic (stack overflow) in the vulnerable version."),
        }
    }
}
