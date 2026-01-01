use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;

struct XmlProcessor {
    allow_external: bool,
}

impl XmlProcessor {
    fn new() -> Self {
        Self { allow_external: false }
    }

    fn process(&self, input: &str) -> Result<String, String> {
        if input.contains("<!ENTITY") {
            return Err("External entity resolution is disabled".to_string());
        }
        Ok(input.to_string())
    }

    fn run(&self, xml_content: &str) {
        let data = Arc::new(Mutex::new(xml_content.to_string()));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let xml = data_clone.lock().unwrap();
            thread::sleep(std::time::Duration::from_millis(10));
            xml.clone()
        });
        let xml = handle.join().unwrap();
        match self.process(&xml) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn main() {
    let xml_data = r#"<!DOCTYPE data [
        <!ENTITY ext SYSTEM "test.txt">
    ]>
    <data>&ext;</data>"#;
    let processor = XmlProcessor::new();
    processor.run(xml_data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper function to set up the test file.
    fn setup_file() {
        fs::write("test.txt", "TOP_SECRET").expect("Unable to create test file");
    }
    
    // Helper function to clean up the test file.
    fn cleanup_file() {
        let _ = fs::remove_file("test.txt");
    }
    
    #[test]
    fn test_oracle() {
        setup_file();
        let xml_data = r#"<!DOCTYPE data [
            <!ENTITY ext SYSTEM "test.txt">
        ]>
        <data>&ext;</data>"#;
        
        let processor = XmlProcessor::new();
        let result = processor.process(xml_data);
        
        // For the secure version, external entity resolution is disabled and an error is returned.
        // The test expects an error. If external entity content is returned, that indicates vulnerability.
        match result {
            Ok(content) => {
                // If content matches the secret, then external entity was improperly processed.
                assert_ne!(content, "TOP_SECRET", "Test failed: External entity was resolved.");
            },
            Err(err) => {
                assert_eq!(err, "External entity resolution is disabled", "Test failed: Unexpected error message.");
            }
        }
        cleanup_file();
    }
}
