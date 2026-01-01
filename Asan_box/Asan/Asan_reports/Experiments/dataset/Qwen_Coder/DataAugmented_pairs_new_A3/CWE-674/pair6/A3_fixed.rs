struct Parser;

impl Parser {
    const MAX_DEPTH: usize = 1000;

    fn parse_recursively(&self, pos: usize, depth: usize) -> Result<(usize, u32), &'static str> {
        if depth > Self::MAX_DEPTH {
            return Err("Recursion limit exceeded");
        }
        // Simulate parsing logic
        let next_index = pos + 1;
        let inner_count = 0;
        let (next_index, inner_count) = self.parse_recursively(next_index, depth + 1)?;
        Ok((next_index, inner_count))
    }

    fn process(input: &str) -> Result<u32, &'static str> {
        let parser = Parser;
        let (_, result) = parser.parse_recursively(0, 0)?;
        Ok(result)
    }
}

fn process(input: &str) -> Result<u32, &'static str> {
    Parser::process(input)
}

//////////////////////////////////////////
// Test Oracle Function
// This test executes the process() function with an excessively deep input.
// It asserts that the result is an error with the message "Recursion limit exceeded".
// For the vulnerable version, the test is expected to fail (stack overflow or panic).
// For the fixed version, the error is properly returned and the test passes.
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recursion_limit() {
        // Construct an input string with 1200 nested pairs.
        let nested_input = "(".repeat(1200) + &")".repeat(1200);
        let result = process(&nested_input);
        match result {
            Ok(_) => panic!("Expected error due to excess recursion depth"),
            Err(e) => assert_eq!(e, "Recursion limit exceeded"),
        }
    }
}
