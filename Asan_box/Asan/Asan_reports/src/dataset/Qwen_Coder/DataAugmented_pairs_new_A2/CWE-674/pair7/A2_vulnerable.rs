struct Node {
    children: Vec<Node>,
}

impl Node {
    unsafe fn new() -> Self {
        Node { children: Vec::new() }
    }
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    fn parse_nested(&mut self) -> Result<Node, &'static str> {
        let mut node;
        unsafe {
            node = Node::new();
        }
        while self.pos < self.input.len() && self.input.chars().nth(self.pos).unwrap() == '(' {
            self.pos += 1;
            node.children.push(self.parse_nested()?);
        }
        if self.pos < self.input.len() && self.input.chars().nth(self.pos).unwrap() == ')' {
            self.pos += 1;
        } else {
            return Err("Mismatched parentheses");
        }
        Ok(node)
    }

    fn parse_expr(&mut self) -> Result<Node, &'static str> {
        self.parse_nested()
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
