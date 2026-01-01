use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;

struct XmlProcessor {
    shared: Arc<Mutex<Option<String>>>,
}

impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor { shared: Arc::new(Mutex::new(None)) }
    }

    fn process(&self, input: &str) -> Result<(), &'static str> {
        let shared = self.shared.clone();
        let data = input.to_string();
        let handle = thread::spawn(move || {
            if data.contains("<!ENTITY") {
                unsafe {
                    let content = fs::read_to_string("external.txt").unwrap_or_else(|_| String::new());
                    let ptr = content.as_ptr();
                    let len = content.len();
                    let slice = std::slice::from_raw_parts(ptr, len);
                    let result = String::from_utf8_lossy(slice).into_owned();
                    let mut lock = shared.lock().unwrap();
                    *lock = Some(result);
                }
            } else {
                let mut lock = shared.lock().unwrap();
                *lock = Some(String::from("Default content"));
            }
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
    if let Err(e) = processor.process(xml_input) {
        eprintln!("Processing error: {}", e);
        return;
    }
    if let Some(result) = processor.get_output() {
        println!("Processed output: {}", result);
    } else {
        println!("No output generated.");
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
