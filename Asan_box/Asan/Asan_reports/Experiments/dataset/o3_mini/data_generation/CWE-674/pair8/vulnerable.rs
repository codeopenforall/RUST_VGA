use std::env;
#[derive(Debug)]
pub struct Node {
    pub kind: char,
    pub children: Vec<Node>,
}
pub struct Parser {
    data: String,
    pos: usize,
}
impl Parser {
    pub fn new(data: String) -> Self {
        Parser { data, pos: 0 }
    }
    pub unsafe fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.data.len() {
            return None;
        }
        let ptr = self.data.as_ptr().add(self.pos);
        self.pos += 1;
        Some(*ptr as char)
    }
    pub fn parse_expr(&mut self) -> Result<Node, String> {
        let ch = unsafe { self.next_char() }.ok_or("Unexpected end of input")?;
        if ch == '(' {
            let mut children = Vec::new();
            loop {
                if self.pos < self.data.len() {
                    let peek = unsafe { *self.data.as_ptr().add(self.pos) as char };
                    if peek == ')' {
                        unsafe { self.next_char() };
                        break;
                    }
                } else {
                    return Err("Missing closing parenthesis".into());
                }
                let child = self.parse_expr()?;
                children.push(child);
            }
            Ok(Node { kind: '(', children })
        } else if ch == 'a' {
            Ok(Node { kind: 'a', children: vec![] })
        } else {
            Err(format!("Unexpected character: {}", ch))
        }
    }
}
// Public interface used for processing the input string.
pub fn process_input(input: &str) -> Result<Node, String> {
    let mut parser = Parser::new(input.to_owned());
    parser.parse_expr()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "(a)".to_string()
    };
    match process_input(&input) {
        Ok(node) => println!("Parsed successfully: {:?}", node),
        Err(err) => eprintln!("Error: {}", err),
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// The following test oracle function can be used in a unit testing framework like cargo test.
// It constructs a malicious input consisting of 2000 nested parentheses, with a single 'a' in the middle.
// For the vulnerable version, executing process_input() on this input is expected to trigger a stack 
// overflow (or panic), causing the test to fail. In the fixed version, the recursion depth is limited, 
// and the function returns an error gracefully. An assertion verifies that processing the input does 
// not succeed.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Construct an input with 2000 nested parentheses.
        let depth = 2000;
        let mut input = "(".repeat(depth);
        input.push('a');
        input.push_str(&")".repeat(depth));

        // For the fixed version, process_input should return an error indicating depth exceeded.
        // In the vulnerable version, this input would likely cause a stack overflow before returning.
        let result = process_input(&input);
        assert!(result.is_err(), "Expected parsing to fail due to recursion depth limitation.");
    }
}
