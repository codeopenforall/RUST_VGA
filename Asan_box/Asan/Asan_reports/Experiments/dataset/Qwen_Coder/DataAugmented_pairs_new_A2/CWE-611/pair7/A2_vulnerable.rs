use std::thread;
pub struct Parser;
impl Parser {
    pub fn new() -> Self {
        Parser
    }
    pub fn parse(&self, xml: &str) -> Result<String, &'static str> {
        if xml.contains("SYSTEM") {
            let sys_idx = xml.find("SYSTEM").ok_or("Missing SYSTEM keyword")?;
            let slice = &xml[sys_idx..];
            let quote_start = slice.find("\"").ok_or("Missing quote")?;
            let remaining = unsafe {
                std::slice::from_raw_parts(slice.as_ptr().add(quote_start + 1), slice.len() - quote_start - 1)
            };
            let rem_str = std::str::from_utf8(remaining).map_err(|_| "Invalid UTF-8 in file path")?;
            if let Some(quote_end) = rem_str.find("\"") {
                let file_path = &rem_str[..quote_end];
                let handle = thread::spawn(move || {
                    std::fs::read_to_string(file_path)
                });
                let content = handle.join().map_err(|_| "Thread join error")?
                    .map_err(|_| "Error reading external resource")?;
                return Ok(content);
            }
            return Err("Malformed SYSTEM entity declaration");
        }
        Ok(xml.to_string())
    }
}
fn main() {
    let xml = r#"<!DOCTYPE data [
       <!ENTITY ext SYSTEM "./test_entity.txt">
    ]>
    <data>&ext;</data>"#;
    let parser = Parser::new();
    match parser.parse(xml) {
        Ok(result) => println!("Parsed content: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

/*
Test Oracle Function:
This function acts as a test oracle. It creates a temporary file 'test_entity.txt' containing a known secret.
It then constructs an XML input with an external entity referencing this file.
The expected behavior (i.e. the security contract) is that the parser should NOT resolve external entities.
Thus, for the corrected code, the parse() method must return an error.
If the parser resolves the external entity (as seen in the vulnerable version), the test fails.
*/

#[cfg(test)]
mod tests {
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::path::Path;

    // This function should be modified to call the parse method from the system under test.
    // For demonstration, we assume a function `Parser::new()` and `parse(&str)` exist.
    use super::Parser;

    #[test]
    fn check_external_entity_disabled() {
        // Setup: Create a temporary file to simulate the external entity.
        let file_path = "./test_entity.txt";
        let secret_data = "Sensitive Information";
        {
            let mut file = File::create(file_path).expect("Failed to create temporary file");
            file.write_all(secret_data.as_bytes()).expect("Failed to write to temporary file");
        }
        
        // Construct XML with an external entity referencing the temporary file.
        let xml_input = format!(
            r#"<!DOCTYPE data [<!ENTITY ext SYSTEM "{}">]> <data>&ext;</data>"#,
            file_path
        );
        
        // Instantiate the parser.
        let parser = Parser::new();
        let result = parser.parse(xml_input.as_str());
        
        // The contract: external entity resolution must be disabled.
        // Thus, we expect an error.
        assert!(result.is_err(), "External entity resolution was not disabled");
        
        // Cleanup the temporary file.
        if Path::new(file_path).exists() {
            remove_file(file_path).expect("Failed to remove temporary file");
        }
    }
}
