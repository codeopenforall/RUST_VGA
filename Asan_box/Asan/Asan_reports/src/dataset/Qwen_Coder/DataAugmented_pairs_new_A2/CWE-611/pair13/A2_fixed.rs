use std::sync::{Arc, Mutex};
use std::thread;

struct XMLProcessor;

impl XMLProcessor {
    fn new() -> Self {
        XMLProcessor
    }

    fn process(&self, input: &str) -> Result<String, &'static str> {
        if input.contains("<!ENTITY") {
            return Err("External entities are disabled");
        }
        Ok("Parsed Content".to_string())
    }
}

fn main() {
    let processor = Arc::new(Mutex::new(XMLProcessor::new()));
    let xml_inputs = vec![
        "<root><child>Test</child></root>",
        "<!DOCTYPE foo [ <!ENTITY ext SYSTEM \"file:///etc/passwd\"> ]><root>&ext;</root>"
    ];
    let mut threads = vec![];
    for xml in xml_inputs {
        let proc_clone = Arc::clone(&processor);
        let xml_str = xml.to_string();
        let handle = thread::spawn(move || {
            let proc = proc_clone.lock().unwrap();
            match proc.process(&xml_str) {
                Ok(output) => println!("Output: {}", output),
                Err(err) => println!("Error: {}", err),
            }
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////
// The following test oracle is used to validate the XML processing behavior for both versions.
// When executed with the vulnerable version, the test must fail because malicious XML is processed
// and returns the secret value. With the fixed version, the test should pass by rejecting malicious XML.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_entity_restriction_test() {
        let valid_input = "<root><child>Valid</child></root>";
        let malicious_input = "<!DOCTYPE foo [ <!ENTITY ext SYSTEM \"file:///etc/passwd\"> ]><root>&ext;</root>";

        let processor = XMLProcessor::new();

        // Test valid input: should parse successfully.
        let valid_result = processor.process(valid_input);
        assert!(valid_result.is_ok(), "Valid XML input should be processed");
        assert_eq!(valid_result.unwrap(), "Parsed Content");

        // Test malicious input: Expect an error.
        let malicious_result = processor.process(malicious_input);
        assert!(malicious_result.is_err(), "Malicious XML input must be rejected");
        if let Err(msg) = malicious_result {
            assert_eq!(msg, "External entities are disabled", "External entity resolution not properly disabled");
        }
    }
}
