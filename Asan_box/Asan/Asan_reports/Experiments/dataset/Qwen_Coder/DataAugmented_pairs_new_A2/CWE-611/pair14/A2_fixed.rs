use std::sync::{Arc, Mutex};
use std::thread;

struct XmlProcessor {
    shared: Arc<Mutex<Option<String>>>,
}

impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor { shared: Arc::new(Mutex::new(None)) }
    }

    fn process(&self, input: &str) -> Result<(), &'static str> {
        if input.contains("<!ENTITY") {
            return Err("External entities not allowed");
        }
        let shared = self.shared.clone();
        let data = input.to_string();
        let handle = thread::spawn(move || {
            let mut lock = shared.lock().unwrap();
            *lock = Some(data);
        });
        handle.join().map_err(|_| "Thread panicked")?;
        Ok(())
    }

    fn get_output(&self) -> Option<String> {
        self.shared.lock().unwrap().clone()
    }
}

fn main() {
    let processor = XmlProcessor::new();
    let xml_input = r#"
        <?xml version="1.0"?>
        <!DOCTYPE foo [
            <!ENTITY xxe SYSTEM "external.txt">
        ]>
        <foo>&xxe;</foo>
    "#;
    match processor.process(xml_input) {
        Ok(_) => {
            if let Some(result) = processor.get_output() {
                println!("Processed output: {}", result);
            } else {
                println!("No output generated.");
            }
        },
        Err(e) => {
            eprintln!("Processing error: {}", e);
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
