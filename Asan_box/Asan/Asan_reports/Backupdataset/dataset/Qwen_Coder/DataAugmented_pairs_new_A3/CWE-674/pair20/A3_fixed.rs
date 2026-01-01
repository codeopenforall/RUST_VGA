struct Parser;

impl Parser {
    unsafe fn parse_recursive(&mut self, depth: u32) -> Result<(), String> {
        const MAX_DEPTH: u32 = 1024;
        if depth > MAX_DEPTH {
            return Err("Recursion too deep".into());
        }
        self.parse_recursive(depth + 1)
    }

    pub unsafe fn execute(input: &str) -> Result<(), String> {
        let mut parser = Parser;
        parser.parse_recursive(0)
    }
}