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