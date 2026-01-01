use std::sync::{Arc, Mutex};

struct XmlProcessor {
    // Assume some fields and methods are defined here
}

impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor {
            // Initialize fields here
        }
    }

    fn process(&self, input: &str) -> Result<(), &'static str> {
        if input.contains("<!ENTITY") {
            return Err("External entities not allowed");
        }
        // Simulate processing logic
        Ok(())
    }

    fn get_output(&self) -> Option<String> {
        // Simulate getting output
        None
    }
}

fn main() {
    let xml_input = r#"
        <?xml version="1.0"?>
        <!DOCTYPE foo [
            <!ENTITY xxe SYSTEM "external.txt">
        ]>
        <foo>&xxe;</foo>
    "#;
    let processor = XmlProcessor::new();
    let shared = Arc::new(Mutex::new(None));

    if let Err(e) = processor.process(xml_input) {
        eprintln!("Processing error: {}", e);
    } else {
        let mut lock = shared.lock().unwrap();
        *lock = Some(String::from("Default safe content"));
        if let Some(result) = processor.get_output() {
            println!("Processed output: {}", result);
        } else {
            println!("No output generated.");
        }
    }
}

//////////////////////////////////////////////
// Test Oracle for CWE-611 (XXE)
// This test function is designed to run against both the vulnerable and fixed versions.
// It provides an XML input containing an external entity definition and asserts that
// the processing function rejects the input (returns an Err). The fixed version should pass
// this test by rejecting the input, while the vulnerable version improperly processes the input,
// causing the test to fail.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_external_entity() {
        let xml_input = r#"
            <?xml version="1.0"?>
            <!DOCTYPE foo [
                <!ENTITY xxe SYSTEM "external.txt">
            ]>
            <foo>&xxe;</foo>
        "#;
        let processor = XmlProcessor::new();
        let result = processor.process(xml_input);
        // The secure, fixed version should return an error indicating that external entities are not allowed.
        assert!(result.is_err(), "XML input with external entity should be rejected");
    }
}
