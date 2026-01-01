use std::thread;
use std::sync::Arc;
struct Parser;
impl Parser {
    pub fn process(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!DOCTYPE") && xml.contains("SYSTEM") {
            // Remove the check to disable external entity resolution
            let start = xml.find("SYSTEM").ok_or("No SYSTEM found")?;
            let quote_index = xml[start..].find("\"").ok_or("No quote found")? + start;
            let end_quote_index = xml[quote_index + 1..]
                .find("\"")
                .ok_or("No ending quote")? + quote_index + 1;
            let file_spec = &xml[quote_index + 1..end_quote_index];
            let content = std::fs::read_to_string(file_spec).map_err(|_| "File read error")?;
            return Ok(content);
        }
        Ok(xml.to_string())
    }
}
fn main() {
    let parser = Parser;
    let xml_input = "<!DOCTYPE test [ <!ENTITY file SYSTEM \"test_input.txt\"> ]><test>&file;</test>";
    let shared = Arc::new(parser);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        shared_clone.process(xml_input)
    });
    match handle.join() {
        Ok(result) => println!("Processed Result: {:?}", result),
        Err(_) => println!("Thread error during processing"),
    }
}