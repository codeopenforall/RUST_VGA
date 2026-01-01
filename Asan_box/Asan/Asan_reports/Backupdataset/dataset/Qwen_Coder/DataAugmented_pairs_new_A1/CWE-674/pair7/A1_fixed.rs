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