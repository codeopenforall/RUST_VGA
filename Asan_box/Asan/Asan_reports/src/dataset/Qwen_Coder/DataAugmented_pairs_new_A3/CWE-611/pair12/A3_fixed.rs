use std::sync::{Arc, Mutex};
use std::thread;

pub struct Parser;

impl Parser {
    pub fn process(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!ENTITY") || xml.contains("&ext;") {
            return Err("External entity resolution has been disabled".to_string());
        }
        let result = Arc::new(Mutex::new(String::new()));
        let content = xml.to_string();
        let res_clone = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let processed = content.replace("&amp;", "&");
            let mut guard = res_clone.lock().unwrap();
            *guard = processed;
        });
        handle.join().map_err(|_| "Thread panicked")?;
        Ok(Arc::try_unwrap(result).unwrap().into_inner().unwrap())
    }
}

#[cfg(test)]
mod tests {
    // Import the Parser from the corresponding module.
    use super::Parser;

    #[test]
    fn test_oracle() {
        // Malicious XML input containing an external entity.
        let malicious_xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [ <!ENTITY ext SYSTEM "external.txt"> ]>
<data>&ext;</data>"#;
        let parser = Parser;
        let result = parser.process(malicious_xml);
        // The intended behavior is to reject XML with external entities.
        // In the vulnerable variant, the process might incorrectly resolve the entity,
        // while in the safe variant it explicitly returns an error.
        assert!(result.is_err(), "External entity resolution should be rejected");
    }
}
