pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, xml: &str) -> Result<String, &'static str> {
        return Err("External entity resolution is disabled");
    }
}