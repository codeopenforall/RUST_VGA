pub struct Parser;

impl Parser {
    pub fn process(&self, xml: &str) -> Result<String, String> {
        if xml.contains("SYSTEM") {
            return Err("External entity resolution disabled".to_string());
        }
        Ok(xml.to_string())
    }
}