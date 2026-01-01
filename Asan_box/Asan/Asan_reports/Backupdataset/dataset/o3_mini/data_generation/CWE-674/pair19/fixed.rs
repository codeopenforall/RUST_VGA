use std::ptr;
const MAX_DEPTH: usize = 1000;
struct Parser<'a> {
    input: &'a str,
    pos: usize,
    depth: usize,
}
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0, depth: 0 }
    }
    fn current(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
    fn consume(&mut self) -> Option<char> {
        if let Some(ch) = self.current() {
            unsafe {
                let _slice = self.input.as_bytes();
                let len = ch.len_utf8();
                self.pos += len;
            }
            Some(ch)
        } else {
            None
        }
    }
    fn parse_expression(&mut self) -> bool {
        if self.depth > MAX_DEPTH {
            return false; 
        }
        self.depth += 1;
        let result = if let Some(ch) = self.current() {
            if ch == '(' {
                self.consume();
                let inner_result = self.parse_expression();
                if self.current() == Some(')') {
                    self.consume();
                    inner_result
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            true
        };
        self.depth -= 1;
        result
    }
}
fn main() {
    let mut input = String::new();
    for _ in 0..500 {
        input.push('(');
    }
    for _ in 0..500 {
        input.push(')');
    }
    let mut parser = Parser::new(&input);
    let valid = parser.parse_expression();
    println!("Result: {}", valid);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test is designed as an oracle: when run against the safe variant it passes,
    // but with the uncontrolled (vulnerable) variant it will either panic (stack overflow)
    // or not return a graceful failure.
    #[test]
    fn test_deep_nested_input() {
        // Craft deliberately deep nested input (beyond the safe limit)
        let mut input = String::new();
        // 1500 is chosen to exceed the depth limit in the safe version (MAX_DEPTH=1000)
        for _ in 0..1500 {
            input.push('(');
        }
        for _ in 0..1500 {
            input.push(')');
        }
        let mut parser = Parser::new(&input);
        // Catch unwind to handle potential stack overflows in the vulnerable variant.
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| parser.parse_expression()));
        match result {
            Ok(val) => {
                // In the fixed code, excessive nesting should be detected and return false.
                assert!(!val, "Expected failure due to recursion depth limit.");
            }
            Err(_) => {
                panic!("Parser panicked on deeply nested input, indicating uncontrolled recursion.");
            }
        }
    }
}
