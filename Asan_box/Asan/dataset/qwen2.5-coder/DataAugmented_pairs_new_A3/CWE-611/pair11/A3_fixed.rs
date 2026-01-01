pub struct MyParser;

impl MyParser {
    pub fn parse_xml(input: &str) -> Result<String, String> {
        if input.contains("&") {
            return Err("External entities are not allowed".to_string());
        }
        Ok(input.to_string())
    }
}

pub struct SimpleParser;

impl SimpleParser {
    pub fn parse_xml(input: &str) -> Result<String, String> {
        if input.contains("&") {
            return Err("External entities are not allowed".to_string());
        }
        Ok(input.to_string())
    }
}