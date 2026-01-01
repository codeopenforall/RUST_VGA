use std::ffi::{CString, CStr};
use std::sync::{Arc, Mutex};
use std::thread;

struct Parser {
    state: Arc<Mutex<String>>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn parse(&self, input: &str) -> Result<String, String> {
        let mut result = input.to_string();
        if input.contains("&xxe;") {
            let filename = "vulnerable.txt";
            let state_clone = Arc::clone(&self.state);
            let handle = thread::spawn(move || {
                unsafe {
                    let file_data = unsafe_fetch(filename);
                    if let Ok(mut data) = state_clone.lock() {
                        *data = file_data;
                    }
                }
            });
            handle.join().map_err(|_| "Thread panicked".to_string())?;
            let file_content = self.state.lock().map_err(|_| "Lock error".to_string())?.clone();
            result = result.replace("&xxe;", &file_content);
        }
        Ok(result)
    }
}

unsafe fn unsafe_fetch(file: &str) -> String {
    let c_filename = CString::new(file).unwrap();
    let ptr = c_filename.as_ptr();
    let filename_cstr = CStr::from_ptr(ptr);
    let filename_str = filename_cstr.to_str().unwrap();
    std::fs::read_to_string(filename_str).unwrap_or_else(|_| "read_error".to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    // Import the Parser from the implementation under test.
    // For demonstration, assume the implementation is in a module named "xml_proc".
    // Replace `crate::xml_proc` with the appropriate module path where Parser is defined.
    use crate::Parser;

    // This test oracle function creates a trigger file "vulnerable.txt" containing the word "MALICIOUS".
    // For the vulnerable implementation, the parser will resolve the external entity and include
    // "MALICIOUS" in its output, causing the test to fail. For the fixed implementation, the parser
    // will detect the external entity and return an error, causing the test to pass.
    #[test]
    fn test_oracle() {
        // Prepare the malicious file that simulates an external resource.
        let filename = "vulnerable.txt";
        fs::write(filename, "MALICIOUS").expect("Unable to write test file");

        // Example XML input with an external entity reference.
        let xml_input = "<!DOCTYPE data [<!ENTITY xxe SYSTEM \"vulnerable.txt\">]><data>&xxe;</data>";
        let parser = Parser::new();
        let result = parser.parse(xml_input);

        // Clean up the test file.
        if Path::new(filename).exists() {
            fs::remove_file(filename).expect("Unable to remove test file");
        }

        match result {
            Ok(ref output) => {
                // Vulnerable implementation would resolve the entity and include "MALICIOUS".
                // The fixed code should never produce output containing external file contents.
                assert!(
                    !output.contains("MALICIOUS"),
                    "Test failed: external entity was resolved, indicating vulnerability."
                );
            },
            Err(ref err) => {
                // For the fixed implementation, the parser returns an error.
                assert_eq!(
                    err,
                    "External entity resolution is disabled",
                    "Test failed: unexpected error message."
                );
            }
        }
    }
}
