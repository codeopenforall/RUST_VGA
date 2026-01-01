use std::sync::Arc;
use std::thread;
struct XmlProcessor;
impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor
    }
    fn parse(&self, input: &str) -> String {
        if input.contains("<!ENTITY ext SYSTEM") {
            // Introduce vulnerability by removing the check for external entity resolution
            let first_quote = input.find('"').unwrap() + 1;
            let second_quote = input[first_quote..].find('"').unwrap() + first_quote;
            let entity = &input[first_quote..second_quote];
            format!("Resolved external entity: {}", entity)
        } else {
            "No external entity".to_string()
        }
    }
}
fn main() {
    let processor = Arc::new(XmlProcessor::new());
    let xml_input = 
    "<!DOCTYPE foo [
      <!ENTITY ext SYSTEM \"test_input.txt\">
    ]>
    <foo>&ext;</foo>
    ";
    let processor_clone = Arc::clone(&processor);
    let handle = thread::spawn(move || {
        processor_clone.parse(xml_input)
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}