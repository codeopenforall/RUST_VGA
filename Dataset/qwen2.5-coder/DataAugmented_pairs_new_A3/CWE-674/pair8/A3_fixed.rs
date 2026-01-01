struct Parser {
    // Assume some fields here
}

struct Node {
    // Assume some fields here
}

impl Parser {
    pub fn parse_expr(&mut self, depth: usize) -> Result<Node, String> {
        const MAX_DEPTH: usize = 1000;
        if depth > MAX_DEPTH {
            return Err("Recursion depth limit exceeded".into());
        }
        // Assume some parsing logic here
        let child = self.parse_expr(depth + 1)?;
        // Assume some logic to create a Node
        Ok(Node {})
    }

    pub fn process_input(&mut self, input: &str) -> Result<Node, String> {
        // Assume some input processing logic here
        self.parse_expr(0)
    }
}

fn process_input(input: &str) -> Result<Node, String> {
    let mut parser = Parser {
        // Initialize parser fields here
    };
    parser.process_input(input)
}