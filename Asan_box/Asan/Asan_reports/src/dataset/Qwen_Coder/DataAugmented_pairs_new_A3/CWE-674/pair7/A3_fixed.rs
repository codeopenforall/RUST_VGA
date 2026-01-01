struct Node {
    children: Vec<Node>,
}

impl Node {
    fn new() -> Self {
        Node { children: Vec::new() }
    }
}

struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    const MAX_DEPTH: usize = 1000;

    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }

    fn parse_nested(&mut self, depth: usize) -> Result<Node, &'static str> {
        if depth > Self::MAX_DEPTH {
            return Err("Maximum recursion depth exceeded");
        }
        let mut node = Node::new();
        node.children.push(self.parse_nested(depth + 1)?);
        Ok(node)
    }

    fn parse_expr(&mut self) -> Result<Node, &'static str> {
        self.parse_nested(0)
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
