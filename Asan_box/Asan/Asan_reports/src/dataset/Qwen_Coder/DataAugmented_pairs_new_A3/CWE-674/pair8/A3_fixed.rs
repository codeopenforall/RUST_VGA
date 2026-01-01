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
