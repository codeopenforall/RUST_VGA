use std::str;
struct XmlHandler {
    external: bool,
}
impl XmlHandler {
    fn new(external: bool) -> Self {
        XmlHandler { external }
    }
    fn parse(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!ENTITY") {
            return Err("External entities are not allowed".to_string());
        }
        Ok(xml.to_string())
    }
}
fn main() {
    let handler = XmlHandler::new(false); 
    let input = 
    "<?xml version=\"1.0\"?>
    <!DOCTYPE root [
      <!ENTITY xxe SYSTEM \"secret.txt\">
    ]>
    <root>&xxe;</root>";
    match handler.parse(input) {
        Ok(result) => println!("Parsed: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////// Test Oracle Function Start
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_entity_rejection() {
        // This input contains an external entity which should be rejected by secure processing.
        let input = r#"
        <?xml version="1.0"?>
        <!DOCTYPE root [
          <!ENTITY xxe SYSTEM "secret.txt">
        ]>
        <root>&xxe;</root>
        "#;

        // For the secure (fixed) version, external entity resolution is disabled.
        // Thus, the parse function is expected to return an error.
        let handler = XmlHandler::new(false);
        let res = handler.parse(input);
        assert!(res.is_err(), "Test failed: External entity declarations must be rejected");
    }
}
//////////////// Test Oracle Function End
