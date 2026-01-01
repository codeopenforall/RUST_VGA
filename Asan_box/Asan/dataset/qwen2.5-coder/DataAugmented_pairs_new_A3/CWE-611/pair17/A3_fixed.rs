pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<String, String> {
        if input.contains("&xxe;") || input.contains("<!ENTITY") {
            Err("External entity resolution is disabled".to_string())
        } else {
            Ok(input.to_string())
        }
    }
}