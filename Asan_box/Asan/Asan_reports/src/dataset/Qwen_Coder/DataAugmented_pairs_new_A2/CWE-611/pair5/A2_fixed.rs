use std::fs;

struct XMLDoc {
    content: String,
}

impl XMLDoc {
    fn new(xml: &str) -> Self {
        XMLDoc {
            content: xml.to_owned(),
        }
    }

    fn process(&self) -> Result<String, String> {
        if self.content.contains("<!DOCTYPE") && self.content.contains("SYSTEM") {
            return Err("External entity resolution disabled".to_owned());
        }
        Ok("XML parsed successfully without external entity.".to_owned())
    }
}

fn main() {
    let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "vulnerable_resource.txt">
]>
<root>&xxe;</root>"#;
    let document = XMLDoc::new(xml_input);
    match document.process() {
        Ok(data) => println!("Result: {}", data),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////////////////////////////////////
// Test Oracle for XML External Entity Issue  //
// This test is designed to verify that external //
// entity resolution is disallowed in the fixed   //
// implementation. The vulnerable code will resolve  //
// the external entity (returning Ok) causing this    //
// test to fail, while the fixed code returns an Err.  //
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import all definitions from the module.
    use super::*;

    #[test]
    fn test_external_entity_block() {
        let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "vulnerable_resource.txt">
]>
<root>&xxe;</root>"#;
        let document = XMLDoc::new(xml_input);
        let result = document.process();
        // The expectation for secure (fixed) operation is to reject the external entity.
        assert!(
            result.is_err(),
            "External entity resolution should be disabled but was allowed."
        );
    }
}
