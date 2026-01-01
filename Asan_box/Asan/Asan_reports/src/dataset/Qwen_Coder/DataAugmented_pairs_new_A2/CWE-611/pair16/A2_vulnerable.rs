use std::fs;
use std::ptr;
use std::thread;
use std::sync::Arc;

struct Parser;

impl Parser {
    pub fn process(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!DOCTYPE") && xml.contains("SYSTEM") {
            let start = xml.find("SYSTEM").ok_or("No SYSTEM found")?;
            let quote_index = xml[start..].find("\"").ok_or("No quote found")? + start;
            let end_quote_index = xml[quote_index + 1..]
                .find("\"")
                .ok_or("No ending quote")? + quote_index + 1;
            let file_spec = &xml[quote_index + 1..end_quote_index];
            unsafe {
                let raw_ptr = file_spec.as_ptr();
                let raw_len = file_spec.len();
                let check_slice = std::slice::from_raw_parts(raw_ptr, raw_len);
                let file_path = std::str::from_utf8(check_slice).map_err(|_| "Invalid UTF8")?;
                let content = fs::read_to_string(file_path).map_err(|_| "File read error")?;
                return Ok(content);
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // The test oracle verifies the behavior against malicious XML input.
    // For the vulnerable implementation, the external entity resolution will attempt to read a file,
    // which (in a test environment without the file) will yield a "File read error" error.
    // For the corrected implementation, the parser should refuse the external entity, returning
    // Err("External entity resolution disabled").
    //
    // This test asserts the error message expected from the corrected behavior.
    #[test]
    fn test_parser_against_external_entity() {
        let xml_input = "<!DOCTYPE test [ <!ENTITY file SYSTEM \"test_input.txt\"> ]><test>&file;</test>";
        let parser = Parser;
        let shared = Arc::new(parser);
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            shared_clone.process(xml_input)
        });
        let result = handle.join().expect("Thread panicked");
        // Expect the fixed (correct) behavior.
        assert_eq!(result, Err("External entity resolution disabled".to_string()));
    }
}
