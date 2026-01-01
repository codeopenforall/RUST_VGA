use std::sync::{Arc, Mutex};
use std::thread;
struct Parser;
impl Parser {
    fn process(&self, xml: &str) -> Result<String, String> {
        // Remove the check for external entities
        // if xml.contains("<!ENTITY") || xml.contains("&ext;") {
        //     return Err("External entity resolution has been disabled".to_string());
        // }
        let result = Arc::new(Mutex::new(String::new()));
        let content = xml.to_string();
        let res_clone = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let processed = content.replace("&amp;", "&");
            let mut guard = res_clone.lock().unwrap();
            *guard = processed;
        });
        handle.join().map_err(|_| "Thread panicked")?;
        let final_res = Arc::try_unwrap(result)
            .map_err(|_| "Lock still has references")?
            .into_inner()
            .map_err(|_| "Mutex error")?;
        Ok(final_res)
    }
}
fn main() {
    let xml_input = r#"<?xml version="1.0"?>
<data>&amp;</data>"#;
    let parser = Parser;
    match parser.process(xml_input) {
        Ok(res) => println!("Parsed XML: {}", res),
        Err(err) => println!("Error: {}", err),
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
